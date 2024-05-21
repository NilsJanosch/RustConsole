use crossterm::{
    ExecutableCommand, execute,
    terminal::{Clear, ClearType},
    style::{Color, Print, ResetColor, SetForegroundColor},
    cursor::MoveTo,
    event::{self, Event, KeyCode}
};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{stdout, Write};

fn main() {
    let options = ["Option 1", "Option 2", "Option 3", "Exit"];
    let mut selected = 0;

    loop {
        display_menu(&options, selected);
        match read_key() {
            KeyCode::Up => {
                if selected > 0 {
                    selected -= 1;
                }
            }
            KeyCode::Down => {
                if selected < options.len() - 1 {
                    selected += 1;
                }
            }
            KeyCode::Enter => {
                if options[selected] == "Exit" {
                    break;
                } else {
                    execute_option(options[selected]);
                }
            }
            _ => {}
        }
    }
    println!("Exiting...");
}

fn display_menu(options: &[&str], selected: usize) {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();
    for (index, option) in options.iter().enumerate() {
        if index == selected {
            stdout.execute(SetForegroundColor(Color::Yellow)).unwrap();
            println!("> {}", option);
            stdout.execute(ResetColor).unwrap();
        } else {
            println!("  {}", option);
        }
    }
    stdout.flush().unwrap();
}

fn read_key() -> KeyCode {
    loop {
        if let Event::Key(event) = event::read().unwrap() {
            return event.code;
        }
    }
}

fn execute_option(option: &str) {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos:>7}/{len:7} {percent}%")
            .expect("Failed to set progress bar template")
            .progress_chars("##-")
    );
    pb.set_message(option.to_string());

    for _ in 0..100 {
        pb.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    pb.finish_with_message("Done");
}
