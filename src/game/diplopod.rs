use bevy::{
    ecs::{component::HookContext, system::SystemState, world::DeferredWorld},
    prelude::*,
};

use crate::{
    MAX_X, MAX_Y,
    game::{CommandResources, DiplopodColors},
};

use super::{ARENA_HEIGHT, ARENA_WIDTH, GameOver, OnGameScreen, TILE_SIZE, UPPER_LEFT};

pub const START_POSITION: Transform = Transform::from_xyz(
    (ARENA_WIDTH / 2) as f32 * TILE_SIZE + UPPER_LEFT.x - MAX_X / 2.,
    (ARENA_HEIGHT / 2) as f32 * TILE_SIZE + UPPER_LEFT.y - MAX_Y / 2.,
    0.0,
);

#[derive(Default, Resource)]
pub struct DiplopodSegments(pub Vec<Entity>);

#[derive(Component)]
pub struct DiplopodHead {
    pub direction: Vec2,
    pub immunity: Timer,
}

impl Default for DiplopodHead {
    fn default() -> Self {
        Self {
            direction: Vec2::ZERO,
            immunity: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

#[derive(Component)]
#[component(on_add=on_add_diplopod_segment)]
pub struct DiplopodSegment;

fn on_add_diplopod_segment(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    world.resource_mut::<DiplopodSegments>().0.push(entity);
}

pub struct SpawnDiplopodSegment;

impl Command for SpawnDiplopodSegment {
    fn apply(self, world: &mut World) {
        let segments = &world.resource::<DiplopodSegments>().0;
        let is_head = segments.is_empty();

        let position = if is_head {
            START_POSITION
        } else {
            *world.get::<Transform>(*segments.last().unwrap()).unwrap()
        };

        let immune = if is_head {
            false
        } else {
            !world
                .get::<DiplopodHead>(*segments.first().unwrap())
                .unwrap()
                .immunity
                .finished()
        };

        let mut command_resources: CommandResources = SystemState::new(world);
        let (mut commands, mut meshes, colors) = command_resources.get_mut(world);

        let color = if immune {
            colors.diplopod_immune.clone()
        } else {
            colors.diplopod_normal.clone()
        };

        let mut segment = commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
            color,
            position,
            DiplopodSegment,
            OnGameScreen,
        ));

        if is_head {
            segment.insert(DiplopodHead::default());
        }

        command_resources.apply(world);
    }
}

pub fn keyboard(keyboard_input: Res<ButtonInput<KeyCode>>, mut heads: Query<&mut DiplopodHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA, KeyCode::KeyH]) {
            direction = Vec2::new(-1.0, 0.0);
        }

        if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD, KeyCode::KeyL]) {
            direction = Vec2::new(1.0, 0.0);
        }

        if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW, KeyCode::KeyK]) {
            direction = Vec2::new(direction.x, 1.0);
        }

        if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS, KeyCode::KeyJ]) {
            direction = Vec2::new(direction.x, -1.0);
        }

        if direction != Vec2::ZERO {
            head.direction = direction;
        }
    }
}

pub fn gamepad(gamepads: Query<&Gamepad>, mut heads: Query<&mut DiplopodHead>) {
    const TILT: f32 = 0.9;

    if let Some(mut head) = heads.iter_mut().next() {
        let mut direction = Vec2::ZERO;

        for gamepad in gamepads.iter() {
            if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX) {
                if left_stick_x <= -TILT {
                    direction = Vec2::new(-1.0, 0.0);
                }

                if left_stick_x >= TILT {
                    direction = Vec2::new(1.0, 0.0);
                }
            }

            if let Some(left_stick_y) = gamepad.get(GamepadAxis::LeftStickY) {
                if left_stick_y <= -TILT {
                    direction = Vec2::new(direction.x, -1.0);
                }

                if left_stick_y >= TILT {
                    direction = Vec2::new(direction.x, 1.0);
                }
            }
        }

        if direction != Vec2::ZERO {
            head.direction = direction;
        }
    }
}

pub fn movement(
    mut heads: Query<(Entity, &DiplopodHead)>,
    mut positions: Query<&mut Transform>,
    segments: ResMut<DiplopodSegments>,
    mut game_over_writer: EventWriter<GameOver>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| positions.get_mut(*e).unwrap().translation)
            .collect::<Vec<Vec3>>();

        let mut head_pos = positions.get_mut(head_entity).unwrap();
        head_pos.translation.x += head.direction.x * TILE_SIZE;
        head_pos.translation.y += head.direction.y * TILE_SIZE;

        if segment_positions.contains(&head_pos.translation)
            && (head.direction.x != 0.0 || head.direction.y != 0.0)
        {
            game_over_writer.write(GameOver);
        }

        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                positions.get_mut(*segment).unwrap().translation = *pos;
            });
    }
}

pub fn limit_immunity(mut heads: Query<&mut DiplopodHead>, time: Res<Time>) {
    if let Ok(mut head) = heads.single_mut()
        && !head.immunity.finished()
    {
        head.immunity.tick(time.delta());
    }
}

pub fn change_color_during_immunity(
    mut query: Query<&mut MeshMaterial2d<ColorMaterial>, With<DiplopodSegment>>,
    heads: Query<&DiplopodHead>,
    diplopod_colors: Res<DiplopodColors>,
) {
    if let Ok(head) = heads.single() {
        let target_color = if head.immunity.remaining_secs() > 2.0 {
            diplopod_colors.diplopod_immune.clone()
        } else if !head.immunity.finished() {
            if let Some(current_color) = query.iter().next() {
                if current_color.0 == diplopod_colors.diplopod_normal.0 {
                    diplopod_colors.diplopod_immune.clone()
                } else {
                    diplopod_colors.diplopod_normal.clone()
                }
            } else {
                diplopod_colors.diplopod_normal.clone()
            }
        } else {
            diplopod_colors.diplopod_normal.clone()
        };

        if let Some(current_color) = query.iter().next()
            && current_color.0 != target_color.0
        {
            for mut segment_color in query.iter_mut() {
                *segment_color = target_color.clone();
            }
        }
    }
}
