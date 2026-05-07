// ============================================
// LEÇON 3: FONCTIONS
// ============================================

fn main() {
    // Appeler une fonction simple
    greet();

    // Fonction avec paramètres
    add_and_print(5, 3);

    // Fonction qui retourne une valeur
    let result = add(10, 20);
    println!("10 + 20 = {}", result);

    // Fonction avec plusieurs retours (tuple)
    let (sum, product) = arithmetic(5, 6);
    println!("Somme: {}, Produit: {}", sum, product);

    // Fonction avec plusieurs paramètres typés
    let area = rectangle_area(4.0, 5.0);
    println!("Area of rectangle: {}", area);

    // Les expressions vs instructions
    let number = {
        let x = 3;
        x + 1  // Pas de ; = return implicite
    };
    println!("number = {}", number);
}

// Fonction simple sans paramètres ni retour
fn greet() {
    println!("Hello!");
}

// Fonction avec paramètres
fn add_and_print(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

// Fonction qui retourne une valeur
// -> type_retour
fn add(a: i32, b: i32) -> i32 {
    a + b  // Pas de ; = return implicite
}

// Fonction retournant un tuple
fn arithmetic(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

// Fonction avec paramètres de type flottant
fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

// Fonction plus complexe avec logique
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// Fonction avec plusieurs instructions
fn fizzbuzz(n: i32) -> String {
    if n % 15 == 0 {
        String::from("FizzBuzz")
    } else if n % 3 == 0 {
        String::from("Fizz")
    } else if n % 5 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }
}

// CONCEPTS CLÉS:
// 1. fn nom(param: type) { } → définir une fonction
// 2. Les paramètres doivent avoir un type explicite
// 3. -> type_retour → déclarer le type retourné
// 4. La dernière expression sans ; est retournée implicitement
// 5. Avec ;, c'est une instruction (retourne ())
// 6. Les fonctions peuvent retourner des tuples
// 7. Rust utilise des expressions (return implicite) vs des instructions
