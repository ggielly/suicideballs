use sdl2::pixels::Color;
use rand::Rng;
use std::ops::{Add, Sub, Mul, Div};

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
    pub config: Config,
    pub gravity_mode: GravityMode,
    // HUD Stats
    pub fps: u32,
    pub wall_collisions: u32,
    pub ball_collisions: u32,
}

pub fn initialize_world(config: Config) -> World {
    let mut balls = Vec::new();
    for _ in 0..10 { // Start with 10 balls
        balls.push(create_random_ball(&config));
    }
    World {
        balls,
        circle_angle: 0.0,
        bounciness: 0.9, // Initial bounciness
        config,
        gravity_mode: GravityMode::Vertical,
        fps: 0,
        wall_collisions: 0,
        ball_collisions: 0,
    }
}

fn create_random_ball(config: &Config) -> Ball {
    let mut rng = rand::thread_rng();
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

    world.circle_angle += world.config.circle_rotation_speed;

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

        ball.velocity = ball.velocity + ball.acceleration;
        ball.old_position = ball.position;
        ball.position = ball.position + ball.velocity;
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

            if is_in_gap {
                // Ball is in the gap, do nothing special for now
            } else {
                world.wall_collisions += 1;
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


    // Spawn new balls
    for _ in 0..fallen_balls_count { // Changed from * 2 to * 1
        if world.balls.len() < world.config.max_balls {
            world.balls.push(create_random_ball(&world.config));
        }
    }

    // Ball-to-ball collision
    let balls = &mut world.balls;
    for i in 0..balls.len() {
        let (left, right) = balls.split_at_mut(i + 1);
        let ball_a = &mut left[i];
        for ball_b in right {
            let axis = ball_a.position - ball_b.position;
            let dist_sq = axis.length_squared();
            let total_radius = ball_a.radius + ball_b.radius;

            if dist_sq < total_radius * total_radius && dist_sq > 0.0 {
                world.ball_collisions += 1;
                let distance = dist_sq.sqrt();
                let normal = axis / distance;
                let overlap = 0.5 * (total_radius - distance);

                ball_a.position = ball_a.position + normal * overlap;
                ball_b.position = ball_b.position - normal * overlap;

                let (v1, v2) = (ball_a.velocity, ball_b.velocity);
                let (m1, m2) = (ball_a.radius * ball_a.radius, ball_b.radius * ball_b.radius);

                let v1_dot_normal = v1.dot(normal);
                let v2_dot_normal = v2.dot(normal);

                let v1_prime_dot = (v1_dot_normal * (m1 - m2) + 2.0 * m2 * v2_dot_normal) / (m1 + m2);
                let v2_prime_dot = (v2_dot_normal * (m2 - m1) + 2.0 * m1 * v1_dot_normal) / (m1 + m2);

                ball_a.velocity = (ball_a.velocity - normal * (v1_dot_normal - v1_prime_dot)) * world.bounciness;
                ball_b.velocity = (ball_b.velocity - normal * (v2_dot_normal - v2_prime_dot)) * world.bounciness;
            }
        }
    }
}
