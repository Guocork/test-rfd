use std::fs;

use rfd::FileDialog;

fn main() {
    if let Some(path) = FileDialog::new().pick_folder() {
        println!("Selected folder: {:?}", path);
        fs::create_dir_all(&path).unwrap_or_else(|err| {
            eprintln!("Error creating new download directory: {:?}", err);
        });
    } else {
        println!("No folder selected");
    }
}
