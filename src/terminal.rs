use std::io::{self, Write};
use crossterm::{
    execute,
    terminal::{size,Clear,ClearType},
    event::{read, Event, KeyEvent},
    cursor::{self,MoveTo},
};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: std::io::Stdout,
}

impl Terminal {
    pub fn default() -> Result<Self,std::io::Error> {
        let size = size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: io::stdout(),
        })
    }

    pub fn size(&self) ->&Size {
        &self.size
    }

    pub fn clear_screen() {
        execute!(io::stdout(), Clear(ClearType::All)).expect("Failed to clear the screen");
    }

    pub fn cursor_position(x:u16, y:u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        execute!(io::stdout(), MoveTo(x,y)).expect("Failed to move the cursor");
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key_event() -> Result<KeyEvent, std::io::Error> {
        loop {
            if let Event::Key(key_event) = read().unwrap() {
                return Ok(key_event);
            }
        }
    }

    pub fn cursor_hide() {
        execute!(io::stdout(), cursor::Hide).expect("Failed to hide the cursor");
    }

    pub fn cursor_show() {
        execute!(io::stdout(), cursor::Show).expect("Failed to show the cursor");
    }

    pub fn clear_current_line() {
        execute!(io::stdout(), Clear(ClearType::CurrentLine)).expect("Failed to clear the screen");

    }
}
