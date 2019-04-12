use pancurses;
use std::time::{Duration, SystemTime};

mod math;
use math::Vector;

pub struct Position {
    pub x: f32,
    pub y: f32,
}

struct Ball {
    position: Position,
    velocity: Vector,
}

impl Ball {
    const SYMBOL: char = 'O';
}

struct Paddle {
    // Defines the top most position of this paddle.
    position: Position,
    // Defines the height in characters.
    height: i32,
    normal: Vector,
}

impl Paddle {
    const SYMBOL: char = '#';

    fn is_within(&self, positon: &Position) -> bool {
        self.position.x as i32 == positon.x as i32
            && self.position.y as i32 <= positon.y as i32
            && self.position.y as i32 + self.height > positon.y as i32
    }
}

struct GameState {
    player: Paddle,
    opponent: Paddle,
    ball: Ball,
    max_x: i32,
    max_y: i32,
    player_score: i32,
    opponent_score: i32,
}

fn draw_ball(window: &pancurses::Window, ball: &Ball) {
    window.mvaddch(ball.position.y as i32, ball.position.x as i32, Ball::SYMBOL);
}

fn draw_paddle(window: &pancurses::Window, paddle: &Paddle) {
    for y_offset in 0..paddle.height {
        window.mvaddch(
            paddle.position.y as i32 + y_offset,
            paddle.position.x as i32,
            Paddle::SYMBOL,
        );
    }
}

fn update(state: &mut GameState, input_key: Option<pancurses::Input>, delta: &Duration) {
    let delta_scale = 1.0 * (delta.as_micros() as f32 / 1e6);

    state.ball.position.x += state.ball.velocity.x * delta_scale;
    state.ball.position.y += state.ball.velocity.y * delta_scale;

    // Ball collided with player.
    if state.player.is_within(&state.ball.position) {
        state.ball.velocity = state.ball.velocity.reflect(&state.player.normal);
    } else if state.opponent.is_within(&state.ball.position) {
        // Ball collided with opponent
        state.ball.velocity = state.ball.velocity.reflect(&state.opponent.normal);
    } else if state.ball.position.y as i32 <= 0 {
        state.ball.velocity = state
            .ball
            .velocity
            .reflect(&Vector::new_normalized(0.0, 1.0));
    } else if state.ball.position.y as i32 >= state.max_y {
        state.ball.velocity = state
            .ball
            .velocity
            .reflect(&Vector::new_normalized(0.0, -1.0));
    } else if state.ball.position.x as i32 <= 0 {
        state.opponent_score += 1;
        state.ball.position.x = (state.max_x / 2) as f32;
        state.ball.position.y = (state.max_y / 2) as f32;
        state.ball.velocity.x *= -1.0;
    } else if state.ball.position.x as i32 >= state.max_x {
        state.player_score += 1;
        state.ball.position.x = (state.max_x / 2) as f32;
        state.ball.position.y = (state.max_y / 2) as f32;
        state.ball.velocity.x *= -1.0;
    }

    match input_key {
        Some(pancurses::Input::KeyUp) => state.opponent.position.y -= 10.0 * delta_scale,
        Some(pancurses::Input::KeyDown) => {
            state.opponent.position.y += 10.0 * delta_scale;
        }
        Some(pancurses::Input::Character('a')) => state.player.position.y += 10.0 * delta_scale,
        Some(pancurses::Input::Character('q')) => state.player.position.y -= 10.0 * delta_scale,
        _ => (),
    }
}

fn draw(window: &pancurses::Window, state: &GameState) {
    window.erase();
    draw_ball(&window, &state.ball);
    draw_paddle(&window, &state.player);
    draw_paddle(&window, &state.opponent);
    window.mvaddstr(state.max_y - 2, 0, format!("Left: {}", state.player_score));
    window.mvaddstr(
        state.max_y - 1,
        0,
        format!("Right: {}", state.opponent_score),
    );
    window.refresh();
}

fn main() {
    let window = pancurses::initscr();
    pancurses::cbreak();
    pancurses::noecho();
    window.clear();
    window.nodelay(true);
    window.keypad(true);

    let max_x = window.get_max_x();
    let max_y = window.get_max_y();

    let ball = Ball {
        position: Position { x: 7.0, y: 2.0 },
        velocity: Vector { x: 15.0, y: 10.0 },
    };

    let player = Paddle {
        position: Position {
            x: 0.0,
            y: (max_y / 3) as f32,
        },
        height: max_y / 3,
        normal: Vector::new_normalized(1.0, 0.0),
    };
    let opponent = Paddle {
        position: Position {
            x: (max_x - 1) as f32,
            y: (max_y / 3) as f32,
        },
        height: max_y / 3,
        normal: Vector::new_normalized(-1.0, 0.0),
    };

    let mut state = GameState {
        player,
        opponent,
        ball,
        max_x,
        max_y,
        player_score: 0,
        opponent_score: 0,
    };

    let mut current_time = SystemTime::now();

    loop {
        let next_time = SystemTime::now();
        let difference = next_time.duration_since(current_time).unwrap();

        let ch = window.getch();

        // Quit if 'q' is pressed
        if let Some(pancurses::Input::Character('x')) = ch {
            break;
        }

        update(&mut state, ch, &difference);
        draw(&window, &state);

        // Don't spin too much...
        std::thread::sleep(std::time::Duration::from_millis(75));

        current_time = next_time;
    }

    pancurses::endwin();
}
