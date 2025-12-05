use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::game::{World, Vector2D, Ball};

const HUD_X_OFFSET: i32 = 600;
const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

pub fn render(canvas: &mut Canvas<Window>, world: &World) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(20, 20, 30));
    canvas.clear();

    // Draw simulation items
    draw_arc(
        canvas,
        world.circle_center,
        world.config.circle_radius,
        world.circle_angle + world.config.circle_gap_angle * 0.5,
        world.circle_angle - world.config.circle_gap_angle * 0.5 + TWO_PI,
        world.config.circle_thickness as i32,
        Color::RGB(200, 200, 220)
    )?;

    for ball in &world.balls {
        // Traînée de mouvement
        canvas.set_draw_color(Color::RGBA(255, 255, 255, 100));
        canvas.draw_line((ball.position.x as i32, ball.position.y as i32), (ball.old_position.x as i32, ball.old_position.y as i32))?;
        
        // Dessiner la balle avec rotation
        draw_ball_with_rotation(canvas, ball)?;
    }

    // Draw HUD background and separator
    canvas.set_draw_color(Color::RGB(30, 30, 40));
    canvas.fill_rect(sdl2::rect::Rect::new(HUD_X_OFFSET, 0, world.config.screen_width - world.config.sim_width, world.config.screen_height))?;
    canvas.set_draw_color(Color::RGB(100, 100, 120));
    canvas.draw_line((HUD_X_OFFSET, 0), (HUD_X_OFFSET, world.config.screen_height as i32))?;

    // Draw HUD text
    let fps_text = format!("FPS: {}", world.fps);
    let ball_count_text = format!("BALLS: {}", world.balls.len());
    let wall_col_text = format!("WALL: {}", world.total_wall_collisions);
    let ball_col_text = format!("BALL: {}", world.total_ball_collisions);
    let bounciness_text = format!("BOUNCE: {:.2}", world.bounciness);
    let gravity_text = format!("GRAVITY: {:?}", world.gravity_mode).to_uppercase();
    let spawn_text = format!("SPAWN: {}", world.balls_to_spawn);

    draw_text(canvas, &fps_text, HUD_X_OFFSET + 20, 20, 3, Color::WHITE)?;
    draw_text(canvas, &ball_count_text, HUD_X_OFFSET + 20, 50, 3, Color::WHITE)?;
    draw_text(canvas, &wall_col_text, HUD_X_OFFSET + 20, 80, 3, Color::WHITE)?;
    draw_text(canvas, &ball_col_text, HUD_X_OFFSET + 20, 110, 3, Color::WHITE)?;
    draw_text(canvas, &bounciness_text, HUD_X_OFFSET + 20, 140, 3, Color::WHITE)?;
    draw_text(canvas, &gravity_text, HUD_X_OFFSET + 20, 170, 3, Color::WHITE)?;
    draw_text(canvas, &spawn_text, HUD_X_OFFSET + 20, 200, 3, Color::WHITE)?;

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

/// Dessine une balle avec un indicateur de rotation
fn draw_ball_with_rotation(canvas: &mut Canvas<Window>, ball: &Ball) -> Result<(), String> {
    let cx = ball.position.x as i32;
    let cy = ball.position.y as i32;
    let radius = ball.radius as i32;
    
    // 1. Dessiner le cercle rempli (corps de la balle)
    draw_filled_circle(canvas, cx, cy, radius, ball.color)?;
    
    // 2. Dessiner le contour fin (1px plus foncé)
    let outline_color = Color::RGB(
        (ball.color.r as i32 - 40).max(0) as u8,
        (ball.color.g as i32 - 40).max(0) as u8,
        (ball.color.b as i32 - 40).max(0) as u8,
    );
    draw_circle_outline(canvas, cx, cy, radius, outline_color)?;
    
    // 3. Dessiner deux lignes diamétrales qui tournent avec la balle
    let inner_radius = (ball.radius * 0.7) as f32;
    
    // Première ligne (angle de rotation)
    let (sin_r, cos_r) = ball.rotation.sin_cos();
    let x1 = ball.position.x + cos_r * inner_radius;
    let y1 = ball.position.y + sin_r * inner_radius;
    let x2 = ball.position.x - cos_r * inner_radius;
    let y2 = ball.position.y - sin_r * inner_radius;
    
    // Deuxième ligne perpendiculaire
    let x3 = ball.position.x - sin_r * inner_radius;
    let y3 = ball.position.y + cos_r * inner_radius;
    let x4 = ball.position.x + sin_r * inner_radius;
    let y4 = ball.position.y - cos_r * inner_radius;
    
    // Couleur des lignes (plus sombre)
    let line_color = Color::RGB(
        (ball.color.r as i32 - 60).max(0) as u8,
        (ball.color.g as i32 - 60).max(0) as u8,
        (ball.color.b as i32 - 60).max(0) as u8,
    );
    canvas.set_draw_color(line_color);
    canvas.draw_line((x1 as i32, y1 as i32), (x2 as i32, y2 as i32))?;
    canvas.draw_line((x3 as i32, y3 as i32), (x4 as i32, y4 as i32))?;
    
    // 4. Point lumineux sur le bord (comme un reflet qui tourne)
    let highlight_dist = ball.radius * 0.6;
    let highlight_angle = ball.rotation + 0.5; // Légèrement décalé
    let hx = ball.position.x + highlight_angle.cos() * highlight_dist;
    let hy = ball.position.y + highlight_angle.sin() * highlight_dist;
    
    let highlight_color = Color::RGB(
        (ball.color.r as i32 + 60).min(255) as u8,
        (ball.color.g as i32 + 60).min(255) as u8,
        (ball.color.b as i32 + 60).min(255) as u8,
    );
    draw_filled_circle(canvas, hx as i32, hy as i32, 2, highlight_color)?;
    
    Ok(())
}

/// Dessine le contour d'un cercle (algorithme de Bresenham)
fn draw_circle_outline(canvas: &mut Canvas<Window>, cx: i32, cy: i32, radius: i32, color: Color) -> Result<(), String> {
    canvas.set_draw_color(color);
    
    let mut x = radius;
    let mut y = 0;
    let mut err = 0;
    
    while x >= y {
        // Dessiner les 8 points symétriques
        canvas.draw_point((cx + x, cy + y))?;
        canvas.draw_point((cx + y, cy + x))?;
        canvas.draw_point((cx - y, cy + x))?;
        canvas.draw_point((cx - x, cy + y))?;
        canvas.draw_point((cx - x, cy - y))?;
        canvas.draw_point((cx - y, cy - x))?;
        canvas.draw_point((cx + y, cy - x))?;
        canvas.draw_point((cx + x, cy - y))?;
        
        y += 1;
        err += 1 + 2 * y;
        if 2 * (err - x) + 1 > 0 {
            x -= 1;
            err += 1 - 2 * x;
        }
    }
    
    Ok(())
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
    // Pas adaptatif basé sur le rayon pour un rendu fluide
    let step = (1.0 / radius).max(0.005).min(0.02);
    let half_thickness = thickness as f32 * 0.5;
    
    for i in 0..thickness {
        let r = radius - half_thickness + i as f32;
        let mut angle = start_angle;
        
        // Calculer le premier point
        let (sin_a, cos_a) = angle.sin_cos();
        let mut p1_x = center.x + r * cos_a;
        let mut p1_y = center.y + r * sin_a;
        
        while angle < end_angle {
            angle += step;
            let (sin_b, cos_b) = angle.sin_cos();
            let p2_x = center.x + r * cos_b;
            let p2_y = center.y + r * sin_b;
            
            canvas.draw_line(
                (p1_x as i32, p1_y as i32),
                (p2_x as i32, p2_y as i32)
            )?;
            
            p1_x = p2_x;
            p1_y = p2_y;
        }
    }
    Ok(())
}
