extern crate rand;

use cell::Cell;

pub struct World {
    pub width:  i16,
    pub height: i16,
    pub cells:  Vec<Vec<Cell>>
}

pub fn create_world(width: i16, height: i16) -> World {
    let mut rows = Vec::new();

    for _ in 0..height {
        let mut row = Vec::new();

        for _ in 0..width {
            row.push(
                match rand::random::<i8>() {
                    0 ... 48 => Cell::Alive,
                    _        => Cell::Dead
                }
            );
        }

        rows.push(row);
    }

    World { width: width, height: height, cells: rows }
}

pub fn next_world(world: &World) -> World {
    let mut rows = Vec::new();

    for y in 0..world.height {
        let mut row = Vec::new();

        for x in 0..world.width {
            let next: Cell = next_cell_state(&world, x, y);
            row.push(next);
        }

        rows.push(row);
    }

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
    let mut count = 0;

    let positions = vec![
        (x-1, y-1),
        (x,   y-1),
        (x+1, y-1),
        (x-1, y),
        (x+1, y),
        (x-1, y+1),
        (x,   y+1),
        (x+1, y+1)
    ];

    for (x, y) in positions {
        if x >= 0 && x < world.width && y >= 0 && y < world.height {
            let ref cell = world.cells[y as usize][x as usize];

            count += match cell {
                &Cell::Alive => 1,
                &Cell::Dead  => 0
            }
        }
    }

    count
}
