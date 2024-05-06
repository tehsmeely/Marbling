pub struct Settings {
    pub tine_displacement: f32,
    pub tine_sharpness: f32,
    pub mouse_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tine_displacement: 10.0,
            tine_sharpness: 2.0,
            mouse_enabled: false,
        }
    }
}
