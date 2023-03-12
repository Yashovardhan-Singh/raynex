use raylib::prelude::*;

type Vec2 = Vector2;

#[derive(Clone, Copy)]
struct Line {
    pub p1 : Vec2,
    pub p2 : Vec2,
}

impl Line {
    pub const fn new(p1_x: f32, p1_y: f32, p2_x: f32, p2_y: f32) -> Line {
        Line { p1 : Vec2::new(p1_x, p1_y), p2 : Vec2::new(p2_x, p2_y), }
    }

    pub fn draw(&mut self, renderer: &mut RaylibDrawHandle) {
        renderer.draw_line_v(self.p1, self.p2, Color::WHITE);
    }
}

fn main() {
    let (mut rl, thread) = init().size(800, 450).title("Raynex").build();

    let mut lines : Vec<Line> = vec![];
    let mut directions : Vec<Vec2> = vec![];

    for i in 0..5 {
        lines.push(
            Line::new(
                get_random_value::<i32>(0, 800) as f32,
                get_random_value::<i32>(0, 450) as f32,
                get_random_value::<i32>(0, 800) as f32,
                get_random_value::<i32>(0, 450) as f32,
            )
        )
    }

    while !rl.window_should_close() {

        let player = rl.get_mouse_position();

        // Hilariously enough it was that easy
        for i in (0..360).step_by(10) {
            directions.push(Vec2 { x: f64::cos(i as f64 * DEG2RAD) as f32, y: f64::sin(i as f64 * DEG2RAD) as f32});
        }

        let mut renderer = rl.begin_drawing(&thread);
        renderer.clear_background(Color::BLACK);


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
            renderer.draw_line_v(player, point, Color::WHITE);
        }

        for i in &lines {
            i.clone().draw(&mut renderer);
        }
    }
}


/// Do the math to chek intersection between a ray and a line segment
/// https://en.wikipedia.org/wiki/Line–line_intersection
fn check_intersect(player: &Vec2, direction: &Vec2, line: &Line) -> Vec2 {

    let denominator = (line.p1.x - line.p2.x) * (player.y - player.y + direction.y) - (line.p1.y - line.p2.y) * (player.x - player.x + direction.x);

    if denominator == 0.0 { return *player; }

    let t = ((line.p1.x - player.x) * (player.y - player.y + direction.y) - (line.p1.y - player.y) * (player.x - player.x + direction.x)) / denominator;
    let u = ((line.p1.x - player.x) * (line.p1.y - line.p2.y) - (line.p1.y - player.y) * (line.p1.x - line.p2.x)) / denominator;

    if t > 0.0 && t < 1.0 && u > 0.0 {
        Vec2 {
            x: line.p1.x + t * (line.p2.x - line.p1.x),
            y: line.p1.y + t * (line.p2.y - line.p1.y),
        }
    } else {
        return *player;
    }
}