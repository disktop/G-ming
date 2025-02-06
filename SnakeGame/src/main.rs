use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal::{self, Clear, ClearType},
    execute, ExecutableCommand,
};
use std::collections::VecDeque;
use std::io::{self, Write};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: u16,
    y: u16,
}

struct Snake {
    body: VecDeque<Position>,
    direction: Direction,
}

impl Snake {
    fn new(start_x: u16, start_y: u16) -> Self {
        let mut body = VecDeque::new();
        body.push_front(Position { x: start_x, y: start_y });

        Snake {
            body,
            direction: Direction::Right,
        }
    }

    fn move_snake(&mut self) {
        let head = self.body.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Position {
                x: head.x,
                y: head.y.saturating_sub(1),
            },
            Direction::Down => Position {
                x: head.x,
                y: head.y.saturating_add(1),
            },
            Direction::Left => Position {
                x: head.x.saturating_sub(1),
                y: head.y,
            },
            Direction::Right => Position {
                x: head.x.saturating_add(1),
                y: head.y,
            },
        };
        self.body.push_front(new_head);
        self.body.pop_back();
    }

    fn grow(&mut self) {
        let tail = self.body.back().unwrap();
        self.body.push_back(*tail);
    }

    fn head(&self) -> Position {
        *self.body.front().unwrap()
    }
}

struct Game {
    snake: Snake,
    width: u16,
    height: u16,
    food: Position,
    score: u16
}

impl Game {
    fn new(width: u16, height: u16) -> Self {
        Game {
            snake: Snake::new(width / 2, height / 2),
            width,
            height,
            food: Position {
                x: width / 4,
                y: height / 4,
            },
            score: 0,
        }
    }

    fn check_collision(&self) -> bool {
        let head = self.snake.head();
        // Check for collision with walls
        if head.x == 0 || head.x == self.width - 1 || head.y == 0 || head.y == self.height - 1 {
            return true;
        }
        // Check for collision with itself
        for segment in self.snake.body.iter().skip(1) {
            if *segment == head {
                return true;
            }
        }
        false
    }

    fn check_food(&mut self) -> bool {
        let head = self.snake.head();
        if head == self.food {
            self.snake.grow();
            self.food = Position {
                x: rand::random::<u16>() % (self.width - 2) + 1,
                y: rand::random::<u16>() % (self.height - 2) + 1,
            };
            self.score += 1;
            return true;
        }
        false
    }

    fn render(&self) {
        // Replace this line:
        // terminal::clear(terminal::ClearType::All).unwrap();

        // With this line:
        execute!(io::stdout(), Clear(ClearType::All)).unwrap();  // Clears the entire terminal

        // Continue with the rest of the render logic...
        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position { x, y };
                if self.snake.body.contains(&position) {
                    print!("🔵");
                } else if position == self.food {
                    print!("🟢");
                } else if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 { 
                    print!("🔴");
                } else {
                    print!("⚪");
                }
            }
            println!();
        }
        println!("Level: {}", self.score);
        io::stdout().flush().unwrap();
    }
}

fn main() {
    // Create a variable to hold the clear action
    let clear_screen = |clear_type: ClearType| {
        execute!(io::stdout(), Clear(clear_type)).unwrap();
    };

    // Now you can call clear_screen instead of calling terminal::clear
    clear_screen(ClearType::All); // Clears the entire terminal
    
    // You can use different ClearTypes as needed
    // For example:
    // clear_screen(ClearType::FromCursorDown);  // Clears from cursor down

    // Rest of your game logic here...
    println!("Game has started!");

    // Add your game loop and other code below
    // This is just a placeholder for the rest of your game
    let width = 28;
    let height = 20;

    terminal::enable_raw_mode().unwrap();
    io::stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    let mut game = Game::new(width, height);

    let mut last_move = Instant::now();
    loop {
        if last_move.elapsed() >= Duration::from_millis(100) {
            game.snake.move_snake();
            if game.check_collision() {
                break;
            }
            if game.check_food() {
                // Food eaten
            }
            game.render();
            last_move = Instant::now();
        }

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(KeyEvent {
                code,
                modifiers: _,
                kind: _,
                state: _,
            }) = event::read().unwrap()
            {
                match code {
                    KeyCode::Esc => break,
                    KeyCode::Up => {
                        if game.snake.direction != Direction::Down {
                            game.snake.direction = Direction::Up;
                        }
                    }
                    KeyCode::Down => {
                        if game.snake.direction != Direction::Up {
                            game.snake.direction = Direction::Down;
                        }
                    }
                    KeyCode::Left => {
                        if game.snake.direction != Direction::Right {
                            game.snake.direction = Direction::Left;
                        }
                    }
                    KeyCode::Right => {
                        if game.snake.direction != Direction::Left {
                            game.snake.direction = Direction::Right;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    println!("Game Over!");
    println!("Scored {} points!", game.score*100);
    terminal::disable_raw_mode().unwrap(); // Correctly close the main function
}
