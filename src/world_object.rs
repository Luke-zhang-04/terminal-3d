use crate::terminal;
use crate::vector3::Vector3;

pub trait WorldObject {
    fn vectices(&self) -> Vec<Vector3>;

    fn triangles(&self) -> Vec<(usize, usize, usize)> {
        Vec::new()
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        Vec::new()
    }

    fn vertex_style(&self) -> terminal::Style {
        ('X', terminal::Color::Reset, terminal::Decor::None)
    }

    fn edge_style(&self) -> terminal::Style {
        ('O', terminal::Color::Reset, terminal::Decor::None)
    }

    fn face_style(&self) -> terminal::Style {
        ('.', terminal::Color::Reset, terminal::Decor::None)
    }
}
