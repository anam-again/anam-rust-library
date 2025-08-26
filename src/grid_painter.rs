pub struct GridPainter {
    rows: [u32; 32],
}

impl GridPainter {
    pub fn new() -> GridPainter {
        let rows: &mut [u32; 32] = &mut [0; 32];
        return GridPainter { rows: *rows };
    }

    fn print_line(bits: u32) {
        // let num: u8 = 0b10110101; // Example number
        for i in 0..32 {
            let bit = (bits >> i) & 1;
            if bit == 1 {
                print!("▓")
            } else {
                print!("░");
            }
        }
        println!();
    }

    pub fn print(self: &Self) {
        for u in self.rows {
            GridPainter::print_line(u);
        }
    }

    pub fn set_px(self: &mut Self, (x, y): (usize, usize), set_to: bool) {
        if x >= 32 || y >= 32 {
            return;
        }
        let row_mask = 2u32.pow(x as u32);
        let mut row = self.rows[y];
        if set_to {
            row = (row) | (row_mask as u32);
            self.rows[y] = row;
        } else {
            row = (row) & !(row_mask as u32);
            self.rows[y] = row;
        }
    }

    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    pub fn set_line(self: &mut Self, (x0, y0): (i32, i32), (x1, y1): (i32, i32), set_to: bool) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut error = dx + dy;

        let (mut x, mut y) = (x0, y0);

        loop {
            self.set_px((x as usize, y as usize), set_to);
            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * error;

            if e2 >= dy {
                if x == x1 {
                    break;
                }
                error += dy;
                x += sx;
            }

            if e2 <= dx {
                if y == y1 {
                    break;
                }
                error += dx;
                y += sy;
            }
        }
    }

    pub fn fill_triangle_scanline(
        self: &mut Self,
        (x0, y0): (i32, i32),
        (x1, y1): (i32, i32),
        (x2, y2): (i32, i32),
        set_to: bool,
    ) {
        // Sort vertices by y-coordinate
        let mut vertices = [y0, y1, y2];
        vertices.sort();

        let [v0, v1, v2] = vertices;

        // Check for degenerate triangle (flat top/bottom)
        if v0 == v1 {
            self.set_line((x0, y0), (x1, y1), set_to);
        } else if v1 == v2 {
            self.set_line((x1, y1), (x2, y2), set_to);
        }

        // Calculate slopes for both sides
        let inv_slope1 = if y1 - y2 != 0 {
            (x1 - x0) as f32 / (y1 - y0) as f32
        } else {
            0.0
        };

        let inv_slope2 = if y2 - y0 != 0 {
            (x2 - x0) as f32 / (y2 - y0) as f32
        } else {
            0.0
        };

        // Fill top part of triangle
        let mut cur_x1 = x0 as f32;
        let mut cur_x2 = x0 as f32;

        for y in y0..=y1 {
            let start_x = cur_x1.min(cur_x2).floor() as i32;
            let end_x = cur_x1.max(cur_x2).ceil() as i32;

            for x in start_x..=end_x {
                self.set_px((x as usize, y as usize), set_to);
            }

            cur_x1 += inv_slope1;
            cur_x2 += inv_slope2;
        }

        // Calculate slopes for bottom part
        let inv_slope3 = if y2 - y1 != 0 {
            (x2 - x1) as f32 / (y2 - y1) as f32
        } else {
            0.0
        };

        // Fill bottom part of triangle
        let mut cur_x1 = x1 as f32;
        let mut cur_x2 = cur_x2; // Continue from previous

        for y in y1 + 1..=y2 {
            let start_x = cur_x1.min(cur_x2).floor() as i32;
            let end_x = cur_x1.max(cur_x2).ceil() as i32;

            for x in start_x..=end_x {
                self.set_px((x as usize, y as usize), set_to);
            }

            cur_x1 += inv_slope3;
            cur_x2 += inv_slope2;
        }
    }
}
