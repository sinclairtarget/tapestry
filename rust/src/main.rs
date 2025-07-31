mod error;
mod escape;
mod io;
mod terminal;

use std::env;
use std::fmt::Write;
use std::process;

use error::TapestryError;

const MAX_COLS: u16 = 512;

fn main() {
    let args: Vec<String> = env::args().collect();
    let allow_private = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        print_usage(&args[0]);
        process::exit(1);
    });

    if let Err(e) = run(allow_private) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn parse_args(args: &[String]) -> Result<bool, TapestryError> {
    if args.len() < 2 {
        return Ok(false);
    }

    let flag = &args[1];
    if flag == "--allow-private" {
        Ok(true)
    } else {
        Err(TapestryError::new(
            "unrecognized command-line flag".to_string(),
        ))
    }
}

fn print_usage(progname: &str) {
    println!("Usage: {progname} [--allow-private]");
}

struct State {
    window_rows: u16,
    window_cols: u16,
    last_input_key: u8,
}

fn ctrl(b: u8) -> u8 {
    b & 0x1f
}

fn draw(state: &State) -> Result<(), TapestryError> {
    io::write(escape::CURSOR_HOME)?;

    let rows = state.window_rows;
    let cols = if state.window_cols > MAX_COLS {
        MAX_COLS
    } else {
        state.window_cols
    };
    let midrow = rows / 2;

    let mut line = String::with_capacity(cols.into());
    let mut buf = String::with_capacity((cols + 2).into());
    for row in 0..rows {
        io::write(escape::ERASE_LINE)?;

        if row == midrow - 2 {
            write!(&mut line, "{} x {}", rows, cols)?;
        } else if row == midrow - 1 {
            if state.last_input_key == b'\0' {
                write!(&mut line, "-")?;
            } else if state.last_input_key.is_ascii_control() {
                write!(&mut line, "{}", state.last_input_key)?;
            } else {
                write!(
                    &mut line,
                    "{} ('{}')",
                    state.last_input_key,
                    char::from(state.last_input_key),
                )?;
            }
        } else if row == midrow + 1 {
            write!(&mut line, "Press ^c or ^q to quit.")?;
        }

        let left_padding = (usize::from(cols) - line.len() - 2) / 2;
        let right_padding = (usize::from(cols) - line.len() - 2).div_ceil(2);
        let repeat = usize::from(cols) - 2;
        if row == 0 {
            write!(&mut buf, "+{:-<repeat$}+\r\n", "")?;
        } else if row == rows - 1 {
            write!(&mut buf, "+{:-<repeat$}+", "")?;
        } else {
            write!(
                &mut buf,
                "|{:left_padding$}{}{:right_padding$}|\r\n",
                "", &line, "",
            )?;
        };

        io::write(buf.as_bytes())?;
        line.clear();
        buf.clear();
    }

    Ok(())
}

fn update(state: &mut State, key: u8) -> bool {
    state.last_input_key = key;

    if state.last_input_key == ctrl(b'q') || state.last_input_key == ctrl(b'c')
    {
        return false;
    }

    true
}

fn run(allow_private: bool) -> Result<(), TapestryError> {
    let tear_down = terminal::set_up(allow_private)?;
    let (rows, cols) = terminal::get_dimensions()?;

    let mut key: u8 = b'\0';
    let mut state: State = State {
        window_rows: rows,
        window_cols: cols,
        last_input_key: b'\0',
    };

    loop {
        draw(&state)?;

        io::read_key(&mut key)?;
        if !update(&mut state, key) {
            break;
        }
    }

    tear_down()?;
    Ok(())
}
