use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Rgba(pub [f64; 4]);

impl From<[f32; 4]> for Rgba {
    fn from(o: [f32; 4]) -> Self {
        [o[0] as f64, o[1] as f64, o[2] as f64, o[3] as f64].into()
    }
}

impl From<[f64; 4]> for Rgba {
    fn from(o: [f64; 4]) -> Self {
        Self(o)
    }
}

impl From<f64> for Rgba {
    fn from(o: f64) -> Self {
        Self([o, o, o, 1.])
    }
}

impl From<[u8; 4]> for Rgba {
    fn from(o: [u8; 4]) -> Self {
        Self([o[0] as f64 / 255., o[1] as f64 / 255., o[2] as f64 / 255., o[3] as f64 / 255.])
    }
}

impl From<u8> for Rgba {
    fn from(o: u8) -> Self {
        [o, o, o, 1].into()
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
        let z = &self.0;
        [z[0] as f32, z[1] as f32, z[2] as f32, z[3] as f32]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Rgb(pub [f64; 3]);

impl From<Rgba> for Rgb {
    fn from(o: Rgba) -> Self {
        let o = &o.0;
        Self([o[0], o[1], o[2]])
    }
}

impl From<f64> for Rgb {
    fn from(o: f64) -> Self {
        Self([o, o, o])
    }
}

impl From<[f32; 3]> for Rgb {
    fn from(o: [f32; 3]) -> Self {
        [o[0] as f64, o[1] as f64, o[2] as f64].into()
    }
}
impl From<[f64; 3]> for Rgb {
    fn from(o: [f64; 3]) -> Self {
        Self(o)
    }
}

impl From<[u8; 3]> for Rgb {
    fn from(o: [u8; 3]) -> Self {
        Self([o[0] as f64 / 255., o[1] as f64 / 255., o[2] as f64 / 255.])
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
        let z = &self.0;
        [z[0] as f32, z[1] as f32, z[2] as f32]
    }
}

impl Into<egui::Rgba> for Rgba {
    fn into(self) -> egui::Rgba {
        egui::Rgba::from_rgba_premultiplied(self.0[0] as f32, self.0[1] as f32, self.0[2] as f32, self.0[3] as f32)
    }
}

impl From<egui::Rgba> for Rgba {
    fn from(o: egui::Rgba) -> Self {
        o.to_array().into()
    }
}
