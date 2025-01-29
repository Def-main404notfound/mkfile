use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::string::String;
fn write_file(text: String, name: String, append: bool) {
    let format_text = text.replace("/", "\n");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .open(name)
        .unwrap();
    file.write_all(format_text.as_bytes()).unwrap();
    println!("File written successfully.");
}
fn read_file(name: String) {
    let mut file = File::open(name).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    println!("{}", text);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 5
        || args.len() < 2
        || (args[1] == "-h"
            && args[1] == "--help"
            && args[1] != "-w"
            && args[1] != "-r"
            && args[1] != "--write"
            && args[1] != "--read"
            && args[1] != "r"
            && args[1] != "append")
    {
        println!("Usage: {} <arg> <filename> <filetext>", args[0]);
        println!("Arg:");
        println!("-w --write: Write text to the file");
        println!("-a --append Append text to the file");
        println!("-r --read:print file.");
        return;
    }
    let arg = args[1].clone();
    let name = args[2].clone();
    let path = Path::new(&name);
    if arg == "-w" || arg == "--write" {
        let text = args[3].clone();
        if path.exists() {
            write_file(text, name, false);
        } else {
            std::fs::File::create(name.clone()).unwrap();
            write_file(text, name, false);
        }
    } else if arg == "-a" || arg == "--append" {
        let text = args[3].clone();
        if path.exists() {
            write_file(text, name, true);
        } else {
            std::fs::File::create(name.clone()).unwrap();
            write_file(text, name, true);
        }
    } else if arg == "-r" || arg == "--read" || path.exists() {
        read_file(name);
    } else {
        println!("File does not exist.");
    }
}
