use std::env;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::io::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut input = Commands {
        types: false,
        path: String::from("."),
    };
    input.update(args);
    let mut accumulate_types: Vec<String> = Vec::new();
    let mut accumulate_names: Vec<String> = Vec::new();
    //fill vectors
    for element in read_dir(input.path)? {
        let directory = element?;
        if input.types {
            push_type(&mut accumulate_types, &directory)?;
        }
        let path_name = directory.path().display().to_string();
        let split = path_name.split("/");
        if let Some(name) = split.last() {
            accumulate_names.push(name.to_string())
        };
    }
    //print vectors
    if input.types {
        for i in 0..accumulate_names.len() {
            println!("{}: {}", &accumulate_types[i], &accumulate_names[i]);
        }
    } else {
        for i in 0..accumulate_names.len() {
            println!("{}", &accumulate_names[i]);
        }
    }
    Ok(())
}

struct Commands {
    path: String,
    types: bool,
}
impl Commands {
    fn update(&mut self, args: Vec<String>) -> bool {
        for element in &args[1..] {
            match &element[..] {
                "-t" => self.types = true,
                _ => self.path = String::from(element),
            };
        }
        true
    }
}
//allow type printing
fn push_type(input: &mut Vec<String>, directory: &DirEntry) -> Result<(), Error> {
    let dir_type;
    let metadata = &directory.path().metadata()?;

    if metadata.is_file() {
        dir_type = "file";
    } else if metadata.is_dir() {
        dir_type = "dir ";
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
