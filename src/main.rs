fn main() {
    println!("Hello, world!");
}

trait GetValue<T> where T: PartialOrd {
    fn get_value(&self) -> &T;
}

struct Point<T = i32> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_y(&self) -> &T {
        &self.y
    }

    fn get_x(&self) -> &T {
        &self.x
    }
}

impl<T> GetValue<T> for Point<T> where T: PartialOrd {
    fn get_value(&self) -> &T {
        &self.x
    }
}


#[test]
fn test_struct_generics_default_in_method_and_trait() {
    //default generics type
    let age = Point{x: 80, y: 60};

    let my_name: Point<i32> = Point::<i32> { x: 20, y: 20 };

    let my_geo_location: Point<f64> = Point::<f64> { x: 121.31313, y: 131.13131};

    let name = Point{x: "khairul".to_string(), y: "aswad".to_string()};

    println!("{} {}", age.get_x(), age.get_y());

    println!("{}", my_name.get_x());
    println!("{}", my_geo_location.get_y());
    println!("{}", name.get_value());

    println!("{}", name.get_x());
    println!("{}", name.get_y());

    println!("{} {}", my_name.x, my_name.y);
    println!("{} {}", my_geo_location.x, my_geo_location.y);
}

enum Value<T> {
    NONE,
    VALUE(T),
}

#[test]
fn test_enum_generics() {
    let none_value = Value::<i32>::NONE;
    let value: Value<String> = Value::<String>::VALUE("khairul".to_string());

    match value {
        Value::NONE => {
            println!("none");
        },
        Value::VALUE(value) => {
            println!("value: {}", value);
        },
    }

    match none_value {
        Value::NONE => {
            println!("none");
        },
        Value::VALUE(value) => {
            println!("value: {}", value);
        },
    }
}

trait CanSayHello {
    fn say_hello(&self) -> String;
}

trait CanSayGoodBye {
    fn say_good_bye(&self) -> String;
}

struct SimpleMan {
    name: String,
}

impl CanSayHello for SimpleMan {
    fn say_hello(&self) -> String {
        format!("hello, my name is {}", self.name)
    }
}

struct SimplePerson {
    name: String,
}

impl CanSayHello for SimplePerson {
    fn say_hello(&self) -> String {
        format!("hallo from {}", self.name)
    }
}

impl CanSayGoodBye for SimplePerson {
    fn say_good_bye(&self) -> String {
        format!("goodbye {}", self.name)
    }
}

struct Hi<T = SimpleMan> where T: CanSayHello {
    value: T,
}

struct Bye<T = SimplePerson> where T: CanSayGoodBye {
    value: T,
}

#[test]
fn test_generic_type_bound_and_generics_default_value() {

    let default_value = Hi{
        value: SimpleMan {
            name: "default value".to_string()
        }
    };

    println!("{}", default_value.value.say_hello());

    let default_valu2 = Bye{
        value: SimplePerson {
            name: "default value".to_string()
        }
    };

    println!("{}", default_valu2.value.say_good_bye());

    let hi = Hi::<SimpleMan> {
        value: SimpleMan {
            name: "khairul".to_string()
        }
    };
    
    println!("{}", hi.value.say_hello());
    
    let bye: Bye<SimplePerson> = Bye::<SimplePerson> {
        value: SimplePerson { name: "simple person".to_string() },
    };

    println!("{}", bye.value.say_good_bye());

    let hi2: Hi<SimplePerson> = Hi::<SimplePerson>{ value: SimplePerson { name: "khairul".to_string() } };

    println!("hi2: {}", hi2.value.say_hello());
    println!("hi2: {}", hi2.value.say_good_bye());
}

fn min<T>(value1: T, value2: T) -> T where T: PartialOrd {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

fn duplicated<T>(value1: T, value2: T) -> bool where T: PartialEq {
    value1 == value2
}

#[test]
fn generic_in_function() {
    let result = min::<i32>(30, 31);
    println!("result: {}", result);

    let result = duplicated::<i32>(10, 10);

    println!("result duplicated number: {}", result);
}

fn say_my_name<T>(value: T) -> T {
    value
}

#[test]
fn test_generic_function() {
    let result = say_my_name::<String>("khairul".to_string());

    println!("{}", result);

    let result = say_my_name::<i32>(80);

    println!("{}", result);
}

struct Apple {
    quantity: i32,
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

use std::cmp::Ordering;

impl PartialOrd for  Apple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

use std::collections::{LinkedList, VecDeque};
use std::ops::Add;

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity
        }
    }
}

#[test]
fn test_operator_add() {
    let apple1 = Apple{ quantity: 10 };
    let apple2 = Apple{ quantity: 20 };

    let apple3 = apple1 + apple2;

    println!("{:?}", apple3.quantity);
}

#[test]
fn feature() {
    let apple1 = Apple{ quantity: 10 };
    let apple2 = Apple{ quantity: 20 };

    println!("apple1 == apple2 : {}", apple1 == apple2);
    println!("apple1 > apple2 : {}", apple1 > apple2);
    println!("apple1 < apple2 : {}", apple1 < apple2);
    println!("apple1 != apple2 : {}", apple1 != apple2);
}


fn double(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x * 2)
    }
}

#[test]
fn test_option_value() {
    let y  = double(None);

    match y {
        Some(v) => println!("{}", v),
        None => println!("none value my bruda")
    }

    let x = double(Some(2));

    match x {
        Some(v) => println!("{}", v),
        None => println!("none value bri bruda")
    }

    let z: Option<i32> = double(Option::Some::<i32>(80));

    println!("{:?}", z);
}

#[test]
fn test_string_manipulation() {
    let my_name = String::from("Khairul Aswad");

    println!("{}", my_name.to_lowercase());
    println!("{}", my_name.to_uppercase());
    println!("{}", my_name.len());
    println!("{}", my_name.replace("Khairul", "Aswad"));
    println!("{}", my_name.contains("Khairul"));
    println!("{}", my_name.starts_with("Kha"));
    println!("{}", my_name.ends_with("ad"));
    println!("{}", my_name.trim());
    println!("{:?}", my_name.get(0..5));
}

struct Category {
    id: String,
    name: String,
}

use std::fmt::{Debug, Formatter, Result};

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Category")
        .field("id", &self.id)
        .field("name", &self.name)
        .finish()
    }
}

#[test]
fn test_format() {
    let food_category: Category = Category { id: "12323".to_string(), name: "food".to_string() };

    println!("{:?}", food_category);
}

#[test]
fn test_closure() {
    let sum: fn(i32, i32) -> i32 = |value1: i32, value2: i32| -> i32 {
        value1 + value2
    };

    let result = sum(7, 7);

    println!("{}", result);
}

fn print_with_filter(value: String, filter: fn(String) -> String) {
    let result = filter(value);
    println!("print with filter: {}", result);
}

fn filter(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn test_closure_as_parameter() {
    print_with_filter(String::from("Khairul Aswad"), filter);

    print_with_filter("alok".to_string(), |value: String| -> String {
        value.add("halo dek")
    });
}

#[test]
fn test_closure_scope() {
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
        println!("Increment");
    };

    increment();
    increment();
    increment();

    println!("Counter: {}", counter);
}

struct Counter {
    counter: i32
}

impl Counter {
    fn increment(&mut self) {
        self.counter += 1;
        println!("Increment")
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter{ counter: 0 };

    counter.increment();
    counter.increment();
    counter.increment();

    println!("Counter: {}", counter.counter);
}

#[test]
fn test_vector() {

    let array_value  = ["khairul", "asawad", "tembak", "tembak"];

    for value in array_value {
        println!("{}", value);
    }

    println!("{:?}", array_value);

    let mut names: Vec<String> = Vec::<String>::new();

    names.push(String::from("Khairul Aswad"));

    names.push(String::from("Aswad Khairul"));

    names.pop();

    for name in &names {
        println!("{}", name);
    }
    
    println!("{:?}", names);
    println!("{}", names[0]);
}

#[test]
fn test_vector_deque() {
    let mut names: VecDeque<String> = VecDeque::<String>::new();

    names.push_front(String::from("middle_value"));

    names.push_back(String::from("back_value"));

    names.push_front(String::from("front_value"));

    for value in &names {
        println!("{}", value);
    }

    println!("{:?}", names);

    println!("{}", names[2]);
}

#[test]
fn test_linked_list() {
    let mut names: LinkedList<String> = LinkedList::<String>::new();

    names.push_front(String::from("middle_value"));

    names.push_back(String::from("back_value"));

    names.push_front(String::from("front_value"));

    for value in &names {
        println!("{}", value);
    }

    println!("{:?}", names);
}