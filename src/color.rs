use std::ops::{Add, Div, Mul, Sub};

use image::Rgb;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    #[inline(always)]
    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    #[inline(always)]
    pub fn gray(lum: f32) -> Self {
        Self {
            r: lum,
            g: lum,
            b: lum,
        }
    }

    #[inline(always)]
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    #[inline(always)]
    pub fn to_rgb(&self, gamma: f32) -> Rgb<u8> {
        let r = (self.r.powf(gamma.recip()) * 255.999) as u8;
        let g = (self.g.powf(gamma.recip()) * 255.999) as u8;
        let b = (self.b.powf(gamma.recip()) * 255.999) as u8;
        Rgb([r, g, b])
    }

    #[inline(always)]
    pub fn from_rgb_24(value: Rgb<u8>, gamma: f32) -> Self {
        Self {
            r: (value.0[0] as f32 / 255.0).powf(gamma),
            g: (value.0[1] as f32 / 255.0).powf(gamma),
            b: (value.0[2] as f32 / 255.0).powf(gamma),
        }
    }

    #[inline(always)]
    pub fn r(&self) -> f32 {
        self.r
    }

    #[inline(always)]
    pub fn g(&self) -> f32 {
        self.g
    }

    #[inline(always)]
    pub fn b(&self) -> f32 {
        self.b
    }
}

impl Add<Color> for Color {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub<Color> for Color {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Div<Color> for Color {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<f32> for Color {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    #[inline(always)]
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl Div<Color> for f32 {
    type Output = Color;

    #[inline(always)]
    fn div(self, rhs: Color) -> Self::Output {
        Color {
            r: self / rhs.r,
            g: self / rhs.g,
            b: self / rhs.b,
        }
    }
}
