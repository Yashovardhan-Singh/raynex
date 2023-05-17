use raylib::prelude::*;

type Vec2 = Vector2;
const FOV: i32 = 40;
const RW: i32 = 800;
const RH: i32 = 450;

#[derive(Clone, Copy)]
struct Line {
    pub p1: Vec2,
    pub p2: Vec2,
}

/// Line struct
/// Used to create boundaries which can bew viewed in raycasts
impl Line {
    pub const fn new(p1_x: f32, p1_y: f32, p2_x: f32, p2_y: f32) -> Line {
        Line {
            p1: Vec2::new(p1_x, p1_y),
            p2: Vec2::new(p2_x, p2_y),
        }
    }

    pub fn draw(&mut self, renderer: &mut RaylibDrawHandle) {
        renderer.draw_line_v(self.p1, self.p2, Color::WHITE);
    }
}

fn main() {
    let (mut rl, thread) = init().size(RW, RH).title("Raynex").build();

    let mut lines: Vec<Line> = vec![];

    let mut directions: Vec<Vec2> = vec![];
    let mut points: Vec<Vec2> = vec![];
    let mut buffer: Vec<f32> = vec![];

    let mut player = Vec2::new(RW as f32 / 2.0, RH as f32 / 2.0);

    for _i in 0..5 {
        lines.push(Line::new(
            get_random_value::<i32>(0, RW) as f32,
            get_random_value::<i32>(0, RH) as f32,
            get_random_value::<i32>(0, RW) as f32,
            get_random_value::<i32>(0, RH) as f32,
        ))
    }

    let mut view_2D = true;
    let mut angle = 0.0;

    for i in 0..FOV {
        directions.push(Vec2 {
            x: f64::cos(i as f64 * DEG2RAD) as f32,
            y: f64::sin(i as f64 * DEG2RAD) as f32,
        });
    }

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_V) {
            view_2D = !view_2D
        }

        buffer.clear();
        points.clear();

        for i in &directions {
            let mut point = Vec2::new(f32::INFINITY, f32::INFINITY);
            let mut distance = f32::MAX;
            for j in &lines {
                let target = check_intersect(&player, i, j);
                if target != player {
                    if target.distance_to(player) < distance {
                        distance = target.distance_to(player);
                        point = target;
                    }
                }
            }

            points.push(point);
            buffer.push(distance * f32::cos(f32::atan(i.y / i.x) - (angle * DEG2RAD as f32)));
            // Non
        }

        if rl.is_key_down(KeyboardKey::KEY_A) {
            angle -= 0.1;
            for i in 0..directions.len() {
                directions[i].x = f64::cos((i as f32 + angle) as f64 * DEG2RAD) as f32;
                directions[i].y = f64::sin((i as f32 + angle) as f64 * DEG2RAD) as f32;
            }
        }

        if rl.is_key_down(KeyboardKey::KEY_W) {
            player.x -= f32::cos((20.0 + angle) * DEG2RAD as f32) * 0.1;
            player.y -= f32::sin((20.0 + angle) * DEG2RAD as f32) * 0.1;
        }

        if rl.is_key_down(KeyboardKey::KEY_S) {
            player.x += f32::cos((20.0 + angle) * DEG2RAD as f32) * 0.1;
            player.y += f32::sin((20.0 + angle) * DEG2RAD as f32) * 0.1;
        }

        if rl.is_key_down(KeyboardKey::KEY_D) {
            angle += 0.1;
            for i in 0..directions.len() {
                directions[i].x = f64::cos((i as f32 + angle) as f64 * DEG2RAD) as f32;
                directions[i].y = f64::sin((i as f32 + angle) as f64 * DEG2RAD) as f32;
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_H) {
            dbg!(&buffer);
        }

        let mut renderer = rl.begin_drawing(&thread);
        renderer.clear_background(Color::BLACK);

        if view_2D {
            for i in &points {
                renderer.draw_line_v(player, i, Color::WHITE);
            }
            for i in &lines {
                i.clone().draw(&mut renderer);
            }
        }

        if !view_2D {
            for i in 0..buffer.len() {
                let h = change_range(buffer[i], 0.0, RW as f32, RH as f32, 0.0) as i32;
                renderer.draw_rectangle(
                    (i as i32 * (RW / FOV)) - ((RW / FOV) / 2),
                    225 - (h / 2),
                    RW / FOV,
                    h,
                    Color::new(
                        255,
                        255,
                        255,
                        change_range(buffer[i] * buffer[i], 0.0, (RW * RW) as f32, 255.0, 0.0)
                            as i32 as i8 as u8,
                    ),
                )
            }
        }
    }
}

/// Do the math to check intersection between a ray and a line segment
/// https://en.wikipedia.org/wiki/Line–line_intersection
fn check_intersect(player: &Vec2, direction: &Vec2, line: &Line) -> Vec2 {
    let denominator = (line.p1.x - line.p2.x) * (player.y - player.y + direction.y)
        - (line.p1.y - line.p2.y) * (player.x - player.x + direction.x);

    if denominator == 0.0 {
        return *player;
    }

    let t = ((line.p1.x - player.x) * (player.y - player.y + direction.y)
        - (line.p1.y - player.y) * (player.x - player.x + direction.x))
        / denominator;
    let u = ((line.p1.x - player.x) * (line.p1.y - line.p2.y)
        - (line.p1.y - player.y) * (line.p1.x - line.p2.x))
        / denominator;

    if t > 0.0 && t < 1.0 && u > 0.0 {
        Vec2 {
            x: line.p1.x + t * (line.p2.x - line.p1.x),
            y: line.p1.y + t * (line.p2.y - line.p1.y),
        }
    } else {
        return *player;
    }
}

/// Remap Range of a number
/// https://stackoverflow.com/questions/44338698/p5-js-map-function-in-python
fn change_range(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2
}
