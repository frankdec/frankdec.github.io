use bevy::{
    app::{App, Startup},
    camera::Camera2d,
    ecs::{
        resource::Resource,
        system::{Commands, Res, ResMut},
    },
    prelude,
    time::Time,
    DefaultPlugins,
};
use bevy_egui::{
    egui::{
        emath::{RectTransform, Rot2},
        pos2, vec2, CentralPanel, Color32, Frame, Rect, Shape, Slider, Vec2,
    },
    EguiContexts, EguiPlugin, EguiPrimaryContextPass,
};

#[derive(Resource)]
struct RotatingDotsConfig {
    count: u32,
    velocity: f32,
    luminance: u8,
}

impl Default for RotatingDotsConfig {
    fn default() -> Self {
        Self {
            count: 100,
            velocity: 1.0,
            luminance: 200,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .insert_resource(RotatingDotsConfig::default())
        .add_systems(Startup, setup)
        .add_systems(EguiPrimaryContextPass, rotating_dots)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn rotating_dots(
    mut config: ResMut<RotatingDotsConfig>,
    time: Res<Time>,
    mut contexts: EguiContexts,
) -> prelude::Result {
    const DOT_RADIUS: f32 = 1.0;
    const MAX_CANVAS_SIZE: Vec2 = vec2(256.0, 256.0);

    let ctx = contexts.ctx_mut()?;
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Rotating dots");

        Frame::dark_canvas(ui.style()).show(ui, |ui| {
            ui.set_min_size(MAX_CANVAS_SIZE);
            let (_id, rect) = ui.allocate_space(MAX_CANVAS_SIZE);

            let to_screen =
                RectTransform::from_to(Rect::from_x_y_ranges(-1.0..=1.0, -1.0..=1.0), rect);
            let count = config.count;
            let color = Color32::from_additive_luminance(config.luminance);
            let shapes: Vec<Shape> = (1..=count)
                .map(|n| {
                    let radius = n as f32 / count as f32;
                    let omega = radius * config.velocity;
                    let t = time.elapsed_secs();
                    let rotor = Rot2::from_angle(omega * t);
                    let vec = rotor * vec2(radius, 0.0);
                    to_screen * pos2(vec.x, vec.y)
                })
                .map(|point| Shape::circle_filled(point, DOT_RADIUS, color))
                .collect();
            ui.painter().extend(shapes);
        });

        ui.add(Slider::new(&mut config.count, 1..=1000).text("Dot count"));
        ui.add(Slider::new(&mut config.velocity, -3.0..=3.0).text("Velocity"));
        ui.add(Slider::new(&mut config.luminance, u8::MIN..=u8::MAX).text("Luminance"));
    });

    Ok(())
}
