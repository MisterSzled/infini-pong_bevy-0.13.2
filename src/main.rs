use bevy::prelude::*;

mod components;
mod events;
mod plugins;
mod resources;
mod materials;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppStates {
        #[default]
        InGame,
        MainMenu,
}

fn main() {
        App::new()
                .init_state::<AppStates>()

                .add_systems(Startup, resources::textures::loader::load)

                .add_plugins(DefaultPlugins)

                .add_plugins(plugins::camera::Camera)

                .add_plugins(plugins::ui::UI)

                .add_plugins(plugins::match_info::MatchInfo)
                .add_plugins(plugins::player::Player)
                .add_plugins(plugins::enemy::Enemy)
                .add_plugins(plugins::ball::Ball)
                .add_plugins(plugins::input::Input)
                .add_plugins(plugins::wall::Wall)

                .init_resource::<resources::player::player_movement::PlayerAvailableMovement>()
                
                .add_event::<events::movement::movement_request::MovementRequest>()
                .add_event::<events::score::increment_score_event::IncrementScoreEvent>()
                .add_event::<events::score::reset_ball_event::ResetBallEvent>()

                .run();
}