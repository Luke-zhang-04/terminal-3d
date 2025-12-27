use crate::vector3::Vector3;

/// Implementation of Bresenhan's line rasterization algorithm, which also reports the depth of each point
/// All vectors have x and y components relative  to the camera screen, and the z component represents the distance from the screen
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
