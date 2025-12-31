use std::io::{self, IsTerminal, Write};

use libc;

use crate::{camera::Camera, render::bresenham_line_3d, world_object::WorldObject};

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

// http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
#[repr(C)]
#[derive(Debug)]
pub struct Size {
    pub rows: libc::c_ushort,
    pub cols: libc::c_ushort,
}

// Get UNIX terminal size
fn get_term_size() -> Option<Size> {
    if !std::io::stdout().is_terminal() {
        return None;
    }
    let mut size = Size { rows: 0, cols: 0 };
    let result = unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut size) };

    if result == 0 { Some(size) } else { None }
}

impl Terminal {
    pub fn new() -> Terminal {
        // \x1b[2J: clear screen
        // \x1b[H: move cursor to top-left
        print!("{esc}[2J", esc = 27 as char);

        let size = get_term_size().unwrap();
        Terminal {
            term_width: size.cols,
            term_height: size.rows,
            display: vec![],
        }
    }

    pub fn get_term_size(&self) -> (u16, u16) {
        (self.term_width, self.term_height * 2) // Report height as doubled
    }

    // Plot character, assuming x and y are in bounds
    fn plot_character(&mut self, x: u16, y: u16, depth: f64, style: Style, frame: u64) {
        // y coordinate should be halved, because monospace characters 2x as tall as they are wide
        let index = (y as f32 / 2.0).floor() as usize * self.term_width as usize + x as usize;

        if self.display[index].frame != frame || self.display[index].dist > depth {
            self.display[index] = Character {
                frame,
                style: style,
                dist: depth,
            }
        }
    }

    fn is_in_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0 && (x as u16) < self.term_width && y >= 0 && y < self.term_height as i64 * 2
    }

    pub fn pre_render(&mut self) {
        let size = get_term_size().unwrap();

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

    pub fn buffer_world_object(&mut self, obj: &dyn WorldObject, camera: &dyn Camera, frame: u64) {
        let vertices = obj.vectices();
        let vertex_style = obj.vertex_style();
        let edge_style = obj.edge_style();
        for vertex in &vertices {
            let pojection = camera.project_vector(*vertex);
            let (x, y) = (pojection.x.round() as i64, pojection.y.round() as i64);
            if self.is_in_bounds(x, y) {
                self.plot_character(x as u16, y as u16, pojection.z, vertex_style, frame);
            }
        }

        for edge in obj.edges() {
            let start = camera.project_vector(vertices[edge.0]);
            let end = camera.project_vector(vertices[edge.1]);

            bresenham_line_3d(start, end, |pixel: (i64, i64), depth: f64| {
                if self.is_in_bounds(pixel.0, pixel.1) {
                    self.plot_character(pixel.0 as u16, pixel.1 as u16, depth, edge_style, frame);
                }
            });
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
