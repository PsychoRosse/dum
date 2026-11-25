use crate::Color;

#[derive(Debug, Clone)]
pub struct Material {
    color: Color,
}

impl Material {
    pub const fn new(color: Color) -> Self {
        Self { color }
    }

    pub const fn from_color(color: Color) -> Self {
        Self::new(color)
    }

    pub const fn color(&self) -> Color {
        self.color
    }

    pub const fn color_value(&self) -> [f32; 4] {
        self.color.value()
    }
}
