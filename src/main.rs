
use ncurses;
use std::time::{SystemTime, Duration};

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

fn draw_ball(ball: &Ball) {
  ncurses::mvaddch(
    ball.position.1 as i32,
    ball.position.0 as i32,
    Ball::SYMBOL as ncurses::chtype,
  );
}

fn draw_paddle(paddle: &Paddle) {
  for y_offset in 0..paddle.height {
    ncurses::mvaddch(
      paddle.position.1 as i32 + y_offset,
      paddle.position.0 as i32,
      Paddle::SYMBOL as ncurses::chtype,
    );
  }
}

fn update(state: &mut GameState, input_key: i32, delta: &Duration) {
  let delta_scale = 1.0 * (delta.as_millis() as f32/1000.0);
  state.ball.position.1 = state.ball.position.1 + delta_scale;

  if input_key == ncurses::KEY_DOWN {
    state.player.position.1 = state.player.position.1 + 10.0* delta_scale;
  } else if input_key == ncurses::KEY_UP {
    state.player.position.1 = state.player.position.1 - 10.0* delta_scale;
  }
}

fn draw(state: &GameState) {
  ncurses::erase();
  draw_ball(&state.ball);
  draw_paddle(&state.player);
  draw_paddle(&state.opponent);
  ncurses::refresh();
}

fn main() {
  /* Start ncurses. */
  ncurses::initscr();
  ncurses::cbreak();
  ncurses::noecho();
  ncurses::clear();
  ncurses::nodelay(ncurses::constants::stdscr(), true);
  ncurses::keypad(ncurses::constants::stdscr(), true);

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

    let ch = ncurses::getch();
    
    // Quit with 'q'
    if ch == 113 {
      break;
    }

    update(&mut state, ch, &difference);
    draw(&state);

    // Don't spin too much...
    std::thread::sleep(std::time::Duration::from_millis(75));

    current_time = next_time;
  }

  /* Terminate ncurses. */
  ncurses::endwin();
}
