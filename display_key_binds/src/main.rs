#![allow(dead_code)]
mod keybind;

use keybind::*;
use std::{fs, path::PathBuf};

fn main() {
    let mut in_file_content = String::new();

    let mut path: PathBuf = PathBuf::new();

    if let Some(home) = dirs::home_dir() {
        path.push(home);
        path.push("dotfiles/sxhkd/sxhkdrc");
        println!("home: {:?}", path);
    }

    in_file_content.push_str(&fs::read_to_string(path).expect("Error reading file"));

    let mut keybinds: Vec<Keybind> = Vec::<Keybind>::new();

    keybinds.push(Keybind::new());
    let mut catagori = String::new();
    let mut change_catagori = false;

    for line in in_file_content.split('\n') {
        if let Some(first_char) = line.chars().next() {
            if first_char == '#' {
                // get catagori
                if (line.as_bytes()[1] as char) == '#'  && !change_catagori{
                    change_catagori = true;
                    continue;
                }
                if (line.as_bytes()[1] as char) == ' ' && change_catagori {
                    // println!("catagori: {catagori}");
                    catagori = line.to_string();
                    continue;
                }
                if (line.as_bytes()[1] as char) == '#' && change_catagori {
                    change_catagori = false;
                    continue;
                }
                keybinds.push(Keybind::new());
                // set name
                if let Some(last) = keybinds.last_mut() {
                    last.set_name(line.to_string());
                    last.set_catagori(catagori.clone());
                } else {
                    println!("last not there")
                }
                continue;
            }
            if !line.starts_with('\t') {
                if let Some(last) = keybinds.last_mut() {
                    last.add_key(line.to_string());
                } else {
                    println!("last not there")
                }
                continue;
            }
        };
    }

    println!("Print Keybinds");
    for keybind in keybinds {
        keybind.print();
    }
}
