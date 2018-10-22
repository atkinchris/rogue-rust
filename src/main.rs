#[macro_use]
extern crate specs_derive;
extern crate specs;

use specs::{Builder, DispatcherBuilder, ReadStorage, System, VecStorage, World, WriteStorage};

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Movement>, WriteStorage<'a, Position>);

    fn run(&mut self, (mov, mut pos): Self::SystemData) {
        use specs::Join;
        for (mov, pos) in (&mov, &mut pos).join() {
            pos.x += mov.x;
            pos.y += mov.y;
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Movement {
    x: u32,
    y: u32,
}

fn main() {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &["hello_world"])
        .with(HelloWorld, "hello_updated", &["update_pos"])
        .build();

    dispatcher.setup(&mut world.res);
    world.create_entity().with(Position { x: 4, y: 7 }).build();
    world
        .create_entity()
        .with(Position { x: 2, y: 5 })
        .with(Movement { x: 1, y: 0 })
        .build();

    dispatcher.dispatch(&mut world.res);
}
