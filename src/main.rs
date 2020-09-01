use bevy::prelude::*;

struct Player {
  velocity: Vec3,
}

//struct Obstacle;

fn main() {
  App::build()
    .add_default_plugins()
    .add_startup_system(setup.system())
    .add_system(player_input_system.system())
    .run();
}

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<ColorMaterial>>
) {
  let texture_handle = asset_server.load("assets/player/bunny1_walk1.png").unwrap();
  commands
    .spawn(Camera2dComponents::default())
    .spawn(SpriteComponents {
      material: materials.add(texture_handle.into()),
      ..Default::default()
    })
    .with(Player { velocity: Vec3::zero() });
}

fn player_input_system(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut player_query: Query<(&mut Player, &mut Translation)>) {
  for (mut player, mut translation) in &mut player_query.iter() {
    let mut vertical_velocity = -9.8;
    if keyboard_input.pressed(KeyCode::Space) {
      vertical_velocity = 16.0;
    }
    player.velocity += Vec3::new(0.0, vertical_velocity, 0.0);
    translation.0 += player.velocity * time.delta_seconds;
  }
}

