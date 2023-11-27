// Define a tuple struct
#[derive(Debug)]
struct KeyPress(String, char);

// Define a classic struct
#[derive(Debug)]
struct MouseClick { x: i64, y: i64 }

// Define the WebEvent enum variants to use the data from the structs
// and a boolean type for the page Load variant
#[derive(Debug)]
enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }



//Structure types is given below from here!

struct Student {
    name: String,
    level: u8,
    remote: bool
}

struct Grades(char, char, char, char, f32);

// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic
}




fn main() {
    println!("Hello, world!");
    println!("\n");

    // todo!("Display the message by using the println!() macro");
    // Call println! with three arguments: a string, a value, a value
    println!("The first letter of the English alphabet is {} and the last letter is {}.", '1', 'Z');

let a_number;
    
let a_word = "Ten";
    
a_number = 10;

println!("The number is {}.", a_number);
println!("The word is {}.", a_word);

let shadow_num = 5;

let shadow_num = shadow_num + 5; 

let shadow_num = shadow_num * 2; 

println!("The number is {}.", shadow_num);


println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);
println!("\n");
println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
println!("\n");
let character_1: char = 'S';
let character_2: char = 'f';
   
let smiley_face = '😃';

let string_1 = "miley ";

let string_2: &str = "ace";

println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
println!("\n");

let tuple_e = ('E', 5i32, true, false, '🦍');

println!("Is '{}' the {}th letter of the alphabet? {} but she said that it will be {} because she has iq of a {}", tuple_e.0, tuple_e.1, tuple_e.2, tuple_e.3, tuple_e.4);
println!("\n");



    // Instantiate classic struct, specify fields in random order, or in specified order
    let user_1 = Student { 
        name: String::from("Anubhav is Gain"),
        remote: true,
        level: 2
    };
    
    let user_2 = Student {
        name: String::from("Gain is Anubhav"),
        level: 5,
        remote: false
    };
    
    // Instantiate tuple structs, pass values in same order as types defined  
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
        user_1.name, user_1.level, user_1.remote, 
        mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_2.name, user_2.level, user_2.remote,
        mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
        println!("\n");


    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);
        
    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);
        
    // Instantiate WebEvent enum variants
    // Set the boolean page Load value to true
    let we_load = WebEvent::WELoad(true);
    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);
        
    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);

    anubhav();

    //Pass input arguments
    let formal = "Formal: Goodbye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);

    // Return a value
    let num = 25;
    println!("\n");
    println!("{} divided by 5 = {}", num, divide_by_5(num));





    // Exercise: Write a function to build a car
    // We have orders for three new cars! 
  let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
  println!("\n");
  println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

  car = car_factory(String::from("Silver"), Transmission::Automatic, true);
  println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

  car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
  println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
  println!("\n");

  // Declare vector, initialize with three values
let three_nums = vec![15, 3, 46];
println!("Initial vector: {:?}", three_nums);  
  
// Declare vector, value = "0", length = 5
let zeroes = vec![0; 5];
println!("Zeroes: {:?}", zeroes); 
println!("\n");


//Boolen exercise in if else
let formal = true;
let greeting = if formal { // if used here as an expression
    "Good day to you."     // return a String
} else {
    "Hey!"                 // return a String
};
println!("{}", greeting);   // prints "Good day to you."


//Examples of hasmap B.
hashmap();
println!("\n");
hashmap2();
println!("\n");

//Examples for the for loop

let big_birds = ["ostrich", "peacock", "stork"];
for bird in big_birds.iter() {
    println!("The {} is a big bird.", bird);
    println!("\n");
}

for number in 0..5 {
    println!("{}", number * 2);
}

// //Panic gets the unbreakable error in rust
// panic!("Farewell!");


// let v = vec![0, 1, 2, 3];
// println!("{}", v[6]); // this will cause a panic!



//Use the Option type to deal with absence
let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

// pick the first item:
let first = fruits.get(0);
println!("\n");
println!("{:?}", first);

// pick the third item:
let third = fruits.get(2);
println!("{:?}", third);

// pick the 99th item, which is non-existent:
let non_existent = fruits.get(99);
println!("{:?}", non_existent);
println!("\n");

//eXAMPLE FOR THE PATTERN MATCHING 

let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
for &index in [0, 2, 99].iter() {
    match fruits.get(index) {
        Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
        None => println!("There is no fruit! :( \n"),
    }
}

//Example for the if loop for getting a number 
let a_number: Option<u8> = Some(7);
if let Some(7) = a_number {
    println!("That's my lucky number!\n");
}


}




//Functions goes down here 

fn anubhav() {
    println!("\n");
    println!("Created a new function here today! 25/11/2023.");
}

fn goodbye(message: &str) {
    println!("\n{}", message);
}

fn divide_by_5(num: u32) -> u32 {
    num / 5
}

// Exercise: Write a function to build a car
// Build a "Car" by using values from the input arguments 
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {

    // Use the values of the input arguments
    // All new cars always have zero mileage
    Car {
        color,
        transmission,
        convertible,
        mileage: 0
    }
}

//Function for hashmap
fn hashmap() {
    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Anubhav Gain"), String::from("Very accurate."));
    reviews.insert(String::from("Anubhav Gain"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

    // Look for a specific review
    let book: &str = "Anubhav Gain";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));

    
}

//function for hash map to create the values and get multiple output 

fn hashmap2() {
    use std::collections::HashMap; 

    let mut reviews = HashMap::new();

    reviews.entry("Anubhav Gain").or_insert(Vec::new()).push("Very accurate".to_string());
    reviews.entry("Anubhav Gain").or_insert(Vec::new()).push("Sweet recipes".to_string());
    reviews.entry("Programming in Rust").or_insert(Vec::new()).push("Great examples".to_string());

    for (key, values) in &reviews {
        if key == &"Anubhav Gain" {
            println!("{}:", key);
        for value in values {
            println!("- {}", value);
        }
    }
    }
}