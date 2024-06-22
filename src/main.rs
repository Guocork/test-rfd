use rfd::FileDialog;

fn main() {
    if let Some(path) = FileDialog::new().pick_folder() {
        println!("Selected folder: {:?}", path);
    } else {
        println!("No folder selected");
    }
}
