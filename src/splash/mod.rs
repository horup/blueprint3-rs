use bevy::{audio::Mp3Loader, prelude::*};

use crate::{AppState, DelayState, Hud};

pub struct SplashPlugin;

fn show_splash(mut hud:ResMut<Hud>, audio:Res<Audio>, asset_server:Res<AssetServer>) {
    hud.center_text = "Some Tank Game!\nBy Horup".into();
    hud.top_right_text = "build date 2021-10-10".into();
    hud.top_left_text = "v1.0".into();

    let music = asset_server.load("music/test.mp3");
    audio.play(music);
}

fn hide_splash(mut hud:ResMut<Hud>) {
    hud.clear_texts();
}

fn update(mouse_input:Res<Input<MouseButton>>, mut app_state:ResMut<DelayState<AppState>>, mut hud:ResMut<Hud>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let time = 0.5;
        app_state.set(AppState::InGame, time);
        hud.fade(time, time, Color::BLACK);
    }
}

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Splash).with_system(show_splash.system()));
        app.add_system_set(SystemSet::on_exit(AppState::Splash).with_system(hide_splash.system()));
        app.add_system_set(SystemSet::on_update(AppState::Splash).with_system(update.system()));
    }
}