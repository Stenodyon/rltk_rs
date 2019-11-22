use super::{codepage437::to_cp437, Console, RGB};

struct BoxDef {
    hor: u8,
    vert: u8,
    top_left: u8,
    top_right: u8,
    bottom_left: u8,
    bottom_right: u8,
}

const BOX_DEF_SINGLE: BoxDef = BoxDef {
    hor: 196,
    vert: 179,
    top_left: 218,
    top_right: 191,
    bottom_left: 192,
    bottom_right: 217,
};

const BOX_DEF_DOUBLE: BoxDef = BoxDef {
    hor: 205,
    vert: 186,
    top_left: 201,
    top_right: 187,
    bottom_left: 200,
    bottom_right: 188,
};

#[inline]
fn _draw_box(
    console: &mut dyn Console,
    sx: i32,
    sy: i32,
    width: i32,
    height: i32,
    fg: RGB,
    bg: RGB,
    box_def: BoxDef,
) {
    for y in sy..sy + height {
        for x in sx..sx + width {
            console.set(
                x,
                y,
                RGB::from_f32(1.0, 1.0, 1.0),
                RGB::from_f32(0.0, 0.0, 0.0),
                32,
            );
        }
    }

    console.set(sx, sy, fg, bg, box_def.top_left);
    console.set(sx + width - 1, sy, fg, bg, box_def.top_right);
    console.set(sx, sy + height - 1, fg, bg, box_def.bottom_left);
    console.set(
        sx + width - 1,
        sy + height - 1,
        fg,
        bg,
        box_def.bottom_right,
    );
    for x in sx + 1..sx + width {
        console.set(x, sy, fg, bg, box_def.hor);
        console.set(x, sy + height - 1, fg, bg, box_def.hor);
    }
    for y in sy + 1..sy + height {
        console.set(sx, y, fg, bg, box_def.vert);
        console.set(sx + width - 1, y, fg, bg, box_def.vert);
    }
}

/// Draws a box, starting at x/y with the extents width/height using CP437 line characters
pub fn draw_box(
    console: &mut dyn Console,
    sx: i32,
    sy: i32,
    width: i32,
    height: i32,
    fg: RGB,
    bg: RGB,
) {
    _draw_box(console, sx, sy, width, height, fg, bg, BOX_DEF_SINGLE);
}

/// Draws a box, starting at x/y with the extents width/height using CP437 line characters
pub fn draw_box_double(
    console: &mut dyn Console,
    sx: i32,
    sy: i32,
    width: i32,
    height: i32,
    fg: RGB,
    bg: RGB,
) {
    _draw_box(console, sx, sy, width, height, fg, bg, BOX_DEF_DOUBLE);
}

/// Draws a horizontal progress bar
#[allow(clippy::too_many_arguments)]
pub fn draw_bar_horizontal(
    console: &mut dyn Console,
    sx: i32,
    sy: i32,
    width: i32,
    n: i32,
    max: i32,
    fg: RGB,
    bg: RGB,
) {
    let percent = n as f32 / max as f32;
    let fill_width = (percent * width as f32) as i32;
    for x in 0..width {
        if x <= fill_width {
            console.set(sx + x, sy, fg, bg, to_cp437('▓'));
        } else {
            console.set(sx + x, sy, fg, bg, to_cp437('░'));
        }
    }
}

/// Draws a vertical progress bar
#[allow(clippy::too_many_arguments)]
pub fn draw_bar_vertical(
    console: &mut dyn Console,
    sx: i32,
    sy: i32,
    height: i32,
    n: i32,
    max: i32,
    fg: RGB,
    bg: RGB,
) {
    let percent = n as f32 / max as f32;
    let fill_height = height - ((percent * height as f32) as i32);
    for y in 0..height {
        if y >= fill_height {
            console.set(sx, sy + y, fg, bg, to_cp437('▓'));
        } else {
            console.set(sx, sy + y, fg, bg, to_cp437('░'));
        }
    }
}
