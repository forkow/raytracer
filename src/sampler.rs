use image::Rgb;

use crate::color::Color;

pub trait Sample2D<T> {
    fn sample(&self, u: f32, v: f32) -> T;
}

#[derive(Debug)]
pub struct Image2DSampler {
    image: image::ImageBuffer<Rgb<u8>, Vec<u8>>,
}

pub type Sampler2DFunction<T> = fn(u: f32, v: f32) -> T;

#[derive(Debug)]
pub enum Sampler2D<T: Copy>
where
    Image2DSampler: Sample2D<T>,
{
    Static(T),
    Image(Image2DSampler),
    Function(Sampler2DFunction<T>),
}

impl<T: Copy> Sampler2D<T>
where
    Image2DSampler: Sample2D<T>,
{
    #[inline(always)]
    pub fn sample(&self, u: f32, v: f32) -> T {
        match self {
            Sampler2D::Static(fixed) => *fixed,
            Sampler2D::Image(image_sampler) => image_sampler.sample(u, v),
            Sampler2D::Function(func) => func(u, v),
        }
    }
}

impl Image2DSampler {
    pub fn new(
        image: image::DynamicImage,
    ) -> Self {
        Self {
            image: image.into_rgb8(),
        }
    }
}

impl Sample2D<Color> for Image2DSampler {
    fn sample(&self, u: f32, v: f32) -> Color {
        let factor = 1.0;
        let x = (u * factor).fract().abs() * self.image.width() as f32;
        let y = (((1.0 - v) * factor).fract().abs()) *
            self.image.height() as f32;
        let (x, y) = (x as u32, y as u32);
        Color::from_rgb_24(*self.image.get_pixel(x, y), 1.0)
    }
}
