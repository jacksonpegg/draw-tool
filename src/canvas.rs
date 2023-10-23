use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug)]
pub enum CanvasError {
    IndexOutOfBoundsError,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
}
impl Pixel {
    fn new(x: usize, y: usize) -> Self {
        return Pixel { x, y };
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<u32>,
}
impl Canvas {
    fn size(&self) -> usize {
        return self.width * self.height;
    }

    fn get_index(&self, pixel: Pixel) -> Result<usize, CanvasError> {
        let index = pixel.y * self.width + pixel.x;
        if index <= self.pixels.len() {
            return Ok(index);
        }
        Err(CanvasError::IndexOutOfBoundsError)
    }

    fn get_pixel(&self, pixel: Pixel) -> Result<&u32, CanvasError> {
        let index = self.get_index(pixel)?;
        if let Some(x) = self.pixels.get(index) {
            return Ok(x);
        } else {
            return Err(CanvasError::IndexOutOfBoundsError);
        }
    }
    fn get_pixel_mut(&mut self, pixel: Pixel) -> Result<&mut u32, CanvasError> {
        let index = self.get_index(pixel)?;
        if let Some(x) = self.pixels.get_mut(index) {
            return Ok(x);
        } else {
            return Err(CanvasError::IndexOutOfBoundsError);
        }
    }
    fn get_coordinate(&self, index: usize) -> Result<Pixel, CanvasError> {
        if index <= self.pixels.len() {
            let x = index % self.width;
            let y = (index - x) / self.width;
            return Ok(Pixel::new(x, y));
        }
        Err(CanvasError::IndexOutOfBoundsError)
    }
    fn set_pixel(&mut self, pixel: Pixel, color: Color) {
        if let Ok(x) = self.get_pixel_mut(pixel) {
            *x = color.as_u32();
        }
    }
}
#[wasm_bindgen]
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        return Canvas {
            width,
            height,
            pixels: vec![0; width * height],
        };
    }
    pub fn get_pixels_ptr(&self) -> *const u32 {
        return self.pixels.as_ptr();
    }
    pub fn fill(&mut self, color: Color) {
        self.pixels.fill(color.as_u32());
    }
    pub fn draw_circle(&mut self, circle: Circle) {
        for index in 0..self.size() {
            if let Ok(p2) = self.get_coordinate(index) {
                if circle.center.x.abs_diff(p2.x).pow(2) + circle.center.y.abs_diff(p2.y).pow(2)
                    <= circle.radius.pow(2)
                {
                    self.set_pixel(p2, circle.color);
                }
            }
        }
    }
    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for index in 0..self.size() {}
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[wasm_bindgen]
impl Color {
    pub fn new(b: u8, g: u8, r: u8) -> Self {
        return Color { r, g, b, a: 255 };
    }
    pub fn as_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32) | ((self.a as u32) << 24)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Pixel,
    pub radius: usize,
    pub color: Color,
}

trait Shape {
    fn translate(&mut self, x: usize, y: usize);
    fn resize(&mut self, size: f32);
}

#[wasm_bindgen]
impl Circle {
    pub fn new(center: Pixel, radius: usize, color: Color) -> Self {
        return Circle {
            center,
            radius,
            color,
        };
    }
}

impl Shape for Circle {
    fn translate(&mut self, x: usize, y: usize) {
        self.center.x += x;
        self.center.y += y;
    }
    fn resize(&mut self, size: f32) {
        self.radius = (self.radius as f32 * size) as usize;
    }
}

pub struct Square {
    location: Pixel,
    size: usize,
}

impl Square {
    pub fn new(location: Pixel, size: usize) -> Self {
        return Square { location, size };
    }
}

impl Shape for Square {
    fn translate(&mut self, x: usize, y: usize) {
        self.location.x += x;
        self.location.y += y;
    }
    fn resize(&mut self, size: f32) {
        self.size = (self.size as f32 * size) as usize;
    }
}
