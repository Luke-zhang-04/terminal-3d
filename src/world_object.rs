use crate::terminal;
use crate::vector3::Vector3;

pub trait WorldObject {
    fn vectices(&self) -> Vec<Vector3>;

    /// Triangles are defined by tuples of indexes corresponding to vercites.
    /// They should be defined in counterclockwise order.
    fn triangles(&self) -> Vec<(usize, usize, usize)> {
        Vec::new()
    }

    /// Edges are defined by tuples of indexes corresponding to vertices.
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

    fn update(&mut self, _frame: u64) {}
}
