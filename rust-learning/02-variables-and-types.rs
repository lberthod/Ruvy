// ============================================
// LEÇON 2: VARIABLES ET TYPES
// ============================================

fn main() {
    // === VARIABLES IMMUABLES (par défaut) ===
    let x = 5;
    // x = 6;  // ❌ ERREUR! x est immuable par défaut
    println!("x = {}", x);

    // === VARIABLES MUTABLES ===
    let mut y = 10;
    y = 20;  // ✅ Correct, y est mutable
    println!("y = {}", y);

    // === TYPES ENTIERS ===
    let a: i32 = 42;       // Entier signé 32-bit
    let b: u64 = 100;      // Entier non-signé 64-bit
    let c: i8 = -5;        // Entier signé 8-bit
    println!("a={}, b={}, c={}", a, b, c);

    // === TYPES FLOTTANTS ===
    let f1: f32 = 3.14;    // Flottant 32-bit
    let f2: f64 = 2.71828; // Flottant 64-bit (par défaut)
    println!("f1={}, f2={}", f1, f2);

    // === BOOLÉENS ===
    let is_rust_awesome: bool = true;
    let is_slow: bool = false;
    println!("Rust c'est awesome? {}", is_rust_awesome);

    // === CARACTÈRES ET CHAÎNES ===
    let character: char = 'R';  // Un seul caractère (4 bytes)
    let string: &str = "Hello"; // Référence à une chaîne (string slice)
    let owned_string: String = String::from("World");
    println!("'{}' {} {}", character, string, owned_string);

    // === DÉDUCTION DE TYPE ===
    let inferred = 99;  // Rust déduit que c'est i32
    let deduced_float = 5.0;  // Rust déduit que c'est f64
    println!("inferred={}, deduced_float={}", inferred, deduced_float);

    // === TUPLES ===
    let tuple: (i32, f64, bool) = (42, 3.14, true);
    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    let (x, y, z) = tuple;  // Destructuration
    println!("Destructuré: x={}, y={}, z={}", x, y, z);

    // === TABLEAUX ===
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);
    println!("Premier élément: {}", array[0]);

    // === VECTEURS (tableau dynamique) ===
    let mut vector = vec![1, 2, 3];
    vector.push(4);
    println!("Vector: {:?}", vector);
}

// CONCEPTS CLÉS:
// 1. let x = valeur  → variable immuable (immutable par défaut)
// 2. let mut x = valeur → variable mutable
// 3. Types primitifs: i32, u64, f64, bool, char, str
// 4. String vs &str: String est du texte propriétaire, &str est une référence
// 5. Tuples: collection de tailles fixes avec différents types
// 6. Arrays: collection de taille fixe du même type
// 7. Vectors: tableau dynamique (Vec<T>)
