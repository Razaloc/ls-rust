use std::env;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::io::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input: String;
    let type_option: bool;
    let mut accumulate_types: Vec<String> = Vec::new();
    let mut accumulate_paths: Vec<String> = Vec::new();
    (input, type_option) = initialize(args)?;
    //fill vectors
    for element in read_dir(input)? {
        let directory = element?;
        if type_option {
            push_type(&mut accumulate_types, &directory)?;
        }
        accumulate_paths.push(directory.path().display().to_string());
    }
    //print vectors
    if type_option {
        for i in 1..accumulate_paths.len() {
            println!("{:?}, {:?}", &accumulate_paths[i], &accumulate_types[i]);
        }
    } else {
        for i in 1..accumulate_paths.len() {
            println!("{:?}", &accumulate_paths[i]);
        }
    }
    Ok(())
}

//initialize values
fn initialize(args: Vec<String>) -> Result<(String, bool), Error> {
    let mut type_option: bool = false;
    let default_input: String;
    if args.contains(&"-t".to_string()) {
        type_option = true;
    }
    if args.len() > 1 && &args[1] != &"-t".to_string() {
        default_input = args[1].to_string();
    } else if args.len() > 2 {
        default_input = args[2].to_string();
    } else {
        default_input = String::from(".");
    }

    Ok((default_input, type_option))
}

//allow type printing
fn push_type(input: &mut Vec<String>, directory: &DirEntry) -> Result<(), Error> {
    let dir_type;
    let metadata = &directory.path().metadata()?;

    if metadata.is_file() {
        dir_type = "file";
    } else if metadata.is_dir() {
        dir_type = "dir";
    } else {
        dir_type = "???";
    }
    input.push(dir_type.to_string());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn pushing_test() {
        for element in read_dir(".") {
            let entry_1: DirEntry = DirEntry::from("entry_1");
            let entry_2: DirEntry = DirEntry::from("entry_2");
            let mut accumulate_types: Vec<String> = Vec::new();
            push_type(&mut accumulate_types, &entry_1);
            push_type(&mut accumulate_types, &entry_2);
        }
        assert_eq!(accumulate_types.len(), 2);
    }
}
