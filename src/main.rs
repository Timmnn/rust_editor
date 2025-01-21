use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Write};
use std::{error::Error, fs::File};

use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{
    style::{Attribute, SetAttribute}, // Add this import
};

fn main_loop() {
    let mut buf = String::from("\033[XXXm");
    let mut stdout = stdout();
    let mut count = 0;

    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.kind == KeyEventKind::Press {
                    count += 1;

                    match key_event.code {
                        KeyCode::Char(c) => buf.push(c),
                        KeyCode::Enter => buf.push('\n'),
                        KeyCode::Backspace => {
                            buf.pop();
                        }
                        _ => continue,
                    }

                    clear_screen(&mut stdout);
                    draw_screen(buf.as_str());
                    execute!(
                        stdout,
                        // Blue foreground
                        SetForegroundColor(Color::Blue),
                        // Red background
                        SetBackgroundColor(Color::Red),
                        // Print text
                        Print("Blue text on Red.".to_string()),
                        // Reset to default colors
                        ResetColor
                    );
                }
            }
        }
    }
}

fn clear_screen(stdout: &mut std::io::Stdout) {
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Show
    )
    .unwrap();
}

fn draw_screen(buf: &str) {
    print!("{}", buf);
}

fn main() {
    main_loop();
}
