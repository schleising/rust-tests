use std::{fs, path::PathBuf};

const EXTENSIONS_TO_CONVERT: [&str; 3] = ["jpg", "jpeg", "webp"];

fn convert_image_to_png(image_path: &PathBuf) -> anyhow::Result<()> {
    let output_path = image_path.with_extension("png");

    let img = image::open(image_path)?;

    img.save(output_path)?;

    // Move the original file to a backup folder
    let backup_folder = image_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory"))?
        .join("backup");
    
    fs::create_dir_all(&backup_folder)?;

    if let Some(file_name) = image_path.file_name() {
        // Construct the backup path
        let to = backup_folder.join(file_name);
        let from = image_path;

        // Move the original file to the backup folder
        fs::rename(from, to)?;
    } else {
        return Err(anyhow::anyhow!("Failed to get file name"));
    }

    Ok(())
}

fn walk_folders(folder: &str) -> anyhow::Result<()> {
    // let paths = fs::read_dir(folder)?;
    let paths = match fs::read_dir(folder) {
        Ok(paths) => paths,

        // Continue if there is an operation not permitted error
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            eprintln!("Permission denied: {}", folder);
            return Ok(());
        }

        // Return the error if it is not a permission denied error
        Err(e) => return Err(e.into()),
    };

    for path in paths {
        let path = path?.path();

        if path.is_dir() {
            walk_folders(&path.to_string_lossy())?;
            continue;
        }

        if path.extension().unwrap_or_default() == "png" {
            continue;
        }

        let ext = path.extension().unwrap_or_default().to_string_lossy();
        if !EXTENSIONS_TO_CONVERT.contains(&ext.as_ref()) {
            continue;
        }

        match convert_image_to_png(&path) {
            Ok(_) => println!("Converted: {:?}", path),
            Err(e) => eprintln!("Error converting {:?}: {}", path, e),
        }

        // Wait for user input
        println!("Press Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
    }

    Ok(())
}

fn main() {
    const INPUT_FOLDER: &str = "/Users/steve/Pictures/";

    if let Err(e) = walk_folders(INPUT_FOLDER) {
        eprintln!("Error: {}", e);
    }
}
