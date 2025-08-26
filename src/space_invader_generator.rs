use hello_rust::grid_painter::GridPainter;

#[derive(Debug)]
pub struct Offset {
    pub x: i32,

    pub y: i32,
}
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
pub struct GenerateSpaceInvaderInput {
    pub body_o0: Offset,
    pub body_o1: Offset,
    pub body_o2: Offset,
    pub body_o3: Offset,
    pub tentacle_o0: Offset,
    pub tentacle_o1: Offset,
    pub m_tentacle_o0: Offset,
    pub m_tentacle_o1: Offset,
    pub horns_o0: Offset,
    pub horns_o1: Offset,
}

fn mirror_fill_triangle_offset(
    grid: &mut GridPainter,
    center: &Point,
    a: &Offset,
    b: &Offset,
    c: &Offset,
) {
    grid.fill_triangle_scanline(
        (center.x + a.x, center.y + a.y),
        (center.x + a.x + b.x, center.y + a.y + b.y),
        (center.x + a.x + c.x, center.y + a.y + c.y),
        true,
    );

    grid.fill_triangle_scanline(
        (center.x - a.x, center.y + a.y),
        (center.x - a.x - b.x, center.y + a.y + b.y),
        (center.x - a.x - c.x, center.y + a.y + c.y),
        true,
    );
}
pub fn generate_space_invader(input: &GenerateSpaceInvaderInput) {
    let center = Point { x: 16, y: 10 };

    let grid: &mut GridPainter = &mut GridPainter::new();

    mirror_fill_triangle_offset(
        grid,
        &center,
        &Offset { x: 0, y: 0 },
        &input.body_o0,
        &input.body_o1,
    );
    mirror_fill_triangle_offset(
        grid,
        &center,
        &Offset { x: 0, y: 0 },
        &input.body_o1,
        &input.body_o2,
    );
    mirror_fill_triangle_offset(
        grid,
        &center,
        &Offset { x: 0, y: 0 },
        &input.body_o2,
        &input.body_o3,
    );
    mirror_fill_triangle_offset(
        grid,
        &center,
        &Offset { x: 0, y: 0 },
        &input.body_o3,
        &input.body_o0,
    );

    mirror_fill_triangle_offset(
        grid,
        &center,
        &input.body_o0,
        &input.tentacle_o0,
        &input.tentacle_o1,
    );

    mirror_fill_triangle_offset(
        grid,
        &center,
        &input.body_o1,
        &input.m_tentacle_o0,
        &input.m_tentacle_o1,
    );

        mirror_fill_triangle_offset(
        grid,
        &center,
        &input.body_o2,
        &input.horns_o0,
        &input.horns_o1,
    );

    // eyes
    grid.set_px(((center.x - 1) as usize, (center.y) as usize), false);
    grid.set_px(((center.x - 2) as usize, (center.y) as usize), false);
    grid.set_px(((center.x + 1) as usize, (center.y) as usize), false);
    grid.set_px(((center.x + 2) as usize, (center.y) as usize), false);

    grid.print();
}
