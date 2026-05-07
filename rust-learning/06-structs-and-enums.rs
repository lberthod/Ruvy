// ============================================
// LEÇON 6: STRUCTS ET ENUMS
// ============================================
// Créer des types personnalisés pour organiser tes données

fn main() {
    // === STRUCTS ===
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };

    println!("Nom: {}", person.name);
    println!("Âge: {}", person.age);
    person.display();

    // === STRUCT MUTABLE ===
    let mut user = User {
        username: String::from("bob"),
        active: true,
    };
    user.active = false;
    println!("User active: {}", user.active);

    // === STRUCT SHORTHAND ===
    let name = String::from("Charlie");
    let email = String::from("charlie@example.com");
    let person2 = Person {
        name,     // Même que name: name
        age: 25,
        email,    // Même que email: email
    };
    println!("{}", person2.name);

    // === ENUMS ===
    let direction = Direction::Up;
    match direction {
        Direction::Up => println!("Aller vers le haut"),
        Direction::Down => println!("Aller vers le bas"),
        Direction::Left => println!("Aller vers la gauche"),
        Direction::Right => println!("Aller vers la droite"),
    }

    // === ENUMS AVEC DONNÉES ===
    let message = Message::Text(String::from("Hello"));
    process_message(message);

    let message = Message::Move { x: 10, y: 20 };
    process_message(message);

    let message = Message::Quit;
    process_message(message);

    // === OPTION (Enum courant) ===
    let some_value: Option<i32> = Some(42);
    match some_value {
        Some(value) => println!("Value: {}", value),
        None => println!("Aucune valeur"),
    }

    let no_value: Option<i32> = None;
    match no_value {
        Some(value) => println!("Value: {}", value),
        None => println!("Aucune valeur"),
    }

    // === RESULT (Enum courant) ===
    let result: Result<i32, String> = Ok(42);
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // === MÉTHODES SUR STRUCTS ===
    let mut rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    println!("Area: {}", rect.area());
    println!("Can hold: {}", rect.can_hold(&Rectangle { width: 2.0, height: 3.0 }));
    rect.scale(2.0);
    println!("After scaling: {} x {}", rect.width, rect.height);
}

// === STRUCT DÉFINITION ===
struct Person {
    name: String,
    age: u8,
    email: String,
}

// === IMPL: Ajouter des méthodes à la struct ===
impl Person {
    fn display(&self) {
        println!("Person: {}, age {}", self.name, self.age);
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

struct User {
    username: String,
    active: bool,
}

// === ENUM DÉFINITION ===
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// === ENUM AVEC DONNÉES ===
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Text(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(message: Message) {
    match message {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Text(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
    }
}

// === STRUCT POUR GÉOMÉTRIE ===
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

// CONCEPTS CLÉS:
// 1. Struct
//    - Grouper des données liées
//    - Accès par nom: struct.field
//
// 2. Impl
//    - Ajouter des méthodes à une struct
//    - &self → lecture
//    - &mut self → modification
//    - Pas de self → fonction associée
//
// 3. Enum
//    - Énumération de variantes
//    - Peut avoir des données associées
//    - Pattern matching pour traiter
//
// 4. Option<T>
//    - Some(T) ou None
//    - Remplace null (qui n'existe pas en Rust)
//
// 5. Result<T, E>
//    - Ok(T) ou Err(E)
//    - Pour les opérations qui peuvent échouer
