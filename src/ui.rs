use crate::settings::Settings;
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::widgets::Slider;
use macroquad::ui::{root_ui, widgets};

pub enum UiEvent {
    Save,
    Clear,
    Scale,
}

pub fn draw_ui(settings: &mut Settings, window_size: &Vec2) -> Option<UiEvent> {
    let mut event = None;
    let widget_width = 200.0;
    widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(widget_width, window_size.y))
        .label("Settings")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            if ui.button(None, "Save") {
                event = Some(UiEvent::Save);
            }
            if ui.button(None, "Clear") {
                event = Some(UiEvent::Clear);
            }
            if ui.button(None, "Scale") {
                event = Some(UiEvent::Scale);
            }
            ui.separator();
            ui.label(None, "Tine Amplitude");
            Slider::new(hash!(), 10f32..200f32).ui(ui, &mut settings.tine_displacement);
            ui.label(None, "Tine Sharpness");
            Slider::new(hash!(), 2f32..30f32).ui(ui, &mut settings.tine_sharpness);
            ui.separator();
        });

    settings.mouse_enabled = !root_ui().is_mouse_over(mouse_position().into());

    event
}
