mod ink;
mod settings;
mod ui;

use crate::ink::{InkDrop, TineLine};
use crate::settings::Settings;
use crate::ui::{draw_ui, UiEvent};
use macroquad::miniquad::start;
use macroquad::miniquad::TextureKind::Texture2D;
use macroquad::prelude::*;

fn draw_shape(points: &Vec<Vec2>, midpoint: Vec2, color: Color) {
    if points.len() < 3 {
        error!(
            "draw_shape: points.len() < 3. Need 3 or more points to draw a shape since it uses \
            triangles."
        );
        return;
    }
    let _ = points.iter().fold(points[points.len() - 1], |prev, point| {
        draw_triangle(midpoint, prev, *point, color);
        *point
    });
}

fn random_color() -> Color {
    Color::new(
        rand::gen_range(0.0, 1.0),
        rand::gen_range(0.0, 1.0),
        rand::gen_range(0.0, 1.0),
        1.0,
    )
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Marbling".to_owned(),
        window_width: 1000,
        window_height: 800,
        ..Default::default()
    }
}

pub struct TineDrag {
    start: Vec2,
}

#[macroquad::main(window_conf)]
async fn main() {
    let window_size = {
        let conf = window_conf();
        vec2(conf.window_width as f32, conf.window_height as f32)
    };
    let (screen_camera, mut render_camera) = {
        let rect = Rect::new(0.0, 0.0, 1000.0, 800.0);
        (
            Camera2D::from_display_rect(rect),
            Camera2D::from_display_rect(rect),
        )
    };

    let export_render_target = render_target(window_size.x as u32, window_size.y as u32);
    render_camera.render_target = Some(export_render_target);

    let mut droplets = vec![];
    droplets.push(InkDrop::new(
        Vec2::new(200.0, 200.0),
        100.0,
        Color::new(1.0, 0.0, 0.0, 1.0),
    ));
    let mut tine_drag = None;
    let mut settings = Settings::default();
    let mut locked_for_export = false;

    loop {
        set_camera(&render_camera);
        clear_background(WHITE);

        if is_mouse_button_pressed(MouseButton::Left) & &settings.mouse_enabled {
            let pos = screen_camera.screen_to_world(mouse_position().into());
            let r = 100.0;
            let new_drop = InkDrop::new(pos, r, random_color());

            for drop in droplets.iter_mut() {
                drop.be_marbled_by(&new_drop, r);
            }
            droplets.push(new_drop);
        }

        if is_mouse_button_pressed(MouseButton::Right) && settings.mouse_enabled {
            tine_drag = Some(TineDrag {
                start: screen_camera.screen_to_world(mouse_position().into()),
            });
        }

        if is_mouse_button_released(MouseButton::Right) && settings.mouse_enabled {
            if let Some(TineDrag { start }) = tine_drag {
                let end: Vec2 = screen_camera.screen_to_world(mouse_position().into());
                let tine_line = TineLine::new(
                    start,
                    end - start,
                    settings.tine_displacement,
                    settings.tine_sharpness,
                );
                for drop in droplets.iter_mut() {
                    drop.be_tine_lined(&tine_line);
                }
            }
            tine_drag = None;
        }

        // Draw all Drops
        for drop in droplets.iter() {
            drop.draw();
        }

        set_camera(&screen_camera);
        if let Some(target) = &render_camera.render_target {
            draw_texture_ex(
                &target.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    flip_x: false,
                    flip_y: true,
                    ..Default::default()
                },
            );
        }
        // Display guideline for tine line
        if let Some(TineDrag { start }) = tine_drag {
            let end: Vec2 = screen_camera.screen_to_world(mouse_position().into());
            draw_line(start.x, start.y, end.x, end.y, 2.0, BLACK);
        }

        if let Some(event) = draw_ui(&mut settings, &window_size) {
            match event {
                UiEvent::Save => {
                    if let Some(target) = &render_camera.render_target {
                        let filename =
                            format!("marble_{}.png", chrono::Utc::now().format("%d_%m_%Y_%H%M"));
                        target.texture.get_texture_data().export_png(&filename);
                        println!("Saved marbled pattern to {}", filename);
                    }
                }
                UiEvent::Clear => {
                    droplets.clear();
                }
                UiEvent::Scale => {
                    for drop in droplets.iter_mut() {
                        drop.scale_points();
                    }
                }
            }
        }

        // FPS
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., BLACK);

        // Done!
        next_frame().await;
    }
}
