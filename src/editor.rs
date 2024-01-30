use crate::Terminal;
use std::io::{self};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct  Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor{
    pub fn run(&mut self){
        enable_raw_mode().expect("Failed to enable raw mode");

        let _stdout = io::stdout();

        loop {
            if let Err(error) = self.refresh_screen(){
                die(&error)
            }
            if self.should_quit {
                disable_raw_mode().expect("Failed to disable raw mode");
                break;
            }
            if let Err(error) = self.process_keypress(){
                die(&error)
            }
        }

        
    }

    pub fn default() -> Self{
        Self{
            should_quit:false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let KeyEvent{code, modifiers, kind, .. } = Terminal::read_key_event()?;
        if kind==KeyEventKind::Press {
            match code {
                KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => self.should_quit = true,
                KeyCode::Char(c) => println!("Char: {:?} ({})\r", c as u8, c),
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => self.move_cursor(code),
                KeyCode::PageUp | KeyCode::PageDown | KeyCode::End | KeyCode::Home => self.move_cursor(code),
                _ => ()
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position {x: 0, y: 0});
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);

        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn move_cursor(&mut self, key_code: KeyCode) {
        let Position { mut y, mut x } = self.cursor_position;
        let window_size = self.terminal.size();
        let height = window_size.height.saturating_sub(1) as usize;
        let width = window_size.width.saturating_sub(1) as usize;
        match key_code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            },
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            },
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height,
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            _ => (),
        }
        self.cursor_position = Position {x, y}
    }

    fn draw_welcome_msg(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_msg();
            } else {
                println!("~\r");
            }
        }
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}

// pub fn run(&self){
//     enable_raw_mode().expect("Failed to enable raw mode");

//     let mut stdout = io::stdout();

//     loop {
//         match read() {
//             Ok(Event::Key(KeyEvent { code, modifiers, kind, .. })) => { // Using `..` to ignore other fields
//                 if kind==KeyEventKind::Press {
//                     match code {
//                         KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => break,
//                         KeyCode::Char(c) => {
//                             if c.is_control() {
//                                 println!("Control char: {:?}\r", c as u8);
//                             } else {
//                                 println!("Char: {:?} ({})\r", c as u8, c);
//                             }
//                         }
//                         _ => println!("Other keycode: {code:?}\r"),
//                     }
//                 }
//                 // Flush stdout after each print
//                 stdout.flush().unwrap();
//             }
//             Ok(_) => {}, // Ignore other types of events
//             Err(err) => die(&err),
//         }
//     }

//     disable_raw_mode().expect("Failed to disable raw mode");
// }