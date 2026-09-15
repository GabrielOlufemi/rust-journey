// fn handle_remove_command (maps: &mut Vec<String>, index: &str) -> Result<String, String>{ 


//     let index = index.parse::<u32>().map_err(|e| format!("an error has occured {}", e))?;

//     remove_todo(maps, index as usize)


// }

//use std::collections::HashMap;

// fn mark_complete (map: &mut HashMap<String, bool>, name: &str) -> Result<(), String> {

//     // for (key, value) in map 
//     //     if key == name.to_string() {
//     //         *value = true;
//     //     }else {
//     //         return Err("todo wasn't found".to_string())
//     //     }
//     // }

//     // Ok(())

//     if let Some(value) = map.get_mut(name) { 
//         *value = true;
//         Ok(())
//     } else {
//         Err("todo wasn't found".to_string())
//     }

// }


use std::io::{self,Error};

fn remove_todo(tasks: &mut Vec<String>, index: usize) -> Result<String, String> {

    if index >= tasks.len() {
        return Err("index out of bounds".to_string())
    }

    Ok(tasks.remove(index))
}


// use std::io::{self, Error, ErrorKind, Read};

// fn get_todo(index: usize, container: &Vec<String>) -> Option<&String> {

//     container.get(index)

// }

// use std::fmt::format;
// use std::io::ErrorKind;
// use std::io::Error;
// use std::io;



// fn parse_index(value: &str) -> Result<u32, String> {

//     let converted = value.parse::<u32>().map_err(|e| format!("An error occured {}", e))?;

//     Ok(converted)
// }


// use std::fs::File;
// use std::io::{self, Read};

// fn read_username() -> Result<String, io::Error> {

//     let mut username_file = File::open("username_doc.txt")?;

//     let mut username = String::new();

//     username_file.read_to_string(&mut username)?;


//     Ok(username)

// }

// fn read_username() -> Result<String, io::Error> {

//     let username_file_result = File::open("username_doc.txt");

//     //receives username file
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e)
//     }

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e)
//     }

// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {

//     let hello_file = File::open("hello.txt");

//     let result = match hello_file {
//         Ok(file) => file,

//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt"){
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Error creating the file, {e}")
//             }, 
//             _ => panic!("Error opening the file, {error:?}")
//         } 
//     };

// }