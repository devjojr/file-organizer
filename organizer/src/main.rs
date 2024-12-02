use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <subdirectory_name> <file_extension>", args[0]);
        return Ok(());
    }

    let subdirectory_name = &args[1];
    let file_extension = &args[2];

    let directory_path = Path::new(".");
    let subdirectory_path = directory_path.join(subdirectory_name);

    create_subdirectory(&subdirectory_path)?;

    // iterate through files in the current directory
    for entry in fs::read_dir(directory_path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(extension) = get_file_extension(&path) {
            if extension == file_extension {
                let new_path = subdirectory_path.join(path.file_name().unwrap());
                move_file(&path, &new_path)?;
                println!("Moved {} to {}", path.display(), new_path.display());
            }
        }
    }

    println!("All files with the extension '{}' moved to '{}'", file_extension, subdirectory_name);
    Ok(())
}

fn get_file_extension(path: &Path) -> Option<&str> {
    path.extension()?.to_str()
}

fn create_subdirectory(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir(path)?;
        println!("Created a new subdirectory: {}", path.display());
    } else {
        println!("Subdirectory already exists: {}", path.display());
    }
    Ok(())
}

fn move_file(source: &Path, dest: &Path) -> io::Result<()> {
    fs::rename(source, dest)?;
    Ok(())
}
