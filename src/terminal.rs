use std::io::{self, Write};

use crate::{render::bresenham_line_3d, world_object::WorldObject};

// Adapted from https://stackoverflow.com/a/28938235/12370337
#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
pub enum Decor {
    None,
    Bold,
    Underline,
    HighIntensity,
    BoldHighIntensity,
}

pub type Style = (char, Color, Decor);

fn get_style_escape(style: Style) -> String {
    if style.1 == Color::Reset {
        return String::from("\x1b[0m");
    }

    let color_num_1 = if style.2 == Decor::HighIntensity || style.2 == Decor::BoldHighIntensity {
        9
    } else {
        3
    };
    let color_num_2 = match style.1 {
        Color::Reset => -1,
        Color::Black => 0,
        Color::Red => 1,
        Color::Green => 2,
        Color::Yellow => 3,
        Color::Blue => 4,
        Color::Purple => 5,
        Color::Cyan => 6,
        Color::White => 7,
    };
    let decor_num = match style.2 {
        Decor::None => 0,
        Decor::Bold => 1,
        Decor::Underline => 4,
        Decor::HighIntensity => 0,
        Decor::BoldHighIntensity => 1,
    };

    format!(
        "\x1b[{decor};{color1}{color2}m",
        decor = decor_num,
        color1 = color_num_1,
        color2 = color_num_2
    )
}

struct Character {
    pub frame: u64,
    pub style: Style,
    pub dist: f64,
}

pub struct Terminal {
    term_width: u16,
    term_height: u16,
    display: Vec<Character>,
}

impl Terminal {
    pub fn new() -> Terminal {
        // \x1b[2J: clear screen
        // \x1b[H: move cursor to top-left
        print!("{esc}[2J", esc = 27 as char);

        Terminal {
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

    pub fn pre_render(&mut self) {
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
    }

    pub fn buffer_world_object(&mut self, obj: &dyn WorldObject, frame: u64) {
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

    pub fn render(&mut self) {
        let mut prev_style: Style = (' ', Color::Reset, Decor::None);
        let mut lock = io::stdout().lock();
        write!(lock, "{esc}[H", esc = 27 as char).unwrap();
        write!(lock, "{}", get_style_escape(prev_style)).unwrap();
        for i in 0..self.display.len() {
            let item = &self.display[i];

            if item.style.1 != prev_style.1 || item.style.2 != prev_style.2 {
                prev_style = item.style;
                write!(lock, "{}", get_style_escape(prev_style)).unwrap();
            }

            if i != 0 && i % (self.term_width as usize) == 0 {
                write!(lock, "\n{}", item.style.0).unwrap();
            } else {
                write!(lock, "{}", item.style.0).unwrap();
            }
        }
        lock.flush().unwrap();
    }
}
