struct player {
    name: String, 
    score: i32 
}




// struct User<T> {
//     name: T
// }

// impl<T> User<T> {
//     fn get_name(&self) -> &T{
//         &self.name
//     }
// } 

// impl<T> Point<T>

// where T: std::fmt::Display,
// {
//     fn printresult(&self) {
//         println!("This is a {}", self.result)
//     }
    
// }

// fn main() {
//     let new_point = Point {
//         result : String::from("chinedu")
//     };

//     new_point.printresult();
// }

// struct Point { x: i32, y: i32 }

// fn main() {
//     let p1 = Point { x: 1, y: 2 };
//     let p2 = &p1;
//     println!("{}", p1.x);
// }


// fn multiply<'a>(a: &'a i32, b: &'a i32) -> i32 {

//     *a * *b

// }

// fn returnIndex<'a>(scores: &'a List[i32], index: i32) -> &'a String {

//     sentence.as_str()[index as usize]

// }

// use std::vec;

// fn count_words(s: &String) -> usize{

//     let mut word = String::new();
//     let mut words: Vec<String> = Vec::new();

//     for char in s.chars() {

//         if char.is_whitespace(){
//             if !word.is_empty(){
//                 words.push(std::mem::take(&mut word));
//             }

//         } else {
//             word.push(char);
//         }
//     }

//     if !word.is_empty(){
//         words.push(word);
//     }

//     words.len()
// }


// fn truncate(s: &String, stop: u32) -> String {

//     let mut word = String::new();

//     if stop == 0{
//         return word
//     }

//     for (index, char ) in s.chars().enumerate() {
//         word.push(char);


//         if index == (stop-1) as usize {
//             return word
//         }   
//     }

//     word
// }


// fn main() {

//     let example = "   I love me some fishes".to_string();

//     let result = count_words(&example);

//     println!("no of words is {}", result);


// }



// fn second_word (sentence: &String) -> usize {

//     let sentence = sentence.as_bytes();

//     let mut space_count = 0;

//     for (i, &item) in sentence.iter().enumerate() {

//         if item == b' ' {
//             space_count += 1;

//             if space_count == 2 {
//                 return i;
//             }
//         }  

//     }

//     sentence.len()
// }

// fn main() {
//     let example = "hello world";

//     let stuff = example.as_bytes();

//     let result = example.len();

//     println!("the length is {}", result);
//     println!("stuff as bytes {:#?}", stuff);
//     println!("slice: {:#?}", &stuff[..]);

// }

// fn main() {

//     let example = ["hello, my name is chinedu"];

//     for item in example{
//         println!("{}", item);
//     }
// }

// fn first_word(s: &String) -> usize{

//    let s = s.as_bytes();

//    for (i, &item) in s.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//    }  

//    s.len()

// } 

// fn consume(v: &Vec<i32>) -> usize {
//     v.len()
// }

// fn main() {
//     let data = vec![1, 2, 3];
//     let n = consume(&data);
//     println!("{} {}", n, data.len());
// }

// // Whether it compiles. If it doesn't, name the line and the rule being broken, using "owner" and "moved".
// // Two different fixes that use different tools. One of them shouldn't be .clone(). Tell me which you'd choose.
// // If data were let data = 5; and consume took an i32, would it compile? Why is that different