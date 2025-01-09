use std::fs;
use std::io;

fn real_main() -> Result<i32, Box<dyn std::error::Error>> {
    // Get the command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if the number of arguments is less than 2
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return Err("No file name provided".into());
    }

    // Get the file name from the command line arguments
    let file_name = std::path::Path::new(&*args[1]);

    // Check if the file exists
    let file = fs::File::open(&file_name)?;

    // Create a new ZipArchive from the file
    let archive = zip::ZipArchive::new(file)?;

    process_archive(archive)?;

    return Ok(0);
}

fn process_archive(
    mut archive: zip::ZipArchive<fs::File>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Iterate over the files in the archive
    for i in 0..archive.len() {
        // Get the file at index i
        let mut file = archive.by_index(i)?;

        // Get the name of the file
        let output = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // Print the name of the file
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} has comment: {}", i, comment);
            }
        }

        // Check if the file is a directory
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, output.display());
            // Create the directory
            fs::create_dir_all(&output)?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                output.display(),
                file.size()
            );
            // Create the file
            if let Some(p) = output.parent() {
                if !p.exists() {
                    // Create the directory
                    fs::create_dir_all(&p)?;
                }
            }
            // Write the file
            let mut outfile = fs::File::create(&output)?;
            // Copy the file
            io::copy(&mut file, &mut outfile)?;
        }

        // Set the permissions of the file
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&output, fs::Permissions::from_mode(mode))?;
            }
        }
    }
    Ok(())
}

fn main() {
    // Call the real_main function and exit with the return value
    std::process::exit(real_main().unwrap_or(1));
}
