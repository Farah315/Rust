use std::io::{stdout};
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use crossterm::{
    cursor, event::{self, Event, KeyCode}, execute, style::{self, Color, StyledContent, Stylize}, terminal,
};
use rand::Rng;

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

fn draw_text<S: Into<String>>(text: S, color: Color) -> StyledContent<String> {
    style::style(text.into()).with(color)
}

fn main() {
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide).unwrap();
    terminal::enable_raw_mode().unwrap();

    let (width, height) = (30, 20);
    let mut snake = VecDeque::from([Point::new(width / 2, height / 2)]);
    let mut direction = Direction::Right;
    let mut food = Point::new(5, 5);
    let mut score = 0;

    let mut last_update = Instant::now();
    let mut tick_rate = Duration::from_millis(200); // Initial speed of the game

    // Display start screen
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    println!("{}", draw_text("Welcome to Snake Game!", Color::Cyan));
    println!("{}", draw_text("Use Arrow Keys to move", Color::Green));
    println!("{}", draw_text("Press 'q' to quit", Color::Yellow));
    println!("\n{}", draw_text("Press any key to start...", Color::Magenta));
    event::read().unwrap(); // Wait for user input to start

    'game_loop: loop {
        // Draw frame
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    print!("{}", draw_text("#", Color::White)); // Walls
                } else if snake.contains(&Point::new(x, y)) {
                    print!("{}", draw_text("O", Color::Green)); // Snake body
                } else if food.x == x && food.y == y {
                    print!("{}", draw_text("F", Color::Red)); // Food
                } else {
                    print!(" "); // Empty space
                }
            }
            println!();
        }

        println!("{}", draw_text(format!("Score: {}", score), Color::Cyan));

        // Handle input
        if event::poll(tick_rate - last_update.elapsed()).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Up if direction != Direction::Down => direction = Direction::Up,
                    KeyCode::Down if direction != Direction::Up => direction = Direction::Down,
                    KeyCode::Left if direction != Direction::Right => direction = Direction::Left,
                    KeyCode::Right if direction != Direction::Left => direction = Direction::Right,
                    KeyCode::Char('q') => break 'game_loop, // Quit
                    _ => {}
                }
            }
        }

        // Update game state
        if last_update.elapsed() >= tick_rate {
            last_update = Instant::now();

            let head = *snake.front().unwrap();
            let new_head = match direction {
                Direction::Up => Point::new(head.x, head.y.saturating_sub(1)),
                Direction::Down => Point::new(head.x, head.y + 1),
                Direction::Left => Point::new(head.x.saturating_sub(1), head.y),
                Direction::Right => Point::new(head.x + 1, head.y),
            };

            // Check for collisions
            if new_head.x == 0
                || new_head.x == width - 1
                || new_head.y == 0
                || new_head.y == height - 1
                || snake.contains(&new_head)
            {
                break 'game_loop; // Game over
            }

            snake.push_front(new_head);

            if new_head.x == food.x && new_head.y == food.y {
                score += 1;
                food = Point::new(
                    rand::thread_rng().gen_range(1..width - 1),
                    rand::thread_rng().gen_range(1..height - 1),
                );
                tick_rate = tick_rate.saturating_sub(Duration::from_millis(10)); // Safely increase speed
            } else {
                snake.pop_back();
            }
        }
    }

    // End screen
    terminal::disable_raw_mode().unwrap();
    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show
    )
    .unwrap();
    println!("{}", draw_text("Game Over!", Color::Red));
    println!("{}", draw_text(format!("Final Score: {}", score), Color::Cyan));
}
