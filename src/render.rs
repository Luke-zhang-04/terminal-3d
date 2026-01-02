use std::mem::swap;

use crate::vector3;
use crate::vector3::Vector3;

/// Implementation of Bresenhan's line rasterization algorithm, which also reports the depth of
/// each point. All vectors have x and y components relative to the camera screen, and the z
/// component represents the distance from the screen
pub fn bresenham_line_3d(
    _start: Vector3,
    _end: Vector3,
    mut generate: impl FnMut((i64, i64), f64),
) {
    let mut start: Vector3 = _start;
    let mut end: Vector3 = _end;

    let steep = (end.y - start.y).abs() > (end.x - start.x).abs();
    if steep {
        // If steep, swap x and y coordinates
        (start.x, start.y) = (start.y, start.x);
        (end.x, end.y) = (end.y, end.x);
    }
    if end.x < start.x {
        (start, end) = (end, start)
    }

    let (x0, y0, x1, y1) = (
        start.x.round() as i64,
        start.y.round() as i64,
        end.x.round() as i64,
        end.y.round() as i64,
    );
    let dx = x1 - x0; // dx will always be positive
    let dy = y1 - y0;
    let dz = end.z - start.z;
    let depth_inc = dz / (dx + 1) as f64; // Note that with this method, the floating-point error gets magnified.

    let mut err: i64 = 0;
    let mut y = y0;
    let mut depth = start.z;

    for x in x0..=x1 {
        if steep {
            generate((y, x), depth);
        } else {
            generate((x, y), depth);
        }

        err += dy.abs();
        if 2 * err >= dx {
            y += dy.signum();
            err -= dx;
        }

        if x == x1 - 1 {
            depth = end.z; // At the end of the line segment, account for rounding error
        } else {
            depth += depth_inc;
        }
    }
}

type VertexTriple = (Vector3, Vector3, Vector3);

/// Sort points by y, ascending
fn sort_by_y(vertices: VertexTriple) -> VertexTriple {
    let mut result = vertices.clone();

    if result.0.y > result.1.y {
        swap(&mut result.0, &mut result.1);
    }
    if result.0.y > result.2.y {
        swap(&mut result.0, &mut result.2);
    }
    if result.1.y > result.2.y {
        swap(&mut result.1, &mut result.2);
    }

    result
}

/// Sort points by y, ascending
fn sort_by_x(vertices: VertexTriple) -> VertexTriple {
    let mut result = vertices.clone();

    if result.0.x > result.1.x {
        swap(&mut result.0, &mut result.1);
    }
    if result.0.x > result.2.x {
        swap(&mut result.0, &mut result.2);
    }
    if result.1.x > result.2.x {
        swap(&mut result.1, &mut result.2);
    }

    result
}

/// Get signed area of a triangle defined by 3 points using the shoelace formula
fn get_triangle_area((a, b, c): VertexTriple) -> f64 {
    ((b.y - a.y) * (b.x + a.x) + (c.y - b.y) * (c.x + b.x) + (a.y - c.y) * (a.x + c.x)) / 2.0
}

/// Implementation of a bounding-box-style triangle face renderer, which finds a bounding rectangle
/// around the 3 points, and then tests every point in the box to see if it is in the triangle.
/// All vectors have x and y components relative to the camera screen, and the z component
/// represents the distance from the screen.
/// Note that the algorithm will `generate` points regardless of which way the triangle is "facing"
/// (i.e no back-face culling is done). If a triangle is not facing the camera, this function
/// should not be invoked at all.
pub fn bounding_box_triangle_3d(vertices: VertexTriple, mut generate: impl FnMut((i64, i64), f64)) {
    let vertices_sorted_y = sort_by_y(vertices);
    let vertices_sorted_x = sort_by_x(vertices);

    // Normal vector of the plane formed by the triangle
    let plane_normal =
        (vertices_sorted_x.2 - vertices_sorted_x.1) * (vertices_sorted_x.0 - vertices_sorted_x.1);

    let y_range = (
        vertices_sorted_y.0.y.round() as i64,
        vertices_sorted_y.2.y.round() as i64,
    );
    let x_range = (
        vertices_sorted_x.0.x.round() as i64,
        vertices_sorted_x.2.x.round() as i64,
    );

    let triangle_area = get_triangle_area(vertices);

    // Degenerate triangle
    if triangle_area.abs() < f64::EPSILON {
        return;
    }

    // Loop through each coordinate in the bounding box
    for y in y_range.0..=y_range.1 {
        for x in x_range.0..=x_range.1 {
            let point = vector3!(x, y, 0);

            // Determine if (x, y) is inside the triangle defined by vertices
            let alpha = get_triangle_area((point, vertices.1, vertices.2)) / triangle_area;
            let beta = get_triangle_area((point, vertices.2, vertices.0)) / triangle_area;
            let gamma = get_triangle_area((point, vertices.0, vertices.1)) / triangle_area;

            // Compiler will optimize this, right?
            if alpha < 0.0 || beta < 0.0 || gamma < 0.0 {
                continue;
            }

            let distance =
                (point - vertices.0).dot(plane_normal) / vector3!(0, 0, -1).dot(plane_normal);

            generate((x, y), distance);
        }
    }
}
