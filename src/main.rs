extern crate kiss3d;
extern crate nalgebra as na;
extern crate time;

use na::Vector3;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;

mod world;
mod cell;


static SIZE:  f32 = 0.025;
static DEPTH: f32 = 0.0025;
static GAP:   f32 = 0.0275;


fn build_tiles(window: &mut Window, tiles: &mut Vec<SceneNode>, world: &world::World) {
    for y in 0..world.height {
        for x in 0..world.width {
            let ref cell = world.cells[y as usize][x as usize];

            match cell {
                &cell::Cell::Alive => {
                    let mut tile = window.add_cube(SIZE, SIZE, DEPTH);

                    tile.set_color(1.0, 0.0, 0.0);
                    tile.append_translation(&Vector3::new(
                        (x as f32) * GAP,
                        (y as f32) * GAP,
                        0.0
                    ));

                    tiles.push(tile);
                },
                _ => {}
            }
        }
    }
}

fn destroy_tiles(window: &mut Window, tiles: &mut Vec<SceneNode>) {
    for mut tile in tiles.iter_mut() {
        window.remove(&mut tile);
    }

    tiles.clear();
}

fn main() {
    let mut world = world::create_world(64, 36);
    let mut window = Window::new("Game of Life: 2d");

    window.set_light(Light::StickToCamera);

    let mut active_tiles = vec![];
    let mut last_time = time::precise_time_ns();

    build_tiles(&mut window, &mut active_tiles, &world);

    while window.render() {
        let current_time = time::precise_time_ns();

        if (current_time - last_time) > 100000000 {
            last_time = current_time;
            world     = world::next_world(&world);

            destroy_tiles(&mut window, &mut active_tiles);
            build_tiles(&mut window, &mut active_tiles, &world);
        }
    }
}
