use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct IsaacUI;

struct FpsText;

impl IsaacUI {
    fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn(CameraUiBundle::default())
            .spawn(TextBundle {
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
                        alignment: TextAlignment {
                            horizontal: HorizontalAlign::Right,
                            ..Default::default()
                        },
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
        mut query: Query<&mut Text, With<FpsText>>,
    ) {
        timer.set_duration(1.0);
        timer.set_repeating(true);
        if timer.tick(time.delta_seconds()).just_finished() {
            for mut text in query.iter_mut() {
                if let Some(fps) = diagnostics
                    .get(FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|diag| diag.average())
                {
                    text.value = format!("fps: {:.0}", fps);
                }
            }
        }
    }

    fn fps_console(mut timer: Local<Timer>, time: Res<Time>, diagnostics: Res<Diagnostics>) {
        timer.set_duration(1.0);
        timer.set_repeating(true);
        if timer.tick(time.delta_seconds()).just_finished() {
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
            .add_system(Self::fps_console.system())
            .add_system(Self::fps_ui.system());
    }
}
