use std::{collections::HashMap, hash::Hash};

fn main(){

    let scores: HashMap<String, u32> = HashMap::new();




}

fn highest(scores: HashMap<String, u32>) -> String{

    if scores.is_empty(){
        return "".to_string()
    }


    for pair in scores {
        
    }


}


// fn main() {

//     let sentence = "HashMap<String, u32>, item: &str, qty: u32) that increases the quantity for a given item — if the item doesn't exist yet in the map, it should be added with that quantity; if it already exists, the quantity should be idk man.";


//     let result = wordcount(sentence);  

//     println!("{:#?}", result); 

// }

// fn wordcount(sentence: &str) -> HashMap<String, u32> {

//     let mut map = HashMap::new();

//     for word in sentence.split_whitespace() {

//         let count = map.entry(word.to_string()).or_insert(0);
//         *count += 1;
//     }

//     return map;

// }

// fn main() { 
//     let mut inventory: HashMap<String, u32> = HashMap::new();

//     inventory.insert("macbookpro".to_string(), 3);

//     add_stock(&mut inventory, "macbookpro", &8);

//     for (key, value) in inventory {
//         println!("{}: {}", key, value);
//     }

// }

// fn add_stock (inventory: &mut HashMap<String, u32>, item: &str, qty: &u32){

//     // pull from inventory
//     let count = inventory.entry(item.to_string()).or_insert(0);
//     *count += 1;

// }




// fn main() {
//     let mut ages: HashMap<String, i32> = HashMap::new();

//     ages.insert(String::from("ade"), 12);
//     ages.insert(String::from("ife"), 14);



//     for (key, value) in ages {
//         println!("{}: {}", key, value);
//     }
// }


// fn main() { 

//     let mut ages: HashMap<String, i32> = HashMap::new();

//     ages.insert(String::from("ade"), 12);
//     ages.insert(String::from("ife"), 14);

//     let if_found = ages.get("ade");

//     match if_found {
//         None => {
//             println!("Not Found")
//         },

//         Some(value) => {
//             println!("Found {value}")
//         }
//     };
// }



// fn main() {

//     let mut scores: HashMap<String, i32> = HashMap::new();

//     scores.insert(String::from("Chinedu"), 24);
//     scores.insert(String::from("Micheal"), 28);
//     scores.insert(String::from("John"), 17);

//     let result = scores.get(&String::from("John")).unwrap();


//     println!("score is {result:?}");

// }



// // hashmap p 
// use std::collections::HashMap;

// fn main() {


//     let values = [10, 20, 20, 30, 40, 40, 40];

//     let mut map = HashMap::new();

//     // let mut scores = HashMap::new();

//     // scores.insert(String::from("Purple"), 20);
//     // scores.insert(String::from("Orange"), 10);
//     // scores.insert(String::from("Orange"), 10);
//     // scores.insert(String::from("Orange"), 10);



//     // println!("{scores:?}");

//     for value in values {
//         let count = map.entry(value).or_insert(0);
//         *count += 1;
//     }

//     println!("{map:?}");

// }

// fn main() { 
//     let hello = "Здравствуйте";


//     for c in hello.bytes(){
//         println!("{}", c);
//     }
// }


// fn main() { 
//     let mut origi = "Oluwa".to_string();

//     origi.push_str("Chinedu");

//     println!("{}", origi);
// }


// #[derive(Debug)]
// enum Sheet {
//     Int(i32),
//     Float(f64),
//     Text(String)
// }

// fn main() { 
    

//     let row = vec![
//         Sheet::Int(3),
//         Sheet::Float(1.9),
//         Sheet::Text(String::from("Olufemi"))

//     ];


//     for i in &row {
//         dbg!(i);
//     } 
// }