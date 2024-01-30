use std::io::{self};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
};

pub struct Editor {
    should_quit: bool,
}

impl Editor{
    pub fn run(&mut self){
        enable_raw_mode().expect("Failed to enable raw mode");

        let _stdout = io::stdout();

        loop {
            if let Err(error) = self.process_keypress(){
                die(&error)
            }
            if self.should_quit {
                disable_raw_mode().expect("Failed to disable raw mode");
                break;
            }
        }

        
    }

    pub fn default() -> Self{
        Self{should_quit:false}
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let KeyEvent{code, modifiers, kind, .. } = read_key_event()?;
        if kind==KeyEventKind::Press {
            match code {
                KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => self.should_quit = true,
                KeyCode::Char(c) => println!("Char: {:?} ({})\r", c as u8, c),
                _ => ()
            }
        }
        Ok(())
    }
}

fn die(e: &std::io::Error) {
    panic!("{}", e);
}

fn read_key_event() -> Result<KeyEvent, std::io::Error> {
    loop {
        if let Event::Key(key_event) = read().unwrap() {
            return Ok(key_event);
        }
    }
}