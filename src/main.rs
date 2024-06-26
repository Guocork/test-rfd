use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use rfd::FileDialog;

fn main() {
    if let Some(path) = FileDialog::new().pick_folder() {
        println!("Selected folder: {:?}", path);
        // fs::create_dir_all(&path).unwrap_or_else(|err| {
        //     eprintln!("Error creating new download directory: {:?}", err);
        // });

        let mut file_path = PathBuf::from(path);
        file_path.push("example.txt") ;

        // 这里creat 如果之前这个文件已经存在 那么将会截断（覆盖）原来的内容，重新写入
        match File::create(&file_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(b"Hello, world!") {
                    eprintln!("Failed to write to file: {}", e);
                } else {
                    println!("File created successfully: {:?}", file_path);
                }
            },
            Err(e) => eprintln!("Failed to create file: {}", e),
        }
    } else {
        println!("No folder selected");
    }
}
