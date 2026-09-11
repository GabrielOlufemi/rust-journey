use std::hash::BuildHasher;

enum Shape {
    Circle(f64),
    Rectangle(f64,f64),
    Triangle(f64, f64)
}

impl Shape {
    fn area(&self) -> f64{
        match self {
            Shape::Circle(radius) => {
                radius * radius * std::f64::consts::PI
            },

            Shape::Rectangle(width, height) => {
                width * height
            },

            Shape::Triangle(base, height) => {
                0.5 * base * height
            }
        }
    }
}

fn main() {

    let new_shape = Shape::Circle(7.8);

    println!("{}", new_shape.area());

}

// enum PaymentMethod {
//     Cash,
//     Card {last_four: String},
//     MobileMoney {provider: String, number: String}
// }

// impl PaymentMethod {
//     fn describe(&self) -> String{
//         match self {
//             PaymentMethod::Cash => "Card method is nice i guess...".to_string(),
//             PaymentMethod::Card { last_four } => {
//                 format!("these are your last four digits {}", last_four)
//             }, 
//             PaymentMethod::MobileMoney { provider, number } => {
//                 format!("provider: {},  number: {}", provider, number)
//             }
//         }
//     }
// }

// fn main() {
//     // returns string defining it 

//     let pmethod = PaymentMethod::MobileMoney { 
//         provider: "mtn".to_string(),
//         number: "09063568438".to_string(),
//     };

//     println!("{}", pmethod.describe());

// }

// #[derive(Debug)]

// enum TrafficLight {
//     Red,
//     Yellow,
//     Green
// }

// impl TrafficLight {
//     fn next(&self) -> TrafficLight{
//         match self{
//             TrafficLight::Red => TrafficLight::Yellow,
//             TrafficLight::Yellow => TrafficLight::Green,
//             TrafficLight::Green => TrafficLight::Red,
//         }
//     }
// }

// fn main() {
//     let light1: TrafficLight = TrafficLight::Green;

//     println!("{:?}", light1.next());
// }


// struct Circle {
//     radius: f64
// }

// impl Circle {
//     fn unit_circle() -> Circle{
//         Circle {
//             radius: 2.0,
//         }
//     }

//     fn scale(&mut self, factor:f64){
//         self.radius = self.radius * factor
//     }
// }

// fn main(){
//     let mut circ1 = Circle::unit_circle();
//     circ1.scale(4.0);
// }


// struct Book {
//     title: String,
//     author: String,
//     pages: u32,
//     finished: bool,
// }

// impl Book {
//     fn new(title: String, author: String, pages: u32) -> Book{
//         Book {
//             title: title,
//             author: author,
//             pages: pages,
//             finished: false,
//         }
//     }

//     fn mark_finished(&mut self){
//         self.finished = true;
//     }
// }

// fn main(){

//     let mut book1 = Book {
//         title: String::from("burti"),
//         author: String::from("mavoswago"),
//         pages: 13,
//         finished: false
//     };
// }

// enum Direction {
//     North, 
//     West,
//     South,
//     East
// }

// fn describe(coords: Direction) -> &'static str {
//     match coords {
//         Direction::North => "North is nice",
//         Direction::West => "Fly god west",
//         Direction::South => "South south",
//         Direction::East => "easter bunny"
//     }
// }

// fn main(){

//     // let coordinates = Direction::North;

//     let result2 = describe(Direction::South);

//     println!("{}", result2);

// }


// struct Point {
//     x: f64,
//     y: f64
// }

// fn distance_between(point: Point) -> f64 {
//     point.x - point.y
// }


//fn main(){

    // let book1 = Book {
    //     title: String::from("burti"),
    //     author: String::from("mavoswago"),
    //     pages: 13,
    //     digital: false
    // };

    // let point1 = Point {
    //     x: 17.3,
    //     y: 12.9
    // };

    // let result = distance_between(point1);

    // println!("Distance between both points is {}", result);


// struct Rectangle {



//}

// struct Rectangle {
//     width: u32,
//     height: u32
// }


// fn main() { 
//     let rect1 = Rectangle {
//         width: 20,
//         height: 15
//     };

//     println!("Our rectangles details are {:?} and {:?}", 
//         rect1.width, 
//         rect1.height
//     )
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }





// struct User {
//     username : String,
//     email : String,
//     sign_in_count : u64,
//     active : bool,
// }

// fn main() {
//     // creating a user 
//     let user1 = User {
//         username: String::from("Gabriel"),
//         email: String::from("kusorogabriel@gmail.com"),
//         sign_in_count: 14,
//         active: true
//     };

//     println!("User 1 name is {}, email is {}", 
//     user1.username,
//     user1.email,
//     );


//     let user2 = User {
//         username: String::from("Chinedu"),
//         email: String::from("chichi@gmail.com"),
//         ..user1
//     };

//     println!("User 2 name is {}, email is {}", 
//         user2.username,
//         user2.email,
//     );

// }




// // // Let’s say we want to write a function that takes an Option<i32> and, 
// // // if there’s a value inside, adds 1 to that value. 
// // // If there isn’t a value inside, the function should return the None value 
// // // and not attempt to perform any operations.

// // fn main() { 


// //     let ex1 = None;
// //     let ex2 = Some(5);

// //     println!("example one + 1 : {:?}", plus_one(ex1));
// //     println!("example two + 1 : {:?}", plus_one(ex2));

// // }

// // fn plus_one(val: Option<u32>) -> Option<u32> {
// //     match val {
// //         None => None,
// //         Some(i) => Some(i+1)
// //     }
// // }