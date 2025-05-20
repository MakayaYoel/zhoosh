use std::io::{self, Write};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use colored::Colorize;

// TODO: Check for terminal color support (cause Windows CMD wants to act different for some reason...)

fn main() {
    let mut dir = String::new();
    let dir_map = HashMap::from([
        ("audio", vec!["wav", "mp3", "flac", "aac", "m4a"]),
        ("video", vec!["mp4", "mkv", "mov", "avi", "webm"]),
        ("documents", vec!["pdf", "docx", "doc", "txt", "odt"]),
        ("photos", vec!["jpg", "jpeg", "png", "gif", "webp"]),
        ("code", vec!["js", "py", "html", "css", "ts", "go", "rs", "php", "java", "cs", "tsx", "jsx"])
    ]);

    // get dir
    print!("Input a directory of your choice: ");
    io::stdout().flush().expect("Failed to flush stdout"); // have to empty the buffer to force the print to actually... print.

    io::stdin()
        .read_line(&mut dir)
        .expect("Unable to read directory input");

    let read_dir = fs::read_dir(dir.trim()).expect("Unable to read directory.");

    // get files
    for entry in read_dir {
        let entry = entry.expect("Couldn't get file.");
        let file_type = entry.file_type().expect("Couldn't get file type");

        if file_type.is_file() {
            let file_name = entry.file_name().into_string().expect("Couldn't convert file name to string.");
            let ext = get_file_extension(&file_name);

            // class file based on extension
            let folder = match get_file_ext_folder(ext, &dir_map) {
                Some(f) => f,
                None => String::from("other")
            };

            // create folder (if necessary)
            let dir = &dir.trim();
            let new_folder_path = format!("{dir}\\{folder}");

            if let Ok(false) = fs::exists(&new_folder_path) {
                fs::create_dir(&new_folder_path)
                    .expect(&format!("Couldn't create {folder} folder."));
            }

            // move file
            let src = format!("{dir}\\{file_name}");
            let dst = format!("{new_folder_path}\\{file_name}");
            
            if let Ok(()) = move_file(&src, &dst) {
                println!("{}", format!("Moved {file_name} into ./{folder}").yellow());
            } else {
                panic!("{}", format!("Failed to move {file_name} into ./{folder}").red());
            }
        }
    }

    println!("{}", "\nZhooshed! Press [Enter] to Exit...".green());
    let mut exit_input = String::new();
    io::stdin()
        .read_line(&mut exit_input)
        .unwrap();
}

fn get_file_extension(file_name: &str) -> &str {
    let ext = Path::new(file_name)
                            .extension()
                            .expect(&format!("Could not fetch file extension for file: {file_name}"));
    
    ext.to_str()
        .expect(&format!("Couldn't convert file extension to string for: {file_name}"))
}

fn get_file_ext_folder(ext: &str, dir_map: &HashMap<&str, Vec<&str>>) -> Option<String> {
    for (folder, extension) in dir_map {
        if extension.contains(&ext) {
            return Some(folder.to_string());
        }
    }

    None
}

fn move_file(source: &str, destination: &str) -> Result<(), io::Error> {
    fs::rename(source, destination)?;
    Ok(())
}