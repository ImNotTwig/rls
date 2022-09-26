pub mod file_extensions;
use file_extensions::file_extensions::{IMAGE_FILES, 
                                       MUSIC, 
                                       LOSSLESS, 
                                       CRYPTO, 
                                       ARCHIVE};
use walkdir::WalkDir;

use std::fs;
use std::path::Path;
use std::ffi::OsStr;

use ansi_term::Style;
use ansi_term::Color::{Red, Green, Blue, Cyan, White, Fixed};

use is_executable::is_executable;




#[macro_use(lazy_static)]
extern crate lazy_static;

pub struct FileColors {
    archive: Style,
    executable: Style,
    folder: Style,
    syslink: Style,
    text: Style,
    image: Style,
    music: Style,
    lossless: Style,
    crypto: Style
}


pub fn main() {
                                                                        // collect the command line arguments into a vector of string
    //let mut argv: Vec<String> = std::env::args().collect();
    let mut files = vec![]; 
                                                                        //setting colors for the theme
    let theme = FileColors {
        archive: Red.bold(),
        executable: Green.bold(),
        folder: Blue.bold(),
        syslink: Cyan.bold(),
        text: White.bold(),
        image: Fixed(133).bold(),
        music: Fixed(92).bold(),
        lossless: Fixed(93).bold(),
        crypto: Fixed(109).bold()
    };

                                                                        // if no arguments are given assume the current directory
    //if argv.len() < 2 {
    //    argv.push(String::from("."));
    //}

                                                                        // iterate over the vector of args ignoring the first argument

                                                                        // iterate over every entry in the dir
    for file in WalkDir::new("./").min_depth(1).max_depth(1).follow_links(true) {
                                                                        // initializing some variables
        let name = file.as_ref().unwrap();
        let is_symlink = name.path_is_symlink();
        let mut name = name.path().display().to_string();
        let mut metadata = fs::metadata(&name).expect("failed to read metadata");
                                                                        //getting the metadata of the name
        if is_symlink == false {
            metadata = fs::metadata(&name).expect("failed to read metadata");
        }else {
            metadata = fs::symlink_metadata(&name).unwrap();
        }

        name = name.replace("./","");                                   //remove the "./" from before the path

                                                                        //check to see if the metadata shows if its a file, 
                                                                        //folder, or system link and then setting color
                                                                        
        if metadata.is_dir() && is_symlink == false {                   //checking if it is a directory
            name.push('/');
            name = theme.folder.paint(name).to_string();
            println!("{}", name);

        }else if is_symlink == true {                                   //checking if its a symlink
            let name_path = fs::read_link(&name);
            name = theme.syslink.paint(name).to_string();
            println!("{} -> {}", name, theme.syslink.paint(name_path.unwrap().display().to_string()));

        }else if metadata.is_file() {                                   //checking if its a file
            let extension = find_extension(&name);
                                                                        //if the file extension is not empty 
            if extension != None {
                let extension = extension.unwrap().to_string();

                if IMAGE_FILES.contains(&extension) {                   //if the file is an IMAGE 
                    name = theme.image.paint(name).to_string();

                }else if is_executable(&name) {                    //if the file is a PROGRAM
                    name = theme.executable.paint(name).to_string();

                }else if MUSIC.contains(&extension) {                   //if the file is a MUSIC FILE
                    name = theme.music.paint(name).to_string();
                    
                }else if LOSSLESS.contains(&extension) {                //if the file is a LOSSLESS MUSIC FILE
                    name = theme.lossless.paint(name).to_string();

                }else if CRYPTO.contains(&extension) {                  //if the file is a CRYPTO FILE
                    name = theme.crypto.paint(name).to_string();

                }else if ARCHIVE.contains(&extension) {                 //if the file is an ARCHIVE
                    name = theme.archive.paint(name).to_string();

                }else {                                                 //is text file or is not recognized
                    name = theme.text.paint(name).to_string();
                }
                files.push(name);
            }else {                                                     //if the file extension is empty 
                name = theme.text.paint(name).to_string();
                files.push(name);
            }
        }
    
    }
    for file in files {                                          //print the files (not folders or symlinks)
        println!("{}", file);
    }
}

fn find_extension(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}