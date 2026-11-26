#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    value: [f32; 4],
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Color {
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
}

impl Color {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            value: [r, g, b, 1.0],
        }
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            value: [r, g, b, a],
        }
    }
}

impl Color {
    pub const fn value(self) -> [f32; 4] {
        self.value
    }

    pub const fn r(self) -> f32 {
        self.value[0]
    }

    pub const fn g(self) -> f32 {
        self.value[1]
    }

    pub const fn b(self) -> f32 {
        self.value[2]
    }

    pub const fn a(self) -> f32 {
        self.value[3]
    }
}

impl Color {
    pub(crate) fn to_wgpu(self) -> wgpu::Color {
        wgpu::Color {
            r: self.value[0] as f64,
            g: self.value[1] as f64,
            b: self.value[2] as f64,
            a: self.value[3] as f64,
        }
    }
}
