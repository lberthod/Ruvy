# 🦀 Rust Cheat Sheet - Référence Rapide

## Syntaxe de Base

```rust
// Commentaire une ligne
/* Commentaire
   multi-ligne */

// Variables
let x = 5;              // Immuable
let mut y = 10;         // Mutable
const MAX: i32 = 100;   // Constante au niveau du programme

// Types
let i: i32 = -42;       // Entier signé
let u: u32 = 42;        // Entier non-signé
let f: f64 = 3.14;      // Flottant
let b: bool = true;     // Booléen
let c: char = 'A';      // Caractère (Unicode)
let s: &str = "Hello";  // String slice (référence)
let s: String = String::from("Hello"); // Chaîne propriétaire

// Affichage
println!("Hello {}", name);      // Affichage formé
println!("{:?}", vec);            // Debug
println!("{:#?}", vec);           // Pretty debug
dbg!(x);                          // Macro debug

// Opérations
let sum = 5 + 6;        // Addition
let diff = 5 - 6;       // Soustraction
let prod = 5 * 6;       // Multiplication
let div = 5 / 2;        // Division
let rem = 5 % 2;        // Modulo
let pow = 2_u32.pow(3); // Puissance (8)

// Booléen
let b = true && false;  // ET
let b = true || false;  // OU
let b = !true;          // NON
```

## Structures de Contrôle

```rust
// If / Else
if x > 0 {
    println!("positif");
} else if x < 0 {
    println!("négatif");
} else {
    println!("zéro");
}

// If comme expression
let y = if condition { 5 } else { 6 };

// Loop (infinie)
loop {
    if done { break; }
    println!("Infini");
}

// Break avec valeur
let result = loop {
    break 42;  // Retourne 42
};

// While
while x < 10 {
    x += 1;
}

// For
for i in 0..10 { }        // 0 à 9
for i in 0..=10 { }       // 0 à 10
for item in &vec { }      // Itérer une référence
for (idx, item) in vec.iter().enumerate() { }  // Avec indice

// Match
match value {
    1 => println!("Un"),
    2 | 3 => println!("Deux ou trois"),
    4..=10 => println!("4 à 10"),
    x if x > 10 => println!("Plus de 10"),
    _ => println!("Autre"),
}

// If let (pattern court)
if let Some(x) = maybe_value {
    println!("Value: {}", x);
}

// While let
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

## Fonctions

```rust
// Fonction simple
fn hello() {
    println!("Hello!");
}

// Avec paramètres
fn add(a: i32, b: i32) -> i32 {
    a + b  // Pas de ; = return implicite
}

// Avec plusieurs retours
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

// Expression dans la fonction
fn calculate(x: i32) -> i32 {
    let y = {
        let z = x + 1;
        z * 2  // Pas de ;
    };
    y + 1
}

// Closure (fonction anonyme)
let add = |a, b| a + b;
let result = add(5, 3);

// Closure avec type explicite
let add = |a: i32, b: i32| -> i32 { a + b };

// Closure qui capture l'environnement
let x = 5;
let add_x = |y| y + x;  // Capture x
```

## Collections

```rust
// Array (taille fixe)
let arr = [1, 2, 3, 4, 5];
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first = arr[0];

// Vector (taille dynamique)
let mut vec = vec![1, 2, 3];
vec.push(4);
let len = vec.len();
let first = vec[0];

// Chaîne
let mut s = String::new();
s.push_str("Hello");
s.push(' ');
s.push_str("World");
let s2 = String::from("Hello");

// Tuple
let tuple = (42, 3.14, true, "Hello");
let (a, b, c, d) = tuple;  // Destructuring
let value = tuple.0;        // Accès par indice

// HashMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value");
let value = map.get("key");  // Option<&V>

// Itération
for item in &vec { }
for item in &mut vec { }
for item in vec { }  // Consomme le vector
```

## Ownership et Borrowing

```rust
// Propriété (ownership)
let s1 = String::from("Hello");
let s2 = s1;  // MOVE: s1 n'existe plus
// println!("{}", s1);  // ❌ Erreur

// Copy (types simples)
let x = 5;
let y = x;  // COPY: x existe toujours
println!("{}, {}", x, y);  // ✅ OK

// Emprunt immutable
let s = String::from("Hello");
let r1 = &s;
let r2 = &s;  // OK: plusieurs emprunts
println!("{}, {}", r1, r2);

// Emprunt mutable
let mut s = String::from("Hello");
let r = &mut s;
r.push_str(" World");  // Modifier
// let r2 = &mut s;  // ❌ Erreur: 1 seul emprunt mutable

// Dereference
let x = 5;
let r = &x;
let value = *r;  // Dereference
```

## Structs

```rust
// Définition
struct Point {
    x: i32,
    y: i32,
}

// Instanciation
let point = Point { x: 0, y: 0 };
let mut point = Point { x: 0, y: 0 };
point.x = 10;

// Shorthand
let x = 5;
let y = 10;
let point = Point { x, y };  // Au lieu de x: x, y: y

// Tuple Struct
struct Color(i32, i32, i32);
let color = Color(255, 0, 0);

// Impl
impl Point {
    // Méthode (prend &self)
    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    // Méthode mutable (prend &mut self)
    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    // Fonction associée (pas de self)
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}

let p = Point { x: 3, y: 4 };
p.distance_from_origin();    // Méthode
Point::origin();              // Fonction associée
```

## Enums

```rust
// Énumération simple
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Enum avec données
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Pattern matching
match message {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to {}, {}", x, y),
    Message::Write(text) => println!("{}", text),
    Message::ChangeColor(r, g, b) => println!("Color"),
}

// Impl pour enum
impl Message {
    fn call(&self) {
        // Logique
    }
}

// Option (enum courant)
enum Option<T> {
    Some(T),
    None,
}

let x: Option<i32> = Some(5);
match x {
    Some(i) => println!("Got {}", i),
    None => println!("Got nothing"),
}

// Result (enum courant)
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Gestion des Erreurs

```rust
// Option
let maybe: Option<i32> = Some(42);
match maybe {
    Some(x) => println!("{}", x),
    None => println!("Nothing"),
}

// Result
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division par zéro"))
    } else {
        Ok(a / b)
    }
}

match divide(10, 2) {
    Ok(result) => println!("{}", result),
    Err(e) => println!("Error: {}", e),
}

// ? operator (propagation)
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Unwrap
let x = Some(5).unwrap();        // 5
let x = None.unwrap();           // PANIC!
let x = None.unwrap_or(0);       // 0
let x = None.unwrap_or_else(|| 0);  // 0 (lazy)

// Map
let maybe = Some(5);
let doubled = maybe.map(|x| x * 2);  // Some(10)
```

## Traits et Génériques

```rust
// Trait (interface)
trait Animal {
    fn speak(&self) -> String;
}

// Implémenter un trait
impl Animal for Dog {
    fn speak(&self) -> String {
        String::from("Woof!")
    }
}

// Génériques
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generique struct
struct Point<T> {
    x: T,
    y: T,
}

// Trait bounds
fn print_it<T: std::fmt::Display>(val: T) {
    println!("{}", val);
}
```

## Standard Library Courant

```rust
// println!, print!, eprint!
println!("Hello {}", name);
eprint!("Error");

// Vec
let mut vec = vec![1, 2, 3];
vec.push(4);
vec.pop();
vec.len();
vec.is_empty();

// String
let s = String::from("Hello");
let s = "Hello".to_string();
s.len();
s.chars();
s.split(' ');

// Option methods
maybe.is_some();
maybe.is_none();
maybe.unwrap_or(default);
maybe.map(|x| x * 2);
maybe.and_then(|x| Some(x * 2));

// Result methods
result.is_ok();
result.is_err();
result.ok();
result.err();
result.map(|x| x * 2);
result.map_err(|e| format!("Error: {}", e));

// Iterators
vec.iter();          // &T
vec.iter_mut();      // &mut T
vec.into_iter();     // T
vec.iter().filter(|x| x > &5).map(|x| x * 2);
```

## Commandes Cargo Utiles

```bash
cargo new project-name      # Créer nouveau projet
cargo build                 # Compiler
cargo run                   # Compiler et exécuter
cargo check                 # Vérifier sans compiler binaire
cargo test                  # Lancer les tests
cargo doc --open           # Générer et ouvrir la doc
cargo fmt                  # Formater le code
cargo clippy               # Linter (conseils)
cargo add package-name     # Ajouter dépendance
```

---

## Mnémoniques Rapides

| Concept | Symbole | Signification |
|---------|---------|---------------|
| `let x` | Immutable par défaut | Plus sûr |
| `let mut x` | Mutable explicite | Intention claire |
| `&` | Emprunt immutable | Lecture |
| `&mut` | Emprunt mutable | Modification |
| `*` | Dereference | Accéder à la valeur |
| `!` | Macro | `println!`, `panic!` |
| `?` | Propagate erreur | Retourner si Err |
| `->` | Type retour | Spécifier retour |
| `..` | Range exclusif | `0..10` = 0 à 9 |
| `..=` | Range inclusif | `0..=10` = 0 à 10 |

---

Bon codage! 🦀
