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
    let depth_inc = dz / dx as f64;
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

        depth += depth_inc;
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

/// Return alpha, beta, gamma barycentric weights of point inside triangle with area
/// Area of `triangle` is a parameter so it isn't re-calculated every time
fn get_bary_weights(point: Vector3, triangle: VertexTriple, area: f64) -> (f64, f64, f64) {
    (
        get_triangle_area((point, triangle.1, triangle.2)) / area,
        get_triangle_area((point, triangle.2, triangle.0)) / area,
        get_triangle_area((point, triangle.0, triangle.1)) / area,
    )
}

/// Implementation of a bounding-box-style triangle face renderer, which finds a bounding rectangle
/// around the 3 points, and then tests every point in the box to see if it is in the triangle.
/// All vectors have x and y components relative to the camera screen, and the z component
/// represents the distance from the screen.
/// Note that the algorithm will `generate` points regardless of which way the triangle is "facing"
/// (i.e no back-face culling is done). If a triangle is not facing the camera, this function
/// should not be invoked at all.
pub fn bounding_box_triangle_3d(vertices: VertexTriple, mut generate: impl FnMut((i64, i64), f64)) {
    let triangle_area = get_triangle_area(vertices);

    // Degenerate triangle
    if triangle_area.abs() < f64::EPSILON {
        return;
    }

    let vertices_sorted_y = sort_by_y(vertices);
    let vertices_sorted_x = sort_by_x(vertices);

    let y_range = (
        vertices_sorted_y.0.y.round() as i64,
        vertices_sorted_y.2.y.round() as i64,
    );
    let x_range = (
        vertices_sorted_x.0.x.round() as i64,
        vertices_sorted_x.2.x.round() as i64,
    );

    // Loop through each coordinate in the bounding box
    for y in y_range.0..=y_range.1 {
        'x_loop: for x in x_range.0..=x_range.1 {
            let point = vector3!(x, y, 0);

            // Determine if (x, y) is inside the triangle defined by vertices
            let (alpha, beta, gamma) = get_bary_weights(point, vertices, triangle_area);

            if alpha < 0.0 || beta < 0.0 || gamma < 0.0 {
                continue;
            }

            // Check all 4 corners, and skip if point is on edge/corner
            // This is super scuffed, but because of the way we calculate depth and the coarsness
            // of the terminal, the face pixels sometimes have their depths calculated to be in front
            // of the shapes edges.
            for dx in [-0.5, 0.5] {
                for dy in [-0.5, 0.5] {
                    let weights =
                        get_bary_weights(point + vector3!(dx, dy, 0), vertices, triangle_area);

                    if weights.0 < 0.0 || weights.1 < 0.0 || weights.2 < 0.0 {
                        continue 'x_loop;
                    }
                }
            }

            let depth = alpha * vertices.0.z + beta * vertices.1.z + gamma * vertices.2.z;

            generate((x, y), depth);
        }
    }

    // Now, fill in the missing lines
    bresenham_line_3d(vertices.0, vertices.1, |pixel, depth| {
        generate(pixel, depth)
    });
    bresenham_line_3d(vertices.1, vertices.2, |pixel, depth| {
        generate(pixel, depth)
    });
    bresenham_line_3d(vertices.2, vertices.0, |pixel, depth| {
        generate(pixel, depth)
    });

    // Unfortunately this is far from perfect, as sometimes there are gaps in the face
    // TODO: re-architect the render pipeline to just use this bounding box method
}
