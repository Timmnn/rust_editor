use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::io::{stdout, Write};

fn main() {
    enable_raw_mode().unwrap();

    let mut stdout = stdout(); // Correctly calling `stdout()`

    stdout
        .execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))
        .unwrap();

    println!("Press 'q' to quit.");

    execute!(stdout, cursor::Hide).unwrap();
    let mut buffer = "".to_string();

    loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                clear_screen();
                //println!("Key pressed: {:?}", key_event);

                match key_event.code {
                    KeyCode::Char(c) => buffer = buffer + c.to_string().as_str(),
                    KeyCode::Enter => buffer = buffer + "\n\r",
                    KeyCode::Backspace => {
                        buffer.pop();
                    }
                    //KeyCode::Esc => "Escape".to_string(),
                    //KeyCode::Left => "Left Arrow".to_string(),
                    //KeyCode::Right => "Right Arrow".to_string(),
                    //KeyCode::Up => "Up Arrow".to_string(),
                    //KeyCode::Down => "Down Arrow".to_string(),
                    //_ => format!("{:?}", key_event.code), // Fallback for other keys
                    _ => {}
                };

                let cursor_pos: (u16, u16) = (3, 5);

                buffer = replace_char_at_position(
                    buffer.as_str(),
                    '█',
                    cursor_pos.0 as usize,
                    cursor_pos.1 as usize,
                );
                println!("{}", buffer);

                if key_event.code == KeyCode::Char('q') {
                    break;
                }

                stdout
                    .execute(cursor::MoveTo(cursor_pos.0, cursor_pos.1))
                    .unwrap();
            }
        }
    }

    disable_raw_mode().unwrap();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn replace_char_at_position(input: &str, replacement: char, row: usize, column: usize) -> String {
    let mut chars: Vec<char> = input.chars().collect();

    // Calculate the index from the row and column (assuming the string is in a grid-like format)
    let index = row * column + column;

    // Check if the index is within bounds of the string length
    if index < chars.len() {
        chars[index] = replacement;
    }

    chars.into_iter().collect()
}
