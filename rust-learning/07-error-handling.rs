// ============================================
// LEÇON 7: GESTION DES ERREURS
// ============================================
// Rust n'a pas d'exceptions, mais Result et Option

use std::fs::File;
use std::io::{self, Read};

fn main() {
    // === OPTION ===
    let fruits = vec!["apple", "banana", "cherry"];

    match fruits.get(0) {
        Some(fruit) => println!("First fruit: {}", fruit),
        None => println!("No fruits"),
    }

    match fruits.get(10) {
        Some(fruit) => println!("Fruit at 10: {}", fruit),
        None => println!("No fruit at that index"),
    }

    // === UNWRAP (attention!) ===
    let number = Some(42);
    let value = number.unwrap();  // Panic si None!
    println!("Value: {}", value);

    // let nothing: Option<i32> = None;
    // let value = nothing.unwrap();  // ❌ PANIC!

    // === UNWRAP_OR (valeur par défaut) ===
    let nothing: Option<i32> = None;
    let value = nothing.unwrap_or(0);  // Retourne 0 si None
    println!("Value: {}", value);

    // === RESULT ===
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // === ? OPERATOR (propagation d'erreur) ===
    match read_file_to_string("example.txt") {
        Ok(contents) => println!("File: {}", contents),
        Err(e) => println!("Error: {}", e),
    }

    // === IF LET (pattern matching court) ===
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let number = Some(42);
    if let Some(n) = number {
        println!("The number is: {}", n);
    } else {
        println!("No number");
    }

    // === MAP ET AND_THEN ===
    let maybe_number = Some(5);
    let doubled = maybe_number.map(|x| x * 2);
    println!("Doubled: {:?}", doubled);  // Some(10)

    let result = divide(20, 4);
    let squared = result.map(|x| x * x);
    println!("Squared: {:?}", squared);  // Ok(25)

    // === CHAÎNER AVEC ? ===
    match read_and_parse("42") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }

    // === CUSTOM ERROR HANDLING ===
    match validate_age(25) {
        Ok(age) => println!("Valid age: {}", age),
        Err(e) => println!("Invalid: {}", e),
    }

    match validate_age(-5) {
        Ok(age) => println!("Valid age: {}", age),
        Err(e) => println!("Invalid: {}", e),
    }
}

// === FONCTION RETOURNANT RESULT ===
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// === FONCTION AVEC ? OPERATOR ===
fn read_file_to_string(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;  // Retourner si erreur
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Parsing avec propagation d'erreur
fn read_and_parse(input: &str) -> Result<i32, String> {
    let number = input
        .trim()
        .parse::<i32>()
        .map_err(|_| String::from("Failed to parse"))?;
    Ok(number * 2)
}

// === VALIDATION PERSONNALISÉE ===
fn validate_age(age: i32) -> Result<i32, String> {
    if age < 0 {
        Err(String::from("Age cannot be negative"))
    } else if age > 150 {
        Err(String::from("Age seems unrealistic"))
    } else {
        Ok(age)
    }
}

// === ENUM CUSTOM ERROR ===
enum ParseError {
    InvalidFormat,
    OutOfRange,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "Invalid format"),
            ParseError::OutOfRange => write!(f, "Value out of range"),
        }
    }
}

fn parse_with_custom_error(s: &str) -> Result<u32, ParseError> {
    s.trim()
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidFormat)
}

// CONCEPTS CLÉS:
// 1. Option<T>
//    - Some(T) → valeur présente
//    - None → pas de valeur
//    - unwrap() → panicker si None
//    - unwrap_or(default) → valeur par défaut
//    - map() → transformer la valeur
//
// 2. Result<T, E>
//    - Ok(T) → succès
//    - Err(E) → erreur
//    - ? → propager l'erreur
//    - map_err() → transformer l'erreur
//
// 3. ? Operator
//    - Retourner l'erreur si elle existe
//    - Syntaxe courte pour Error handling
//
// 4. if let
//    - Pattern matching court
//    - for Option et Result
//
// 5. Custom Errors
//    - Créer tes propres types d'erreur
//    - Implémenter Display trait
