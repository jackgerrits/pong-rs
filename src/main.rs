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
    // 0 is directly up
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
        state.ball.velocity = state.ball.velocity.reflect(&mut state.player.normal);
    }

    if state.opponent.is_within(&state.ball.position) {
        state.ball.velocity = state.ball.velocity.reflect(&mut state.opponent.normal);
    }

    match input_key {
        Some(pancurses::Input::KeyUp) => {
            state.player.position.y = state.player.position.y - 10.0 * delta_scale;
        }
        Some(pancurses::Input::KeyDown) => {
            state.player.position.y = state.player.position.y + 10.0 * delta_scale;
        }
        _ => (),
    }
}

fn draw(window: &pancurses::Window, state: &GameState) {
    window.erase();
    draw_ball(&window, &state.ball);
    draw_paddle(&window, &state.player);
    draw_paddle(&window, &state.opponent);
    window.refresh();
}

fn main() {
    /* Start pancurses. */
    let window = pancurses::initscr();
    pancurses::cbreak();
    pancurses::noecho();
    window.clear();
    window.nodelay(true);
    window.keypad(true);

    let ball = Ball {
        position: Position { x: 7.0, y: 2.0 },
        velocity: Vector::new_normalized(45.0, 45.0),
    };

    let player = Paddle {
        position: Position { x: 0.0, y: 0.0 },
        height: 5,
        normal: Vector::new_normalized(1.0, 0.0),
    };
    let opponent = Paddle {
        position: Position { x: 8.0, y: 0.0 },
        height: 5,
        normal: Vector::new_normalized(-1.0, 0.0),
    };

    let mut state = GameState {
        player,
        opponent,
        ball,
    };

    let mut current_time = SystemTime::now();

    loop {
        let next_time = SystemTime::now();
        let difference = next_time
            .duration_since(current_time)
            .expect("SystemTime::duration_since failed");

        let ch = window.getch();

        match ch {
            Some(pancurses::Input::Character('q')) => break,
            _ => (),
        }

        update(&mut state, ch, &difference);
        draw(&window, &state);

        // Don't spin too much...
        std::thread::sleep(std::time::Duration::from_millis(75));

        current_time = next_time;
    }

    /* Terminate pancurses. */
    pancurses::endwin();
}
