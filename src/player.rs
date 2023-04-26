use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::{plugin::InputManagerSystem, prelude::*};

#[derive(Component)]
struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Movement {
    Left,
    Right,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Slot {
    Primary,
    Secondary,
    Ability1,
    Ability2,
    Ability3,
    Ability4,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Ability {
    Evade,
    Bomb,
    Shoot,
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
struct AbilitySlotMap {
    map: HashMap<Slot, Ability>,
}

#[derive(Resource)]
struct PlayerResource {
    movement_speed: f32,
    health: f32,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    #[bundle]
    sprite: SpriteBundle,
    movement_input_map: InputMap<Movement>,
    movement_action_state: ActionState<Movement>,
    slot_input_map: InputMap<Slot>,
    slot_action_state: ActionState<Slot>,
    ability_action_state: ActionState<Ability>,
    ability_slot_map: AbilitySlotMap,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Movement>::default())
            .add_plugin(InputManagerPlugin::<Slot>::default())
            .add_plugin(InputManagerPlugin::<Ability>::default())
            .add_system(
                Self::copy_action_state
                    .in_base_set(CoreSet::PreUpdate)
                    .after(InputManagerSystem::ManualControl),
            )
            .insert_resource(PlayerResource {
                movement_speed: 250.,
                health: 100.,
            })
            .add_startup_system(Self::spawn_player)
            .add_system(Self::report_abilities_used)
            .add_system(Self::handle_movement)
            .add_system(Self::wrap_player_around_window);
    }
}

impl PlayerPlugin {
    fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
        let player_sprite = SpriteBundle {
            texture: asset_server.load("Ships/ship_0004.png"),
            ..default()
        };

        let mut ability_slot_map = AbilitySlotMap::default();
        ability_slot_map.insert(Slot::Primary, Ability::Shoot);
        ability_slot_map.insert(Slot::Secondary, Ability::Bomb);
        ability_slot_map.insert(Slot::Ability1, Ability::Evade);

        let player_bundle = PlayerBundle {
            player: Player,
            sprite: player_sprite,
            movement_input_map: InputMap::new([
                (KeyCode::A, Movement::Left),
                (KeyCode::D, Movement::Right),
                (KeyCode::Left, Movement::Left),
                (KeyCode::Right, Movement::Right),
            ]),
            movement_action_state: ActionState::default(),
            slot_input_map: InputMap::new([
                (KeyCode::Space, Slot::Ability1),
                (KeyCode::Q, Slot::Ability2),
                (KeyCode::W, Slot::Ability3),
                (KeyCode::E, Slot::Ability4),
            ])
            .insert(MouseButton::Left, Slot::Primary)
            .insert(MouseButton::Right, Slot::Secondary)
            .insert(KeyCode::Z, Slot::Primary)
            .insert(KeyCode::X, Slot::Secondary)
            .build(),
            slot_action_state: ActionState::default(),
            ability_action_state: ActionState::default(),
            ability_slot_map,
        };

        commands.spawn(player_bundle);
    }

    fn copy_action_state(
        mut query: Query<(
            &ActionState<Slot>,
            &mut ActionState<Ability>,
            &AbilitySlotMap,
        )>,
    ) {
        for (slot_state, mut ability_state, ability_slot_map) in query.iter_mut() {
            for slot in Slot::variants() {
                if let Some(&matching_ability) = ability_slot_map.get(&slot) {
                    ability_state
                        .set_action_data(matching_ability, slot_state.action_data(slot).clone());
                }
            }
        }
    }

    fn report_abilities_used(query: Query<&ActionState<Ability>>) {
        for ability_state in query.iter() {
            for ability in ability_state.get_just_pressed() {
                dbg!(ability);
            }
        }
    }

    fn handle_movement(
        mut player: Query<&mut Transform, With<Player>>,
        action_query: Query<&ActionState<Movement>, With<Player>>,
        player_resource: Res<PlayerResource>,
        time: Res<Time>,
    ) {
        let mut player_transform = player.single_mut();
        let action_state = action_query.single();

        if action_state.pressed(Movement::Left) {
            player_transform.translation.x -= player_resource.movement_speed * time.delta_seconds();
        } else if action_state.pressed(Movement::Right) {
            player_transform.translation.x += player_resource.movement_speed * time.delta_seconds();
        }
    }

    // a very rough and dirty clamping method
    // FIXME: Get true window dimensions
    fn wrap_player_around_window(mut player: Query<&mut Transform, With<Player>>) {
        let mut player_transform = player.single_mut();

        if player_transform.translation.x < -300. {
            player_transform.translation.x = 250.;
        }

        if player_transform.translation.x > 250. {
            player_transform.translation.x = -300.;
        }
    }
}
