use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};

use std::{error, io};

pub trait MenuItem {
    fn menuitem_txt(&self) -> &str;
    fn execute(&self);
}

pub struct ListDepartments();

impl ListDepartments {
    fn new() -> Self {
        Self()
    }
}

impl MenuItem for ListDepartments {
    fn menuitem_txt(&self) -> &str {
        "List department hierarchy"
    }

    fn execute(&self) {
        println!("executing ListDepartments");
    }
}

fn display_menu<T: MenuItem>(
    menu_items: &[T],
    stdout: &mut io::Stdout,
) -> Result<(), Box<dyn error::Error>> {
    print!("What do you want to do?");
    stdout.execute(cursor::MoveToNextLine(1))?;

    for (idx, item) in menu_items.iter().enumerate() {
        print!("{}. {}", idx + 1, item.menuitem_txt());
        stdout.execute(cursor::MoveToNextLine(1))?;
    }

    Ok(())
}

pub fn run() -> Result<(), Box<dyn error::Error>> {
    let mut stdout = io::stdout();

    stdout.execute(terminal::EnterAlternateScreen)?;
    enable_raw_mode()?;

    let menu_items = vec![ListDepartments::new()];

    loop {
        display_menu(&menu_items, &mut stdout)?;

        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    stdout.execute(terminal::LeaveAlternateScreen)?;

    Ok(())
}
