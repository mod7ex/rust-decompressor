#![allow(unused)]

use std::fs;
use std::io;
use std::env::args;
use std::path::Path;

fn main() {
    std::process::exit(_main());
}

fn _main() -> i32 {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        eprintln!("[USAGE]: {} <target: filename>", args[0]);
        return 1;
    }

    let file_path =  Path::new(&*args[1]);

    let zipped_file = fs::File::open(&file_path).unwrap();

    let mut archive = zip::ZipArchive::new(zipped_file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("[Comment] File {}: {}", i, comment);
            };
        }

        if (*file.name()).ends_with('/') {
            // Folder
            fs::create_dir_all(&outpath).unwrap();
            println!("Created folder {}", outpath.display());
        } else {
            // File
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();

            println!("File {} extracted to \"{}\" ({} bytes)", i, &outpath.display(), file.size());
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}

