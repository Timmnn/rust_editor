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

                let key = match key_event.code {
                    KeyCode::Char(c) => c.to_string(), // Convert character keys
                    //KeyCode::Enter => "Enter".to_string(),
                    //KeyCode::Backspace => "Backspace".to_string(),
                    //KeyCode::Esc => "Escape".to_string(),
                    //KeyCode::Left => "Left Arrow".to_string(),
                    //KeyCode::Right => "Right Arrow".to_string(),
                    //KeyCode::Up => "Up Arrow".to_string(),
                    //KeyCode::Down => "Down Arrow".to_string(),
                    //_ => format!("{:?}", key_event.code), // Fallback for other keys
                    _ => "".to_string(),
                };

                buffer = buffer + key.as_str();

                println!("{}", buffer);

                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode().unwrap();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
