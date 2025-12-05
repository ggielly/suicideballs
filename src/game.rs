use sdl2::pixels::Color;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign};

// --- Gravity ---
#[derive(Debug)]
pub enum GravityMode {
    Centripetal,
    Vertical,
}

// --- Config ---
pub struct Config {
    pub screen_width: u32,
    pub sim_width: u32,
    pub screen_height: u32,
    pub max_balls: usize,
    pub centripetal_gravity: f32,
    pub vertical_gravity: f32,
    pub circle_radius: f32,
    pub circle_thickness: f32,
    pub circle_rotation_speed: f32,
    pub circle_gap_angle: f32,
    pub ball_radius: f32,
    pub max_velocity: f32, // Limite de vitesse pour éviter le tunneling
    pub grid_cell_size: f32, // Taille des cellules de la grille spatiale
}

impl Default for Config {
    fn default() -> Self {
        Self {
            screen_width: 800,
            sim_width: 600,
            screen_height: 800,
            max_balls: 50,
            centripetal_gravity: 0.05,
            vertical_gravity: 0.1,
            circle_radius: 250.0,
            circle_thickness: 20.0,
            circle_rotation_speed: 0.00665,
            circle_gap_angle: std::f32::consts::FRAC_PI_4,
            ball_radius: 15.0,
            max_velocity: 15.0, // Limite de vitesse
            grid_cell_size: 40.0, // ~2x le diamètre des balles
        }
    }
}


#[derive(Clone, Copy, Default)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Add for Vector2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Vector2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<f32> for Vector2D {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self { x: self.x * scalar, y: self.y * scalar }
    }
}

impl Div<f32> for Vector2D {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self { x: self.x / scalar, y: self.y / scalar }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<f32> for Vector2D {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Vector2D {
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            self / len
        } else {
            Self { x: 1.0, y: 0.0 }
        }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

pub struct Ball {
    pub position: Vector2D,
    pub old_position: Vector2D, // For trails
    pub velocity: Vector2D,
    pub acceleration: Vector2D,
    pub radius: f32,
    pub color: Color,
}

pub struct World {
    pub balls: Vec<Ball>,
    pub circle_angle: f32,
    pub bounciness: f32,
    pub friction: f32,
    pub config: Config,
    pub gravity_mode: GravityMode,
    pub balls_to_spawn: u32,
    pub rng: ThreadRng, // RNG réutilisable
    // HUD Stats
    pub fps: u32,
    pub wall_collisions: u32,
    pub ball_collisions: u32,
    pub total_wall_collisions: u64,
    pub total_ball_collisions: u64,
}

pub fn initialize_world(config: Config) -> World {
    let mut rng = rand::thread_rng();
    let balls = vec![create_random_ball_with_rng(&config, &mut rng)];
    World {
        balls,
        circle_angle: 0.0,
        bounciness: 0.9,
        friction: 0.995, // Légère friction pour stabiliser
        config,
        gravity_mode: GravityMode::Vertical,
        balls_to_spawn: 2,
        rng,
        fps: 0,
        wall_collisions: 0,
        ball_collisions: 0,
        total_wall_collisions: 0,
        total_ball_collisions: 0,
    }
}

fn create_random_ball_with_rng(config: &Config, rng: &mut ThreadRng) -> Ball {
    let radius = config.ball_radius;
    
    // Spawn in a circle
    let angle = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
    let dist = rng.gen_range(0.0..config.circle_radius - radius);
    let circle_center = Vector2D { x: config.sim_width as f32 / 2.0, y: config.screen_height as f32 / 2.0 };

    let position = Vector2D {
        x: circle_center.x + angle.cos() * dist,
        y: circle_center.y + angle.sin() * dist,
    };

    Ball {
        position,
        old_position: position,
        velocity: Vector2D { x: rng.gen_range(-2.0..2.0), y: rng.gen_range(-2.0..2.0) },
        acceleration: Vector2D::default(),
        radius,
        color: Color::RGB(rng.gen_range(100..255), rng.gen_range(100..255), rng.gen_range(100..255)),
    }
}

pub fn update_world(world: &mut World) {
    // Reset counters for the new frame
    world.wall_collisions = 0;
    world.ball_collisions = 0;

    // Garder l'angle borné pour éviter les problèmes de précision
    world.circle_angle = (world.circle_angle + world.config.circle_rotation_speed) % (2.0 * std::f32::consts::PI);

    let circle_center = Vector2D { x: world.config.sim_width as f32 / 2.0, y: world.config.screen_height as f32 / 2.0 };

    // Ball physics and collision
    for ball in &mut world.balls {
        match world.gravity_mode {
            GravityMode::Centripetal => {
                let to_center = circle_center - ball.position;
                ball.acceleration = ball.acceleration + to_center.normalized() * world.config.centripetal_gravity;
            },
            GravityMode::Vertical => {
                ball.acceleration = ball.acceleration + Vector2D { x: 0.0, y: world.config.vertical_gravity };
            }
        }

        ball.velocity += ball.acceleration;
        ball.velocity *= world.friction; // Appliquer friction
        
        // Limiter la vitesse pour éviter le tunneling
        let speed_sq = ball.velocity.length_squared();
        let max_vel = world.config.max_velocity;
        if speed_sq > max_vel * max_vel {
            ball.velocity *= max_vel / speed_sq.sqrt();
        }
        
        ball.old_position = ball.position;
        ball.position += ball.velocity;
        ball.acceleration = Vector2D::default();

        let to_ball = ball.position - circle_center;
        let dist_sq = to_ball.length_squared();

        if dist_sq > (world.config.circle_radius - ball.radius).powi(2) {
            let ball_angle = to_ball.y.atan2(to_ball.x);
            let gap_start = world.circle_angle - world.config.circle_gap_angle / 2.0;
            let gap_end = world.circle_angle + world.config.circle_gap_angle / 2.0;

            let norm_ball_angle = (ball_angle + 2.0 * std::f32::consts::PI) % (2.0 * std::f32::consts::PI);
            let norm_gap_start = (gap_start + 2.0 * std::f32::consts::PI) % (2.0 * std::f32::consts::PI);
            let norm_gap_end = (gap_end + 2.0 * std::f32::consts::PI) % (2.0 * std::f32::consts::PI);

            let is_in_gap = if norm_gap_start < norm_gap_end {
                norm_ball_angle > norm_gap_start && norm_ball_angle < norm_gap_end
            } else {
                norm_ball_angle > norm_gap_start || norm_ball_angle < norm_gap_end
            };

            if !is_in_gap {
                world.wall_collisions += 1;
                world.total_wall_collisions += 1;
                let normal = to_ball.normalized();
                ball.position = circle_center + normal * (world.config.circle_radius - ball.radius);
                let dot = ball.velocity.dot(normal);
                ball.velocity = ball.velocity - normal * (2.0 * dot) * world.bounciness;
            }
        }
    }

    // Remove balls that are far outside the circle (fallen)
    let initial_ball_count = world.balls.len();
    world.balls.retain(|ball| {
        let dist_sq = (ball.position - circle_center).length_squared();
        dist_sq < (world.config.circle_radius + 50.0).powi(2) // Remove if 50px away from circle
    });
    let fallen_balls_count = initial_ball_count - world.balls.len();


    // Spawn new balls - chaque balle tombée génère balls_to_spawn nouvelles balles
    let balls_to_add = fallen_balls_count * world.balls_to_spawn as usize;
    for _ in 0..balls_to_add {
        if world.balls.len() < world.config.max_balls {
            let new_ball = create_random_ball_with_rng(&world.config, &mut world.rng);
            world.balls.push(new_ball);
        }
    }

    // Ball-to-ball collision avec grille spatiale
    let cell_size = world.config.grid_cell_size;
    let grid_width = (world.config.sim_width as f32 / cell_size).ceil() as usize + 1;
    let grid_height = (world.config.screen_height as f32 / cell_size).ceil() as usize + 1;
    let grid_size = grid_width * grid_height;
    
    // Construire la grille
    let mut grid: Vec<Vec<usize>> = vec![Vec::new(); grid_size];
    for (i, ball) in world.balls.iter().enumerate() {
        let cx = ((ball.position.x / cell_size) as usize).min(grid_width - 1);
        let cy = ((ball.position.y / cell_size) as usize).min(grid_height - 1);
        grid[cy * grid_width + cx].push(i);
    }
    
    // Vérifier les collisions seulement entre balles dans les cellules voisines
    let bounciness = world.bounciness;
    let restitution = bounciness.sqrt();
    
    for cy in 0..grid_height {
        for cx in 0..grid_width {
            let cell_idx = cy * grid_width + cx;
            let cell_balls = &grid[cell_idx];
            
            // Collisions dans la même cellule
            for i in 0..cell_balls.len() {
                for j in (i + 1)..cell_balls.len() {
                    let idx_a = cell_balls[i];
                    let idx_b = cell_balls[j];
                    if let Some(collision) = check_collision(&world.balls[idx_a], &world.balls[idx_b]) {
                        world.ball_collisions += 1;
                        world.total_ball_collisions += 1;
                        resolve_collision(&mut world.balls, idx_a, idx_b, collision, restitution);
                    }
                }
            }
            
            // Collisions avec cellules voisines (droite, bas, bas-droite, bas-gauche)
            let neighbors = [
                (cx + 1, cy),
                (cx, cy + 1),
                (cx + 1, cy + 1),
                (cx.wrapping_sub(1), cy + 1),
            ];
            
            for (nx, ny) in neighbors {
                if nx < grid_width && ny < grid_height {
                    let neighbor_idx = ny * grid_width + nx;
                    for &idx_a in cell_balls {
                        for &idx_b in &grid[neighbor_idx] {
                            if let Some(collision) = check_collision(&world.balls[idx_a], &world.balls[idx_b]) {
                                world.ball_collisions += 1;
                                world.total_ball_collisions += 1;
                                resolve_collision(&mut world.balls, idx_a, idx_b, collision, restitution);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Données de collision
struct CollisionData {
    normal: Vector2D,
    overlap: f32,
}

/// Vérifie si deux balles sont en collision
#[inline]
fn check_collision(ball_a: &Ball, ball_b: &Ball) -> Option<CollisionData> {
    let axis = ball_a.position - ball_b.position;
    let dist_sq = axis.length_squared();
    let total_radius = ball_a.radius + ball_b.radius;
    
    if dist_sq < total_radius * total_radius && dist_sq > 0.0 {
        let distance = dist_sq.sqrt();
        Some(CollisionData {
            normal: axis / distance,
            overlap: 0.5 * (total_radius - distance),
        })
    } else {
        None
    }
}

/// Résout une collision entre deux balles
#[inline]
fn resolve_collision(balls: &mut [Ball], idx_a: usize, idx_b: usize, collision: CollisionData, restitution: f32) {
    let CollisionData { normal, overlap } = collision;
    
    // Séparer les balles
    balls[idx_a].position += normal * overlap;
    balls[idx_b].position -= normal * overlap;
    
    let v1 = balls[idx_a].velocity;
    let v2 = balls[idx_b].velocity;
    let m1 = balls[idx_a].radius * balls[idx_a].radius;
    let m2 = balls[idx_b].radius * balls[idx_b].radius;
    
    let v1_dot_normal = v1.dot(normal);
    let v2_dot_normal = v2.dot(normal);
    
    let v1_prime_dot = (v1_dot_normal * (m1 - m2) + 2.0 * m2 * v2_dot_normal) / (m1 + m2);
    let v2_prime_dot = (v2_dot_normal * (m2 - m1) + 2.0 * m1 * v1_dot_normal) / (m1 + m2);
    
    balls[idx_a].velocity = (balls[idx_a].velocity - normal * (v1_dot_normal - v1_prime_dot)) * restitution;
    balls[idx_b].velocity = (balls[idx_b].velocity - normal * (v2_dot_normal - v2_prime_dot)) * restitution;
}

// Fonctions pour modifier le nombre de balles à générer
pub fn increase_balls_to_spawn(world: &mut World) {
    world.balls_to_spawn += 1;
}

pub fn decrease_balls_to_spawn(world: &mut World) {
    if world.balls_to_spawn > 1 {
        world.balls_to_spawn -= 1;
    }
}
