use rttex::get_image_buffer;
use std::path::Path;
use walkdir::WalkDir;
use std::panic;
use std::fs;

fn main() {
    let game_dir = Path::new("game");
    for entry in WalkDir::new(game_dir) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                println!("Error reading directory entry: {}", e);
                continue;
            }
        };
        
        if entry.path().extension().unwrap_or_default() == "rttex" {
            let path_str = match entry.path().to_str() {
                Some(s) => s,
                None => {
                    println!("Invalid UTF-8 path: {}", entry.path().display());
                    continue;
                }
            };
            
            let result = panic::catch_unwind(|| {
                get_image_buffer(path_str)
            });
            
            match result {
                Ok(Some(img)) => {
                    let png_path = entry.path().with_extension("png");
                    match img.save(&png_path) {
                        Ok(()) => {
                            println!("Converted {} to {}", entry.path().display(), png_path.display());
                            match fs::remove_file(entry.path()) {
                                Ok(()) => {},
                                Err(e) => println!("Failed to remove original file {}: {}", entry.path().display(), e),
                            }
                        },
                        Err(e) => println!("Failed to save PNG for {}: {}", entry.path().display(), e),
                    }
                },
                Ok(None) => {
                    println!("Failed to load {}", entry.path().display());
                },
                Err(_) => {
                    println!("Panic occurred while processing {}, skipping...", entry.path().display());
                }
            }
        }
    }
}
