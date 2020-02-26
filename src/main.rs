use rand::Rng;
use std::time::Duration;
use std::thread::sleep;

type World = Vec<Vec<bool>>;

struct Coord {
    x: usize,
    y: usize
}

fn main() {
    let sleep_time = Duration::from_millis(500);

    let mut world = random_world();

    loop {
        display(&world);
        world = progress(&world);
        sleep(sleep_time);
    }
}

fn random_world() -> World {
    let mut r = rand::thread_rng();
    let mut world = Vec::new();
    for _i in 1..20 {
        let mut row = Vec::new();
        for _j in 1..20 {
            row.push(r.gen::<f64>() > 0.5);
        }
        world.push(row);
    }
    world
}

fn display(world: &World) {
    for row in world.iter() {
        print!("#");
        for tile in row.iter() {
            print!("{}", if *tile {'X'} else {' '});
        }
        println!("#");
    }
    println!("{}", "#".repeat(world.len() + 2));
}

fn progress(world: &World) -> World {
    let mut result = Vec::new();
    for i in 0..world.len() {
        let mut row = Vec::new();
        for j in 0..world[0].len() {
            row.push(alive(world, Coord{x:i, y:j}));
        }
        result.push(row);
    }
    result
}

fn alive(world: &World, coord: Coord) -> bool {
    let living = world[coord.x][coord.y];
    let alive_neighbours = surrounding_alive(world, coord);
    return if living {
        alive_neighbours == 2 || alive_neighbours == 3
    } else {
        alive_neighbours == 3
    }
}

fn surrounding_alive(world: &World, coord: Coord) -> usize {
    let height = world.len() as i32;
    let width = world[0].len() as i32;
    let mut result = 0;
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    for (dx, dy) in directions  {
        let mut x = (coord.x as i32 + dx) % height;
        let mut y = (coord.y as i32 + dy) % width;
        if x < 0 {
            x += height;
        }
        if y < 0 {
            y += width;
        }
        if world[x as usize][y as usize] {
            result += 1;
        }
    }
    result
}