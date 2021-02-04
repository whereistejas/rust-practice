use std::string::String;

fn main() {
    let str1 = String::from("Luke Skywalker");
    let str2 = String::from("Anakin Skywalker");
    
    let result = longest_string(&str1, &str2);

    println!("longest string: {}", result);

    println!("string1: {}", str1);
}

fn longest_string<'a> (str1: &'a String, str2: &'a String) -> &'a String {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
