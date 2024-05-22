use std::collections::HashMap;
use std::fs::File;
use std::env;
use std::io::{Read};

fn main() {
    let arg: Vec<String> = env::args().collect();
    let action = &arg[1];
    let file_dir = &arg[2];

    if action == "read" {
        println!("$ {}, {}", action, file_dir);

        let frequency_map = get_and_read_file(String::from(file_dir));
        for (c, char_count) in frequency_map {
            println!("{} - {}", c, char_count);
        }

    } else {
        println!("No such action");
    }

}

fn get_and_read_file(file_dir: String) -> HashMap<char, i32> {

    // reading the file
    let mut file = File::open(file_dir)
        .unwrap_or_else(|e| {
            panic!("Failed to open file: {}", e);
        });
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut frequency_map = HashMap::new();
    for c in contents.chars() {
        let char_count = frequency_map.entry(c).or_insert(0);
        *char_count += 1;
    };
    return frequency_map;

}

#[cfg(test)]
mod tests {
    use crate::get_and_read_file;
    #[test]
    fn test_get_and_read_file() {
        let result = get_and_read_file(String::from("test.txt"));
        assert_eq!(result.get(&'t'), Some(&2i32))
    }

}
