extern crate rand;

use cell::Cell;

pub struct World {
    pub width:  i16,
    pub height: i16,
    pub cells:  Vec<Vec<Cell>>
}

pub fn create_world(width: i16, height: i16) -> World {
    let rows = (0..height).map(|_| {
        (0..width).map(|_| {
            match rand::random::<i8>() {
                0 ... 48 => Cell::Alive,
                _        => Cell::Dead
            }
        }).collect()
    }).collect();

    World { width: width, height: height, cells: rows }
}

pub fn next_world(world: &World) -> World {
    let rows = (0..world.height).map(|y| {
        (0..world.width)
            .map(|x| next_cell_state(&world, x, y) )
            .collect()
    }).collect();

    World { width: world.width, height: world.height, cells: rows }
}

fn next_cell_state(world: &World, x: i16, y: i16) -> Cell {
    let ref cell      = world.cells[y as usize][x as usize];
    let num_neighbors = count_neighbors(&world, x, y);

    match (cell, num_neighbors) {
        (&Cell::Alive, 2) => Cell::Alive,
        (_, 3)            => Cell::Alive,
        _                 => Cell::Dead
    }
}

fn count_neighbors(world: &World, x: i16, y: i16) -> i16 {
    let neighbors = vec![
        (x-1, y-1),
        (x,   y-1),
        (x+1, y-1),
        (x-1, y),
        (x+1, y),
        (x-1, y+1),
        (x,   y+1),
        (x+1, y+1)
    ];

    neighbors.iter().fold(0, |mut acc, &(x, y)| {
        if x >= 0 && x < world.width && y >= 0 && y < world.height {
            let ref cell = world.cells[y as usize][x as usize];

            acc += match cell {
                &Cell::Alive => 1,
                &Cell::Dead  => 0
            }
        }

        acc
    })
}
