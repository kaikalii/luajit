use std::{
    fs,
    io::{stdin, stdout, BufRead, Write},
    path::{Path, PathBuf},
    process::exit,
};

use clap::Parser;
use colored::{Color, Colorize};
use mlua::{Function, Lua, Value};

#[derive(Parser)]
struct App {
    path: Option<PathBuf>,
}

fn main() {
    let _ = ctrlc::set_handler(|| {
        print!("\r ");
        exit(0);
    });

    let app = App::parse();

    if let Some(mut path) = app.path {
        if path.extension().is_none() {
            path = path.with_extension("lua");
        }
        if let Err(e) = run_file(&path) {
            eprintln!("{e}");
        }
    } else {
        run_repl();
    }
}

fn run_file(path: &Path) -> anyhow::Result<()> {
    let code = fs::read(path)?;
    let lua = Lua::new();
    lua.load(&code).exec()?;
    Ok(())
}

fn run_repl() {
    let lua = Lua::new();
    let tostring: Function = lua.globals().get("tostring").unwrap();
    print_prompt(false);
    let mut last_was_error;
    for line in stdin().lock().lines() {
        let line = if let Ok(line) = line {
            line
        } else {
            break;
        };
        match lua.load(&line).eval::<Value>() {
            Ok(val) => {
                if val != Value::Nil {
                    let stringed: String = tostring.call(val).unwrap();
                    println!("{}", stringed.bold());
                }
                last_was_error = false;
            }
            Err(e) => {
                eprintln!("{e}");
                last_was_error = true;
            }
        }
        print_prompt(last_was_error);
    }
}

fn print_prompt(error: bool) {
    let color = if error {
        Color::BrightRed
    } else {
        Color::BrightGreen
    };
    print!("{}", "> ".bold().color(color));
    let _ = stdout().flush();
}
