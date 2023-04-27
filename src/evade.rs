use bevy::prelude::*;

use crate::player::{Player, PlayerResource};

pub struct EvadeEvent;

#[derive(Component)]
pub struct EvadeTimer {
    pub time: Timer,
}

pub struct EvadePlugin;
impl Plugin for EvadePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EvadeEvent>()
            .add_system(Self::start_player_evasion);
    }
}

impl EvadePlugin {
    fn start_player_evasion(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
        mut ev_evade: EventReader<EvadeEvent>,
        player_query: Query<Entity, With<Player>>,
        mut player_resource: ResMut<PlayerResource>,
    ) {
        for _ev in ev_evade.iter() {
            let evade_sfx = asset_server.load("Audio/forceField_000.ogg");

            let player_entity = player_query.single();

            if player_resource.evades > 0 {
                audio.play(evade_sfx);
                player_resource.evades -= 1;
                commands.entity(player_entity).insert(EvadeTimer {
                    time: Timer::from_seconds(1., TimerMode::Once),
                });
            }

            println!("Evades left: {}", player_resource.evades);
        }
    }
}
