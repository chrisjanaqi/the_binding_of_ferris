use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct IsaacUI;

struct FpsText;

impl IsaacUI {
    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn(UiCameraComponents::default())
            .spawn(TextComponents {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    margin: Rect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                text: Text {
                    value: "".to_string(),
                    font: asset_server.load("fonts/nova_mono.ttf"),
                    style: TextStyle {
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                },
                ..Default::default()
            })
            .with(FpsText);
    }

    fn fps_ui(
        mut timer: Local<Timer>,
        time: Res<Time>,
        diagnostics: Res<Diagnostics>,
        mut query: Query<With<FpsText, &mut Text>>,
    ) {
        timer.duration = 1.0;
        timer.repeating = true;
        timer.tick(time.delta_seconds);
        if !timer.just_finished {
            return;
        }

        for mut text in query.iter_mut() {
            if let Some(fps) = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|diag| diag.average())
            {
                text.value = format!("fps: {:.0}", fps);
            }
        }
    }

    fn fps_console(mut timer: Local<Timer>, time: Res<Time>, diagnostics: Res<Diagnostics>) {
        timer.duration = 1.0;
        timer.repeating = true;
        timer.tick(time.delta_seconds);
        if timer.just_finished {
            if let Some(fps) = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|diag| diag.average())
            {
                println!("{:.0} fps", fps);
            }
        }
    }
}

impl Plugin for IsaacUI {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(Self::setup.system())
            .add_systems(vec![Self::fps_console.system(), Self::fps_ui.system()]);
    }
}
