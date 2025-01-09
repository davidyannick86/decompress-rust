use std::fs;
use std::io;
use std::process::Output;

fn real_main() -> i32 {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return 1;
    }

    let file_name = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&file_name).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let output = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} has comment: {}", i, comment);
            }
        }
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, output.display());
            fs::create_dir_all(&output).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                output.display(),
                file.size()
            );
            if let Some(p) = output.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&output).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    return 0;
}

fn main() {
    std::process::exit(real_main());
}
