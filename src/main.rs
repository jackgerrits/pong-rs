use pancurses;
use std::time::{Duration, SystemTime};

struct Position(f32, f32);

struct Ball {
    position: Position,
    // 0 is directly up
    _direction_in_degrees: u32,
}

impl Ball {
    const SYMBOL: char = 'O';
}

struct Paddle {
    // Defines the top most position of this paddle.
    position: Position,
    // Defines the height in characters.
    height: i32,
}

impl Paddle {
    const SYMBOL: char = '#';
}

struct GameState {
    player: Paddle,
    opponent: Paddle,
    ball: Ball,
}

fn draw_ball(window: &pancurses::Window, ball: &Ball) {
    window.mvaddch(ball.position.1 as i32, ball.position.0 as i32, Ball::SYMBOL);
}

fn draw_paddle(window: &pancurses::Window, paddle: &Paddle) {
    for y_offset in 0..paddle.height {
        window.mvaddch(
            paddle.position.1 as i32 + y_offset,
            paddle.position.0 as i32,
            Paddle::SYMBOL,
        );
    }
}

fn update(state: &mut GameState, input_key: Option<pancurses::Input>, delta: &Duration) {
    let delta_scale = 1.0 * (delta.as_micros() as f32 / 1e6);
    state.ball.position.1 = state.ball.position.1 + delta_scale;

    match input_key {
        Some(pancurses::Input::KeyUp) => {
            state.player.position.1 = state.player.position.1 - 10.0 * delta_scale;
        }
        Some(pancurses::Input::KeyDown) => {
            state.player.position.1 = state.player.position.1 + 10.0 * delta_scale;
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
        position: Position(5.0, 4.0),
        _direction_in_degrees: 0,
    };
    let player = Paddle {
        position: Position(0.0, 0.0),
        height: 5,
    };
    let opponent = Paddle {
        position: Position(8.0, 0.0),
        height: 5,
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
