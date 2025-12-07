use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::game::{World, Vector2D, Ball, TRAIL_LENGTH};

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
        // Dessiner la traînée avec dégradé de couleur
        draw_trail(canvas, ball)?;
        
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

/// Dessine la traînée d'une balle avec effet comète (dégradé de largeur et couleur)
fn draw_trail(canvas: &mut Canvas<Window>, ball: &Ball) -> Result<(), String> {
    let trail_len = ball.trail.len();
    if trail_len < 2 {
        return Ok(());
    }
    
    let base_r = ball.color.r as f32;
    let base_g = ball.color.g as f32;
    let base_b = ball.color.b as f32;
    let ball_diameter = ball.radius * 2.0;
    
    // Dessiner chaque segment de la traînée
    for i in 1..trail_len {
        let progress = i as f32 / trail_len as f32; // 0.0 (début) -> 1.0 (fin, près de la balle)
        
        let p1 = ball.trail[i - 1];
        let p2 = ball.trail[i];
        
        // Calculer la direction du segment
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let seg_len = (dx * dx + dy * dy).sqrt();
        
        if seg_len < 0.5 {
            continue; // Segment trop court
        }
        
        // Normale perpendiculaire au segment
        let nx = -dy / seg_len;
        let ny = dx / seg_len;
        
        // Largeur du segment (de 0 au début à diamètre complet à la fin)
        let width_start = (i - 1) as f32 / trail_len as f32 * ball_diameter;
        let width_end = progress * ball_diameter;
        
        // Dessiner des lignes parallèles pour remplir la largeur avec dégradé
        let max_offset = (width_end * 0.5) as i32;
        
        for offset in -max_offset..=max_offset {
            let offset_f = offset as f32;
            let offset_ratio = if max_offset > 0 { 
                1.0 - (offset_f.abs() / max_offset as f32) 
            } else { 
                1.0 
            };
            
            // Dégradé radial: plus lumineux au centre, plus sombre sur les bords
            let radial_fade = offset_ratio * offset_ratio; // Courbe quadratique pour effet plus doux
            
            // Dégradé longitudinal: de transparent à opaque
            let alpha = (progress * radial_fade * 220.0) as u8;
            let brightness = 0.2 + progress * 0.8 * radial_fade;
            
            let r = (base_r * brightness).min(255.0) as u8;
            let g = (base_g * brightness).min(255.0) as u8;
            let b = (base_b * brightness).min(255.0) as u8;
            
            canvas.set_draw_color(Color::RGBA(r, g, b, alpha));
            
            // Calculer la largeur à chaque extrémité du segment
            let w1_ratio = width_start / ball_diameter.max(1.0);
            let w2_ratio = width_end / ball_diameter.max(1.0);
            
            // Limiter l'offset selon la largeur à chaque point
            let offset1 = (offset_f * w1_ratio).clamp(-width_start * 0.5, width_start * 0.5);
            let offset2 = offset_f;
            
            let x1 = p1.x + nx * offset1;
            let y1 = p1.y + ny * offset1;
            let x2 = p2.x + nx * offset2;
            let y2 = p2.y + ny * offset2;
            
            canvas.draw_line(
                (x1 as i32, y1 as i32),
                (x2 as i32, y2 as i32)
            )?;
        }
    }
    
    // Connecter la traînée à la position actuelle avec un segment large
    if let Some(last) = ball.trail.last() {
        let dx = ball.position.x - last.x;
        let dy = ball.position.y - last.y;
        let seg_len = (dx * dx + dy * dy).sqrt();
        
        if seg_len > 0.5 {
            let nx = -dy / seg_len;
            let ny = dx / seg_len;
            let max_offset = (ball_diameter * 0.5) as i32;
            
            for offset in -max_offset..=max_offset {
                let offset_f = offset as f32;
                let offset_ratio = 1.0 - (offset_f.abs() / max_offset.max(1) as f32);
                let radial_fade = offset_ratio * offset_ratio;
                
                let alpha = (radial_fade * 240.0) as u8;
                let r = (base_r * radial_fade).min(255.0) as u8;
                let g = (base_g * radial_fade).min(255.0) as u8;
                let b = (base_b * radial_fade).min(255.0) as u8;
                
                canvas.set_draw_color(Color::RGBA(r, g, b, alpha));
                canvas.draw_line(
                    (last.x as i32 + (nx * offset_f) as i32, last.y as i32 + (ny * offset_f) as i32),
                    (ball.position.x as i32 + (nx * offset_f) as i32, ball.position.y as i32 + (ny * offset_f) as i32)
                )?;
            }
        }
    }
    
    Ok(())
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
