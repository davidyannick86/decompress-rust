use std::fs;
use std::io;

fn real_main() -> i32 {
    // Get the command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if the number of arguments is less than 2
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return 1;
    }

    // Get the file name from the command line arguments
    let file_name = std::path::Path::new(&*args[1]);

    // Check if the file exists
    let file = fs::File::open(&file_name).unwrap();

    // Create a new ZipArchive from the file
    let archive = zip::ZipArchive::new(file).unwrap();

    fun_name(archive);

    return 0;
}

fn fun_name(mut archive: zip::ZipArchive<fs::File>) {
    // Iterate over the files in the archive
    for i in 0..archive.len() {
        // Get the file at index i
        let mut file = archive.by_index(i).unwrap();

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
            fs::create_dir_all(&output).unwrap();
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
                    fs::create_dir_all(&p).unwrap();
                }
            }
            // Write the file
            let mut outfile = fs::File::create(&output).unwrap();
            // Copy the file
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Set the permissions of the file
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&output, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}

fn main() {
    // Call the real_main function and exit with the return value
    std::process::exit(real_main());
}
