use std::fs::File;
use std::fs;
use std::io::{Seek, SeekFrom, Read, Write};
use std::path::Path;
use std::env;
use std::fs::OpenOptions;
use std::time::SystemTime;

fn read_bytes_and_write_to_file(file_path: &str, output_directory: &str) -> std::io::Result<()> {
    let path = Path::new(file_path);
    let output_file_path = format!("{}/{}_recovered.txt", output_directory, path.file_stem().unwrap().to_str().unwrap());
    // set up logging
    let mut file = File::open(file_path)?;
    let success_log_path = format!("{}/successfully_recovered_log.txt", output_directory);
    let mut success_log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(success_log_path)
        .unwrap();

    let error_log_path = format!("{}/error_log.txt", output_directory);
    let mut error_log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(error_log_path)
        .unwrap();


    // check if file is too small for the buffer
    let mut file_size_buffer = vec![0; 1024];
    let bytes_read = file.read(&mut file_size_buffer)?;
    if bytes_read < 1024 {
        writeln!(error_log, "[ERROR]file {:?} doesn't fill whole buffer", file).unwrap();
        return Ok(())
    }

    // move cursor to offset 512 to check byte signature matches .doc
    file.seek(SeekFrom::Start(0x200))?;
    let mut buffer = vec![0; 4];
    file.read_exact(&mut buffer)?;
    if buffer != [0xec, 0xa5, 0xc1, 0x00] {
        return Ok(());
    }

    // move cursor to offset 0xA00 (start of text stream)
    if file.seek(SeekFrom::Start(0xa00)).is_err() {
        writeln!(error_log, "No offset 0xa00 in file file may be a .doc file but has no text: {}", file_path).unwrap();
        return Ok(());
    }

    buffer = vec![];
    file.read_to_end(&mut buffer)?;

    // stop read of bytes when encountering null byte
    let null_byte_index = buffer.iter().position(|&x| x == 0).unwrap();
    let bytes_without_null = &buffer[..null_byte_index];

    // write bytes between offset 0xA00 and null byte to file
    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(bytes_without_null)?;
     writeln!(success_log, "Text recovered and written to: {:?}", output_file).unwrap();
     Ok(())
}

fn main() {

    // define args
    let mut directory_path = "";
    let mut output_directory = "";
    let mut show_help = false;

    // collect args
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        if arg == "-d" {
            directory_path = &args[i + 1];
        } else if arg == "-o" {
            output_directory = &args[i + 1];
        } else if arg == "-h" {
            show_help = true;
        }
    }

    if show_help {
        println!("Usage: ./doc2txt -d <directory> -o <output_directory>");
        return;
    }

    if directory_path == "" || output_directory == "" {
        println!("Invalid command line arguments. Please use -d for directory and -o for output directory.");
        return;
    }

    // check if output directory exists
    if !Path::new(output_directory).exists() {
        fs::create_dir(output_directory).unwrap();
    }

    // Get all files in the directory
    let paths = fs::read_dir(directory_path).unwrap();

    let start = SystemTime::now();
    println!("Started: {:?}", start);

    // Iterate over all files and apply the function
    for path in paths {
        let file_path = path.unwrap().path();
    if file_path.is_file(){
        match read_bytes_and_write_to_file(file_path.to_str().unwrap(),output_directory){
            Ok(_) => continue,
            Err(e) => println!("An error occured: {}", e) // unknown errors print to console

        }
    }
   }
    let end = SystemTime::now();
    println!("Finished: {:?}", end);

}
