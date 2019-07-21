use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, ImageFormat, Sprite, SpriteRender, SpriteSheet, Texture};

use crate::components::{Energy, IsPlayer};

pub struct Gameplay;

pub const TILE_SIZE: u32 = 16;
pub const MARGIN_SIZE: u32 = 1;
pub const CAMERA_WIDTH: f32 = (50 * TILE_SIZE) as f32;
pub const CAMERA_HEIGHT: f32 = (40 * TILE_SIZE) as f32;

fn initialise_camera(world: &mut World) {
  let mut transform = Transform::default();
  transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 1.0);

  world
    .create_entity()
    .with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
    .with(transform)
    .build();
}

fn initialise_entities(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
  let mut transform = Transform::default();
  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet_handle.clone(),
    sprite_number: 0,
  };

  transform.set_translation_xyz(CAMERA_WIDTH * 0.5, CAMERA_HEIGHT * 0.5, 0.0);

  world
    .create_entity()
    .with(transform)
    .with(sprite_render)
    .with(IsPlayer)
    .with(Energy::new())
    .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load("sprites.png", ImageFormat::default(), (), &texture_storage)
  };

  let sprite_count: u32 = 32;
  let mut sprites = Vec::with_capacity((sprite_count * sprite_count) as usize);
  let offsets = [TILE_SIZE as f32 / 2.0; 2];

  for y in 0..sprite_count {
    for x in 0..sprite_count {
      let sprite = Sprite::from_pixel_values(
        sprite_count * (TILE_SIZE + MARGIN_SIZE) - MARGIN_SIZE,
        sprite_count * (TILE_SIZE + MARGIN_SIZE) - MARGIN_SIZE,
        TILE_SIZE,
        TILE_SIZE,
        x * (TILE_SIZE + MARGIN_SIZE),
        y * (TILE_SIZE + MARGIN_SIZE),
        offsets,
        false,
        false,
      );
      sprites.push(sprite);
    }
  }

  let sprite_sheet = SpriteSheet {
    texture: texture_handle,
    sprites,
  };

  let loader = world.read_resource::<Loader>();
  loader.load_from_data(
    sprite_sheet,
    (),
    &world.read_resource::<AssetStorage<SpriteSheet>>(),
  )
}

impl SimpleState for Gameplay {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    let sprite_sheet = load_sprite_sheet(world);

    initialise_entities(world, sprite_sheet);
    initialise_camera(world);
  }
}
