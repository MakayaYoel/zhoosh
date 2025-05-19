use std::io::{self, Write};
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

fn main() {
    let mut dir = String::new();
    // let mut ext_map: HashMap<String, Vec<String>> = HashMap::new();
    let dir_map = HashMap::from([
        ("audio", vec!["wav", "mp3", "flac", "aac", "m4a"]),
        ("video", vec!["mp4", "mkv", "mov", "avi", "webm"]),
        ("documents", vec!["pdf", "docx", "doc", "txt", "odt"]),
        ("photos", vec!["jpg", "jpeg", "png", "gif", "webp"]),
        ("code", vec!["js", "py", "html", "css", "ts", "go", "rs", "php", "java", "cs", "tsx", "jsx"])
    ]);

    print!("Input a directory of your choice: ");
    io::stdout().flush().expect("Failed to flush stdout"); // have to empty the buffer to force the print to actually... print.

    io::stdin()
        .read_line(&mut dir)
        .expect("Unable to read directory input");

    let read_dir = fs::read_dir(dir.trim()).expect("Unable to read directory.");

    for entry in read_dir {
        let entry = entry.expect("Couldn't get file.");
        let file_type = entry.file_type().expect("Couldn't get file type");

        if file_type.is_file() {
            let file_name = entry.file_name().into_string().expect("huh.");
            let ext = get_file_extension(&file_name);

            // ext_map
            //     .entry(ext.to_string())
            //     .or_insert(Vec::new())
            //     .push(file_name);

            if let Some(folder_type) = find_folder_for_extension(ext, &dir_map) {
                let src_path = PathBuf::from(dir.trim()).join(&file_name);
                let mut dst_dir = PathBuf::from(dir.trim());
                dst_dir.push(folder_type);

                if !dst_dir.exists() {
                    fs::create_dir(&dst_dir).expect("Could not create directory");
                }

                let mut dst_path = dst_dir.clone();
                dst_path.push(&file_name);

                move_file(&src_path, &dst_path).expect("Could not move file.");
            } else {
                let mut other_dir = PathBuf::from(dir.trim());
                other_dir.push("other");

                if !other_dir.exists() {
                    fs::create_dir(&other_dir).expect("Could not create path.");
                }

                let src_path = PathBuf::from(dir.trim()).join(&file_name);
                let mut dst_path = other_dir.clone();
                dst_path.push(&file_name);

                move_file(&src_path, &dst_path).expect("Could not move file.");
            }
        }
    }

    println!("Zhooshed!");
}

fn get_file_extension(file_name: &str) -> &str {
    let ext = Path::new(file_name).extension().expect("Could not fetch extension.");

    ext.to_str().expect("Couldn't convert file extension to string.")
}

fn find_folder_for_extension(ext: &str, dir_map: &HashMap<&str, Vec<&str>>) -> Option<String> {
    for (folder, extensions) in dir_map {
        if extensions.contains(&ext) {
            return Some(folder.to_string());
        }
    }

    None
}

fn move_file(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::copy(src, dst)?;
    fs::remove_file(src)?;
    Ok(())
}
