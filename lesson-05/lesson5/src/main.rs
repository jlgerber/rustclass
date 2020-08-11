use std::fs;

use std::io;

fn get_string() -> io::Result<String> {
   let mut buffer = String::new();

   fs::read_to_string(&mut buffer)?;

   Ok(buffer)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let results = get_string()?;
   Ok(())
}
