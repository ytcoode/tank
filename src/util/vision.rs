// returns (x1, x2, y1, y2)
pub fn range(
    map_width: u32,
    map_height: u32,
    vision_width: u32,
    vision_height: u32,
    x: u32,
    y: u32,
) -> (u32, u32, u32, u32) {
    assert!(x < map_width);
    assert!(y < map_height);

    let vx = vision_width / 2;
    let vy = vision_height / 2;

    let mut x1 = if x <= vx { 0 } else { x - vx };
    let mut x2 = x1 + vision_width;

    if x2 > map_width {
        x1 -= (x2 - map_width).min(x1);
        x2 = x1 + vision_width;
    }

    let mut y1 = if y <= vy { 0 } else { y - vy };
    let mut y2 = y1 + vision_height;

    if y2 > map_height {
        y1 -= (y2 - map_height).min(y1);
        y2 = y1 + vision_height;
    }

    (x1, x2, y1, y2)
}
