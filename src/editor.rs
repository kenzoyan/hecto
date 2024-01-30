use crate::Terminal;
use std::io::{self};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
};

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
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
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let KeyEvent{code, modifiers, kind, .. } = Terminal::read_key_event()?;
        if kind==KeyEventKind::Press {
            match code {
                KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => self.should_quit = true,
                KeyCode::Char(c) => println!("Char: {:?} ({})\r", c as u8, c),
                _ => ()
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);

        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
            println!("~\r");
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