use std::env;
use std::fs;
use std::path::Path;
use std::process;

use crossterm::style::{Attribute::*, Color::*};
use termimad::*;

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();

    skin.set_headers_fg(rgb(64, 157, 254));
    skin.italic.add_attr(Underlined);

    for h in &mut skin.headers {
        h.align = Alignment::Left;
        h.add_attr(Bold);
        h.set_fg(rgb(0x75, 0x71, 0xF9));
    }

    skin.scrollbar.thumb.set_fg(ansi(178));
    // skin.table.align = Alignment::Center;
    // skin.code_block.align = Alignment::Center;
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Need a file to try to read?");
        process::exit(0x0100);
    }

    // Skip first arg, as it's probably oneseelf
    for file in args[1..].into_iter() {
        if !Path::new(&file).exists() {
            println!("{} Doesn't exit?", &file);
            process::exit(0x0101);
        }

        let contents = fs::read(&file)?;
        let foo = String::from_utf8_lossy(&contents);
        let s = make_skin();
        s.print_text(&foo);
    }
    Ok(())
}
