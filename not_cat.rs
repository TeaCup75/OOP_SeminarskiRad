use std::env::args;
use std::fs::OpenOptions;
use std::io::Read;

fn main() {
    let mut args = args();

    let path = args.nth(1);

    let Some(path) = path else {
        println!("You must provide a path to a file");
        return;
    };

    let file = OpenOptions::new()
        .read(true)
        .open(&path);

    let Ok(mut file) = file else {
        println!("Failed to open the file. Does the file {} exist?", path);
        return;
    };

    let mut buf = String::new();
    if file.read_to_string(&mut buf).is_err() {
        println!("Failed to read from the file.");
    }

    println!("{}", buf);
}
