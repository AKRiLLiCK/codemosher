use std::io::stdout;
use std::io;
use std::io::Write;
use crossterm::cursor;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::event;
use crossterm::terminal;
use crossterm::event::KeyEventKind;
use crossterm::event::Event;
use crossterm::terminal::ClearType;
use crossterm::style::Print;

fn generate_game() -> io::Result<()> {
    let mut stdout = io::stdout();
    let code: u32 = rand::random_range(0..=10);

    execute!(stdout, crossterm::style::Print("Guess the number between 0 and 10!\n"))?;
    let _ = stdout.flush();

    let _: () = loop {
        let guess = read_line();
        let Ok(input) = guess else {
            execute!(stdout, crossterm::style::Print("Error reading input. Please try again.\n"))?;
            continue;
        };
        let Ok(num) = input.parse::<u32>() else {
            execute!(stdout, crossterm::style::Print("This isn't a number! Please try again.\n"))?;
            continue;
        };
        if check_code(num, code)? {
            execute!(stdout, crossterm::style::Print("Correct! You've guessed the number!\n"))?;
            break;
        }
    };
    Ok(())
}

fn check_code(guess: u32, code: u32) -> io::Result<bool> {
	let mut stdout = stdout();
	execute!(stdout, Print(format!("Your guess: {}\n", guess)))?;

	match guess.cmp(&code) {
	    std::cmp::Ordering::Less => { execute!(stdout, Print("Bigger!\n"))?; Ok(false) }
		std::cmp::Ordering::Equal => { execute!(stdout, Print("You got it!\n"))?; Ok(true) }
    	std::cmp::Ordering::Greater => { execute!(stdout, Print("Smaller!\n"))?; Ok(false) }
	}
}

fn main() -> io::Result<()> {
	terminal::enable_raw_mode()?;
	let mut stdout = io::stdout();

	execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
	execute!(stdout, crossterm::style::Print("Welcome to codemosher, do you want to play a game? [y/n]\n"))?;
	stdout.flush()?;

    loop {
        match read_line() {
            Ok(input) => match input.as_str() {
                "y" => {
                    generate_game()?;
                    break;
                }
                "n" => {
                    execute!(stdout, crossterm::style::Print("Exiting...\n"))?;
                    break;
                }
                _ => {}
            },
            Err(_) => continue,
        }
    }

	stdout.flush()?;
	Ok(())
}

fn read_line() -> Result<String, ()> {
	let mut buf = String::new();
	loop {
		match event::read() {
			Ok(Event::Key(k)) if k.kind == KeyEventKind::Press => match k.code {
				KeyCode::Char(c) => buf.push(c),
				KeyCode::Backspace => { buf.pop(); }
				KeyCode::Enter => return Ok(buf),
				_ => {}
			},
			Ok(_) => {},
			Err(_) => return Err(()),
		}
	}
}