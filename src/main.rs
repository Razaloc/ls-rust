use std::fs::read_dir;
use std::io::Error;

fn main() -> Result<(), Error> {
    for element in read_dir(".")? {
        let directory = element?;
        let dir_type;
        let metadata = &directory.path().metadata()?;

        if metadata.is_file() {
            dir_type = "file";
        } else if metadata.is_dir() {
            dir_type = "dir";
        } else {
            dir_type = "???";
        }

        println!(
            "{:?}
                 {:?}",
            directory.path(),
            dir_type
        );
    }

    Ok(())
}
