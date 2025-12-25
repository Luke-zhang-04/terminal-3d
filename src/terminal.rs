use crate::world::World;

// Adapted from https://stackoverflow.com/a/28938235/12370337
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
            for vertex in obj.vectices() {
                println!(
                    "{} {} {} {}",
                    vertex.x, vertex.y, self.term_width, self.term_height
                );
                if vertex.x >= 0.0
                    && vertex.x < self.term_width as f64
                    && vertex.y >= 0.0
                    && vertex.y < self.term_height as f64
                {
                    let index = ((vertex.y.round() as i64) * self.term_width as i64
                        + vertex.x.round() as i64) as usize;

                    if self.display[index].frame != frame || self.display[index].dist > vertex.z {
                        self.display[index] = Character {
                            frame,
                            style: obj.vertex_style(),
                            dist: vertex.z,
                        }
                    }
                }
            }
        }

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for i in 0..self.display.len() {
            if i != 0 && i % (self.term_width as usize) == 0 {
                print!("\n{}", self.display[i].style.0);
            } else {
                print!("{}", self.display[i].style.0);
            }
        }
    }
}
