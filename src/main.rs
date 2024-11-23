use menu_control::{
    navigate::{Menu, MenuAction},
    utils,
};
use std::io::{stdout, Write};
use std::{process, thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::enable_raw_mode();

    let mut stdout = stdout();

    utils::clear_terminal();

    loop {
        menu_control::utils::clear_terminal();
        main_menu()?;
        stdout.flush()?;
    }
}

fn main_menu() -> Result<(), Box<dyn std::error::Error>> {
    let options: Vec<String> = vec![
        "1. Create new profile".to_string(),
        "2. List of profiles".to_string(),
        "3. Profile preferences".to_string(),
        "4. About".to_string(),
        "5. Exit".to_string(),
    ];

    let mut main_menu = Menu::new(
        options,
        0,
        "Command Center".to_string(),
        "@Rafael Ramos - 2024".to_string(),
    );

    loop {
        utils::clear_terminal();

        match Menu::navigate_control(&mut main_menu)? {
            MenuAction::Navigate(_) => continue,
            MenuAction::Select => {
                match main_menu.selected {
                    0 => {
                        println!("entrou 0");
                        thread::sleep(time::Duration::from_secs(2));
                    }
                    1 => {
                        println!("entrou 1");
                        thread::sleep(time::Duration::from_secs(2));
                    }
                    2 => {
                        println!("entrou 2");
                        thread::sleep(time::Duration::from_secs(2));
                    }
                    3 => {
                        println!("entrou 3");
                        thread::sleep(time::Duration::from_secs(2));
                    }
                    4 => {
                        utils::clear_terminal();
                        println!("Closing...");
                        thread::sleep(time::Duration::from_secs(2));
                        process::exit(0)
                    }
                    _ => println!("Invalid option!"),
                }
                println!("\nPress any key to return to the menu...");
                // event::read()?;
            }
            MenuAction::Back => return Ok(()),
            MenuAction::Exit => {
                utils::clear_terminal();
                println!("Closing...");
                process::exit(0)
            }
        }
    }
}
