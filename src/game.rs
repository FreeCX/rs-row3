#![allow(dead_code, unused_variables, unreachable_code)]
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point<T> {
    x: T,
    y: T
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ItemType {
    Empty,
    Red,
    Green,
    Blue,
    Yellow,
    Purple
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RenderState {
    Ready,
    Clean,
}

pub struct Field {
    size: Point<u8>,
    tile: Point<u8>,
    items: Vec<ItemType>,
    indexes: Vec<(usize, usize)>,
    state: RenderState
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

macro_rules! point {
    ($x:expr,$y:expr) => {crate::game::Point::new($x, $y)};
    ($v:expr) => {crate::game::Point::new($v, $v)};
}

impl Distribution<ItemType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ItemType {
        use rand::seq::SliceRandom;
        use ItemType::*;

        let items = vec![Red, Green, Blue, Yellow, Purple];
        *items.choose(rng).unwrap_or(&Red)
    }
}

impl Field {
    pub fn new() -> Field {
        Field {
            state: RenderState::Ready,
            size: point!(0),
            tile: point!(0),
            indexes: Vec::new(),
            items: Vec::new()
        }
    }
    pub fn set_tile_size(&mut self, size: Point<u8>) {
        self.tile = size;
    }
    pub fn set_field_size(&mut self, size: Point<u8>) {
        let block_size = size.x as usize * size.y as usize;
        self.size = size;
        self.items.resize(block_size, ItemType::Empty);
        self.indexes.resize(block_size, (0, 0));
        for y in 0..self.size.y as usize {
            for x in 0..self.size.x as usize {
                let index = self.index(x, y);
                self.items[index] = self.generate_item(x, y);
                self.indexes[index] = (x, y);
            }
        }
    }
    pub fn is_ready(&self) -> bool {
        match self.state {
            RenderState::Ready => true,
            _ => false
        }
    }
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for (index, item) in self.items.iter().enumerate() {
            let (px, py) = self.indexes[index];
            let rect = Rect::new(px as i32 * self.tile.x as i32, py as i32 * self.tile.y as i32, self.tile.x as u32, self.tile.y as u32);
            let color = match item {
                ItemType::Empty => continue,
                ItemType::Red => Color::RGB(255, 0, 0),
                ItemType::Green => Color::RGB(0, 255, 0),
                ItemType::Blue => Color::RGB(0, 0, 255),
                ItemType::Yellow => Color::RGB(255, 255, 0),
                ItemType::Purple => Color::RGB(128, 0, 128),
            };
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).ok();
        }
    }
    pub fn update(&mut self) {
        unimplemented!()
    }
    pub fn swap(&mut self, a: usize, b: usize) {
        unimplemented!()
    }
    pub fn randomize(&mut self) {
        for y in 0..self.size.y as usize {
            for x in 0..self.size.x as usize {
                let index = self.index(x, y);
                self.items[index] = self.generate_item(x, y);
            }
        }
    }
    fn index<T: Into<usize>>(&self, x: T, y: T) -> usize {
        (self.size.y as usize) * y.into() + x.into()
    }
    fn get_neighbors(&self, x: usize, y: usize) -> [ItemType; 2] {
        let mut result = [ItemType::Empty, ItemType::Empty];
        if x > 0 {
            result[0] = self.items[self.index(x - 1, y)];
        }
        if y > 0 {
            result[1] = self.items[self.index(x, y - 1)];
        }
        result
    }
    fn generate_item(&self, x: usize, y: usize) -> ItemType {
        let mut rng = rand::thread_rng();
        let neighbors = self.get_neighbors(x, y);
        'infty: loop {
            let current_item: ItemType = rng.gen();
            for item in &neighbors {
                if item == &current_item {
                    continue 'infty;
                }
            }
            return current_item;
        }
        unreachable!()
    }
}