mod misc;
use crossterm::cursor::{MoveLeft, MoveToColumn};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use misc::models::BUILTINS;
use misc::parsers::parse_input;
use std::io::{self, stdout, Write};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut input = String::new();
    print!("$ ");
    stdout().flush()?;

    loop {
        // io::stdin().read_line(&mut input).unwrap();
        //
        match read()? {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Tab => {
                    let suggestions: Vec<&str> = BUILTINS
                        .iter()
                        .filter(|builtin| builtin.starts_with(&input))
                        .cloned()
                        .collect();

                    if suggestions.len() > 0 {
                        execute!(stdout(), MoveToColumn(2), Clear(ClearType::UntilNewLine))?;
                        input = suggestions.first().unwrap().to_string();
                        print!("{} ", input);
                        stdout().flush()?;
                    }
                }
                KeyCode::Backspace => {
                    if !input.is_empty() {
                        input.pop();
                        execute!(stdout(), MoveLeft(1), Clear(ClearType::UntilNewLine))?;
                        stdout().flush()?;
                    }
                }
                KeyCode::Enter => {
                    println!();
                    execute!(stdout(), MoveToColumn(0))?;
                    let result = parse_input(&input);

                    match result {
                        Ok(stream) => {
                            if let Some(s) = stream.print() {
                                s.lines().for_each(|l| {
                                    println!("{}", l);
                                    execute!(stdout(), MoveToColumn(0)).unwrap();
                                });
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }

                    input.clear();
                    print!("$ ");
                    stdout().flush()?;
                }
                KeyCode::Char(c) => {
                    input.push(c);
                    print!("{}", c);
                    stdout().flush()?;
                }
                _ => {
                    print!("$ ");
                    stdout().flush()?;
                }
            },
            _ => disable_raw_mode()?,
        }
    }
}
