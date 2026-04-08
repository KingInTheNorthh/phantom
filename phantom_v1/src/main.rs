use std::fs;
use std::io;

fn print_pids() -> io::Result<()> {

    let entries = fs::read_dir("/proc")?;

    for entry in entries{
        let entry = entry?;

        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();

        // filter
        if name_str.parse::<u32>().is_ok() {

            println!("{}", name_str);
        }



    }

   Ok(())
}

fn main() {
    if let Err(e) = print_pids() {
        eprintln!("Error reading /proc: {}", e);
    }
}
