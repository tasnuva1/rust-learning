// -------------------------------------------------------version 1---------------------------------------------------------------------
// use std::env::args;
// fn main() {
//     // key value database app
//     // handle the arguments
//     // create a apscraption to manage it
//     // functionality to added, view it in a data sturecture
//     // write the sturecture data to disk file

//     // let key = args().nth(1).expect("key is not there.");
//     let key = args().skip(1).next().expect("key is not there.");
//     let value = args().skip(2).next().expect("value is not there.");
//     // println!("this is {} , {}", key, value);

//     let onetime = Database::new(key, value); // I store it and it's gone when quiting and the whole thing is just one time thing.

//     onetime.view();
// }

// struct Database {
//     key: String,
//     value: String,
// }

// impl Database {
//     fn new(key: String, value: String) -> Self {
//         Self { key, value }
//     }
//     fn view(self) {
//         let forment = format!("this is key: {} . this is value: {}", self.key, self.value);
//         println!("{}", forment);
//     }
// }
// -------------------------------------------------------version 2---------------------------------------------------------------------

// use std::{collections::HashMap, env::args, fs::File};
// fn main() {
//     // key value database app
//     // handle the arguments
//     // write the data to disk file to store it
//     // read the file then added to the data sturecture
//     // For tracking the stored data with functionality.
//     // create a apscraption to manage different functionality
//     // functionality to added, view it in a data sturecture

//     let key = args().skip(1).next().expect("key is not there.");
//     let value = args().skip(2).next().expect("value is not there.");
//     let conant = format!("{}\t{}\n", key, value);
//     // std::fs::write("bar.txt", conant).expect("Cant not write");

//     let mut file = match File::create_new("database.txt") {
//         Ok(file) => file
//             .write_all(conant.as_bytes())
//             .expect("Writing, give you and error"),
//         Err(err) => std::fs::write("database.txt", conant).expect("Can not write to this file"),
//     }; // Err type is file exist.
// }

// struct Database {
//     key_value: HashMap<String, String>,
// }

// impl Database {
//     fn new(key: String, value: String) -> Database<String, String> {
//         // functionality to added in a data sturecture
//         // read the file then added to the data sturecture
//         Database {
//             key_value: HashMap::new(key, value),
//         }
//     }
// }

// -------------------------------------------------------version 3---------------------------------------------------------------------

// use std::{collections::HashMap, env::args, fs::File};
// fn main() {
//     // key value database app
//     // handle the arguments
//     // store it to the disk for data presistens
//     // viewing in a sturactural way
//     // first need to add the stored data to the data stsuraction for sturactureal view

//     let key = args().skip(1).next().expect("key is not there.");
//     let value = args().skip(2).next().expect("value is not there.");
//     let content = format!("{}\t{}\n", key, value);
//     // std::fs::write("bar.txt", conant).expect("Cant not write");

//     // let is_file_exit = std::path::Path::new("does_not_exist.txt").exists()
//     // if is_file_exit == false {
//     // }
//     std::fs::write("database.txt", content).expect("Can not write to this file or file do exit")
//     let key_value = Database::new();
// }

// struct Database {
//     key_value: HashMap<String, String>,
// }

// impl Database {
//     fn new() -> Database {
//         // To view it in a sturactural way
//         // > First need to, add presistens data to the data stsuraction
//         let mut data_sturature = HashMap::new();
//         let contents = std::fs::read_to_string("database.txt")
//             .expect("Should have been able to read the file");

//         for line in contents.lines() {
//             let mut chanks = line.splitn(2, "\t");
//             let key = chanks.next().expect("currapted file");
//             let value = chanks.next().expect("currapted file");
//             data_sturature.insert(key.to_owned(), value.to_owned());
//         }
//         Database {
//             key_value: data_sturature,
//         }
//     }
// }
// -------------------------------------------------------version 4-----------------------------------------------------------------
use std::{collections::HashMap, env::args, fs::File};

fn main() {
    // key value database app
    // handle the arguments

    // First store the privice stored data into the runtime data stureture.
    // The handled key value argument store into the runtime data stureture.

    // insart all the stored data to data sturcutue. (in mamory)
    // then instart handled arguments to the data sturecute. (in mamory)
    // then take the contents form the data stureture, and write it to the disk.

    let key = args().skip(1).next().expect("key is not there.");
    let value = args().skip(2).next().expect("value is not there.");

    let mut key_value = Database::new(); // disk load to mamory
    key_value.insert(key.to_uppercase(), value.to_uppercase());
    key_value.insert(key, value);
    key_value.load_mamory_to_disk(); // overwrite the whole file // logs/HEAD -> hash name
}

struct Database {
    key_value: HashMap<String, String>,
}

impl Database {
    fn new() -> Database {
        let mut data_sturature = HashMap::new();

        let contents = std::fs::read_to_string("database.txt")
            .expect("Should have been able to read the file");

        for line in contents.lines() {
            let mut chanks = line.splitn(2, "\t");
            let key = chanks.next().expect("currapted file");
            let value = chanks.next().expect("currapted file");
            data_sturature.insert(key.to_owned(), value.to_owned());
        }
        Database {
            key_value: data_sturature,
        }
    }
    fn insert(&mut self, key: String, value: String) {
        self.key_value.insert(key, value);
    }

    fn load_mamory_to_disk(self) {
        let mut content = String::new();
        for (key, value) in self.key_value {
            // let key = each.0;
            // let value = each.1;
            content.push_str(&key);
            content.push('\t');
            content.push_str(&value);
            content.push('\n');
            // let pair = format!("{}\t{}\n", key, value);
            // content.push_str(pair);
        }
        std::fs::write("database.txt", content)
            .expect("Can not write to this file or file do exit.");
    }
}

// overwriting is much faster than appending for most filesystems/storages. Overwriting does not change file size, while appending does.
