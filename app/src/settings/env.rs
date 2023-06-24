// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

// pub fn main() {
//     let path = Path::new("file.txt");
//     let display = path.display();

//     let mut file = match File::open(&path) {
//         Ok(file) => file,
//         Err(_) => {
//             let mut new_file = match File::create(&path) {
//                 Ok(new_file) => new_file,
//                 Err(e) => panic!("Could not create file {}: {}", display, e),
//             };

//             let sample_file = "This is a sample file.";
//             match new_file.write_all(sample_file.as_bytes()) {
//                 Ok(_) => println!("File created and written successfully."),
//                 Err(e) => panic!("Could not write to file {}: {}", display, e),
//             }

//             return;
//         }
//     };

//     let mut contents = String::new();
//     match file.read_to_string(&mut contents) {
//         Ok(_) => println!("File contents: {}", contents),
//         Err(e) => panic!("Could not read file {}: {}", display, e),
//     }
// }
