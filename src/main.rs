mod misc;
use crossterm::cursor::MoveToColumn;
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
                KeyCode::Enter => {
                    let result = parse_input(&input);

                    match result {
                        Ok(stream) => {
                            if let Some(s) = stream.print() {
                                print!("{}", s);
                                stdout().flush()?;
                            }
                        }
                        Err(e) => {
                            print!("{}", e);
                            stdout().flush()?;
                        }
                    }
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
