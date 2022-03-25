use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Rgba(pub [f32; 4]);

impl From<[f32; 4]> for Rgba {
    fn from(o: [f32; 4]) -> Self {
        Self([(o[0] * 255.), (o[1] * 255.), (o[2] * 255.), (o[3] * 255.)])
    }
}

impl From<Rgb> for Rgba {
    fn from(o: Rgb) -> Self {
        let o = &o.0;
        Self([o[0], o[1], o[2], 1.])
    }
}

impl Into<[u8; 4]> for Rgba {
    fn into(self) -> [u8; 4] {
        let s = self.0;
        [
            (s[0] * 255.) as u8,
            (s[1] * 255.) as u8,
            (s[2] * 255.) as u8,
            (s[3] * 255.) as u8,
        ]
    }
}

impl Into<[f32; 4]> for Rgba {
    fn into(self) -> [f32; 4] {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Rgb(pub [f32; 3]);

impl From<Rgba> for Rgb {
    fn from(o: Rgba) -> Self {
        let o = &o.0;
        Self([o[0], o[1], o[2]])
    }
}

impl From<[f32; 3]> for Rgb {
    fn from(o: [f32; 3]) -> Self {
        Self([(o[0] * 255.), (o[1] * 255.), (o[2] * 255.)])
    }
}

impl Into<[u8; 3]> for Rgb {
    fn into(self) -> [u8; 3] {
        let s = self.0;
        [
            (s[0] * 255.) as u8,
            (s[1] * 255.) as u8,
            (s[2] * 255.) as u8,
        ]
    }
}

impl Into<[f32; 3]> for Rgb {
    fn into(self) -> [f32; 3] {
        self.0
    }
}

impl Into<egui::Rgba> for Rgba {
    fn into(self) -> egui::Rgba {
        egui::Rgba::from_rgba_premultiplied(self.0[0], self.0[1], self.0[2], self.0[3])
    }
}
impl From<egui::color::Rgba> for Rgba {
    fn from(o: egui::color::Rgba) -> Self {
        Self(o.to_array())
    }
}
