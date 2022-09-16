use std::fs::read_dir;
use std::io::Error;

fn main() -> Result<(), Error> {
    for element in read_dir(".")? {
        let directory = element?;
        println!("{:?}", directory.path());
    }

    Ok(())
}
