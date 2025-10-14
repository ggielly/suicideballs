use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::game::{World, Vector2D, GravityMode};

const HUD_X_OFFSET: i32 = 600;

pub fn render(canvas: &mut Canvas<Window>, world: &World) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(20, 20, 30));
    canvas.clear();

    // Draw simulation items
    draw_arc(
        canvas,
        Vector2D { x: world.config.sim_width as f32 / 2.0, y: world.config.screen_height as f32 / 2.0 },
        world.config.circle_radius,
        world.circle_angle + world.config.circle_gap_angle / 2.0,
        world.circle_angle - world.config.circle_gap_angle / 2.0 + 2.0 * std::f32::consts::PI,
        world.config.circle_thickness as i32,
        Color::RGB(200, 200, 220)
    )?;

    for ball in &world.balls {
        canvas.set_draw_color(Color::RGBA(255, 255, 255, 100));
        canvas.draw_line((ball.position.x as i32, ball.position.y as i32), (ball.old_position.x as i32, ball.old_position.y as i32))?;
        draw_filled_circle(canvas, ball.position.x as i32, ball.position.y as i32, ball.radius as i32, ball.color)?;
    }

    // Draw HUD background and separator
    canvas.set_draw_color(Color::RGB(30, 30, 40));
    canvas.fill_rect(sdl2::rect::Rect::new(HUD_X_OFFSET, 0, world.config.screen_width - world.config.sim_width, world.config.screen_height))?;
    canvas.set_draw_color(Color::RGB(100, 100, 120));
    canvas.draw_line((HUD_X_OFFSET, 0), (HUD_X_OFFSET, world.config.screen_height as i32))?;

    // Draw HUD text
    let bounciness_text = format!("BOUNCE: {:.2}", world.bounciness);
    let ball_count_text = format!("BALLS: {}", world.balls.len());
    let fps_text = format!("FPS: {}", world.fps);
    let wall_col_text = format!("WALL COLS: {}", world.wall_collisions);
    let ball_col_text = format!("BALL COLS: {}", world.ball_collisions);
    let gravity_text = format!("GRAVITY: {:?}", world.gravity_mode).to_uppercase();


    draw_text(canvas, &fps_text, HUD_X_OFFSET + 20, 20, 3, Color::WHITE)?;
    draw_text(canvas, &ball_count_text, HUD_X_OFFSET + 20, 50, 3, Color::WHITE)?;
    draw_text(canvas, &wall_col_text, HUD_X_OFFSET + 20, 80, 3, Color::WHITE)?;
    draw_text(canvas, &ball_col_text, HUD_X_OFFSET + 20, 110, 3, Color::WHITE)?;
    draw_text(canvas, &bounciness_text, HUD_X_OFFSET + 20, 140, 3, Color::WHITE)?;
    draw_text(canvas, &gravity_text, HUD_X_OFFSET + 20, 170, 3, Color::WHITE)?;

    canvas.present();
    Ok(())
}

fn draw_text(canvas: &mut Canvas<Window>, text: &str, x: i32, y: i32, scale: u32, color: Color) -> Result<(), String> {
    canvas.set_draw_color(color);
    let mut current_x = x;
    for ch in text.chars() {
        let char_map = get_char_map(ch.to_ascii_uppercase());
        for (row_idx, row) in char_map.iter().enumerate() {
            for (col_idx, &pixel) in row.iter().enumerate() {
                if pixel == 1 {
                    let rect = sdl2::rect::Rect::new(
                        current_x + (col_idx as u32 * scale) as i32,
                        y + (row_idx as u32 * scale) as i32,
                        scale,
                        scale
                    );
                    canvas.fill_rect(rect)?;
                }
            }
        }
        current_x += (6 * scale) as i32; // 5 columns + 1 for spacing
    }
    Ok(())
}

fn get_char_map(c: char) -> [[u8; 5]; 7] {
    match c {
        '.' => [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 1, 1, 0, 0],
        ],
        '0' => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 1, 1],
            [1, 0, 1, 0, 1],
            [1, 1, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ],
        '1' => [
            [0, 0, 1, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ],
        '2' => [
            [1, 1, 1, 1, 0],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
        ],
        '3' => [
            [1, 1, 1, 1, 0],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
        ],
        '4' => [
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 1, 0],
        ],
        '5' => [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
            [0, 0, 0, 0, 0],
        ],
        '6' => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 0],
        ],
        '7' => [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
        ],
        '8' => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 0],
        ],
        '9' => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 0],
        ],
        'A' => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 0, 0, 0, 0],
        ],
        'B' => [
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
            [0, 0, 0, 0, 0],
        ],
        'C' => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 0],
        ],
        'F' => [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
        'G' => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 0],
            [1, 0, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 0],
        ],
        'I' => [
            [1, 1, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0],
        ],
        'L' => [
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0],
        ],
        'O' => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 0],
        ],
        'P' => [
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
        'R' => [
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
            [1, 0, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 0, 1],
            [0, 0, 0, 0, 0],
        ],
        'S' => [
            [0, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
        'T' => [
            [1, 1, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0],
        ],
        'V' => [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0],
        ],
        'W' => [
            [1, 0, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 1, 0, 1, 1],
            [1, 1, 0, 1, 1],
            [1, 0, 0, 0, 1],
            [0, 0, 0, 0, 0],
        ],
        ':' => [
            [0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
        ' ' => [[0; 5]; 7],
        _ => [[0; 5]; 7], // Blank for unknown chars
    }
}

fn draw_filled_circle(canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, radius: i32, color: Color) -> Result<(), String> {
    canvas.set_draw_color(color);
    for y in -radius..=radius {
        let x_span = ((radius.pow(2) - y.pow(2)) as f32).sqrt() as i32;
        canvas.draw_line((center_x - x_span, center_y + y), (center_x + x_span, center_y + y))?;
    }
    Ok(())
}

fn draw_arc(canvas: &mut Canvas<Window>, center: Vector2D, radius: f32, start_angle: f32, end_angle: f32, thickness: i32, color: Color) -> Result<(), String> {
    canvas.set_draw_color(color);
    let step = 0.01;
    for i in 0..thickness {
        let r = radius - (thickness as f32 / 2.0) + i as f32;
        let mut angle = start_angle;
        while angle < end_angle {
            let p1 = Vector2D { x: center.x + r * angle.cos(), y: center.y + r * angle.sin() };
            let p2 = Vector2D { x: center.x + r * (angle + step).cos(), y: center.y + r * (angle + step).sin() };
            canvas.draw_line((p1.x as i32, p1.y as i32), (p2.x as i32, p2.y as i32))?;
            angle += step;
        }
    }
    Ok(())
}
