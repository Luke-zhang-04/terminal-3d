use std::io::{self, Write};

use crate::{render::bresenham_line_3d, world::World};

// Adapted from https://stackoverflow.com/a/28938235/12370337
#[derive(Clone, Copy)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
}

#[derive(Clone, Copy)]
pub enum Decor {
    None,
    Bold,
    Underline,
    Background,
    HighIntensity,
    BoldHighIntensity,
    HighIntensityBackground,
}

pub type Style = (char, Color, Decor);

struct Character {
    pub frame: u64,
    pub style: Style,
    pub dist: f64,
}

pub struct Terminal<'a> {
    world: &'a World,
    term_width: u16,
    term_height: u16,
    display: Vec<Character>,
}

impl Terminal<'_> {
    pub fn new(world: &'_ World) -> Terminal<'_> {
        Terminal {
            world,
            term_width: 0,
            term_height: 0,
            display: vec![],
        }
    }

    // Plot character, assuming x and y are in bounds
    fn plot_character(&mut self, x: u16, y: u16, depth: f64, style: Style, frame: u64) {
        let index = y as usize * self.term_width as usize + x as usize;

        if self.display[index].frame != frame || self.display[index].dist > depth {
            self.display[index] = Character {
                frame,
                style: style,
                dist: depth,
            }
        }
    }

    fn is_in_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0 && (x as u16) < self.term_width && y >= 0 && (y as u16) < self.term_height
    }

    pub fn render(&mut self, frame: u64) {
        let size = termsize::get().unwrap();

        if self.term_height != size.rows || self.term_height != size.cols {
            self.term_height = size.rows;
            self.term_width = size.cols;
            let char_count = size.rows as usize * size.cols as usize;

            self.display = Vec::with_capacity(char_count);

            for _ in 0..char_count {
                self.display.push(Character {
                    frame: 0,
                    style: (' ', Color::Reset, Decor::None),
                    dist: 0.0,
                });
            }
        }

        for obj in self.world.values() {
            let vertices = obj.vectices();
            let vertex_style = obj.vertex_style();
            let edge_style = obj.edge_style();
            for vertex in &vertices {
                let (x, y) = (vertex.x.round() as i64, vertex.y.round() as i64);
                if self.is_in_bounds(x, y) {
                    self.plot_character(x as u16, y as u16, vertex.z, vertex_style, frame);
                }
            }

            for edge in obj.edges() {
                bresenham_line_3d(
                    vertices[edge.0],
                    vertices[edge.1],
                    |pixel: (i64, i64), depth: f64| {
                        if self.is_in_bounds(pixel.0, pixel.1) {
                            self.plot_character(
                                pixel.0 as u16,
                                pixel.1 as u16,
                                depth,
                                edge_style,
                                frame,
                            );
                        }
                    },
                );
            }
        }

        let mut lock = io::stdout().lock();
        write!(lock, "{esc}[2J{esc}[1;1H", esc = 27 as char).unwrap();
        for i in 0..self.display.len() {
            if i != 0 && i % (self.term_width as usize) == 0 {
                write!(lock, "\n{}", self.display[i].style.0).unwrap();
            } else {
                write!(lock, "{}", self.display[i].style.0).unwrap();
            }
        }
        lock.flush().unwrap();
    }
}
