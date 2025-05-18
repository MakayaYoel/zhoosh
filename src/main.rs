use std::io::{self, Write};
use std::fs;
use std::path::Path;

fn main() {
    let mut dir = String::new();

    print!("Input a directory of your choice: ");
    io::stdout().flush().expect("Failed to flush stdout"); // have to empty the buffer to force the print to actually... print.

    io::stdin()
        .read_line(&mut dir)
        .expect("Unable to read directory input");

    let dir = fs::read_dir(dir.trim()).expect("Unable to read directory.");

    for entry in dir {
        let entry = entry.expect("Couldn't get file.");
        let file_type = entry.file_type().expect("Couldn't get file type");

        if file_type.is_file() {
            let file_name = entry.file_name().into_string().expect("huh.");
            let ext = get_file_extension(&file_name);

            println!("{file_name} has an extension of {ext}");
        }
    }
}

fn get_file_extension(file_name: &str) -> &str {
    let ext = Path::new(file_name).extension().expect("Could not fetch extension.");

    ext.to_str().expect("Couldn't convert file extension to string.")
}
