// ============================================
// LEÇON 5: PROPRIÉTÉ ET EMPRUNT (OWNERSHIP)
// ============================================
// C'est LE concept clé de Rust! Pas de garbage collector,
// la mémoire est gérée automatiquement via les règles de propriété.

fn main() {
    // === PROPRIÉTÉ (OWNERSHIP) ===
    // Chaque valeur a un propriétaire unique
    let s1 = String::from("Hello");
    let s2 = s1;  // s1 est DÉPLACÉE vers s2
    // println!("{}", s1);  // ❌ ERREUR! s1 n'existe plus

    println!("{}", s2);  // ✅ OK

    // Les types simples (Copy) se copient, pas se déplacent
    let x = 5;
    let y = x;  // x est COPIÉ dans y (pas déplacé)
    println!("x={}, y={}", x, y);  // ✅ Tous les deux OK

    // === EMPRUNT IMMUTABLE (Borrow) ===
    let s = String::from("Rust");
    let len = calculate_length(&s);  // Emprunter immutablement
    println!("'{}' a {} caractères", s, len);
    println!("s = {}", s);  // ✅ s existe toujours

    // Plusieurs emprunts immuables en même temps
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("r1={}, r2={}, r3={}", r1, r2, r3);

    // === EMPRUNT MUTABLE (Mutable Borrow) ===
    let mut s = String::from("Hello");
    change_string(&mut s);
    println!("s = {}", s);  // "Hello World"

    // ❌ On ne peut avoir qu'un SEUL emprunt mutable à la fois
    let mut number = 5;
    let r1 = &mut number;
    // let r2 = &mut number;  // ❌ ERREUR!
    // println!("{}, {}", r1, r2);

    println!("{}", r1);  // ✅ OK

    // === RÈGLES DE PROPRIÉTÉ ===
    // 1. Chaque valeur a UN propriétaire
    // 2. Quand le propriétaire est supprimé, la valeur l'est aussi
    // 3. On peut emprunter la valeur (& ou &mut)

    let owner = String::from("Owner");
    borrow_immutable(&owner);
    borrow_immutable(&owner);  // Multiple immuables emprunts OK
    println!("{}", owner);  // Toujours le propriétaire

    // === DÉPLACEMENT (MOVE) ===
    let s1 = String::from("Original");
    let s2 = s1;  // Déplacement!
    println!("{}", s2);
    // println!("{}", s1);  // ❌ ERREUR! s1 a été déplacé

    // === RETOUR ET DÉPLACEMENT ===
    let s1 = String::from("Hello");
    let s2 = takes_ownership(s1);  // s1 est déplacé dans la fonction
    println!("{}", s2);
    // println!("{}", s1);  // ❌ ERREUR!

    // === CLONAGE EXPLICITE ===
    let s1 = String::from("Clone");
    let s2 = s1.clone();  // Copie profonde explicite
    println!("s1={}, s2={}", s1, s2);  // ✅ Les deux existent
}

// Emprunt immutable: emprunter sans modifier
fn calculate_length(s: &String) -> usize {
    s.len()
} // s sort du scope, mais elle n'est pas propriétaire

// Emprunt mutable: emprunter et modifier
fn change_string(s: &mut String) {
    s.push_str(" World");
}

fn borrow_immutable(s: &String) {
    println!("Borrowed immutably: {}", s);
}

fn takes_ownership(s: String) -> String {
    println!("s = {}", s);
    s  // Retourner la propriété
}

// CONCEPTS CLÉS:
// 1. Ownership (Propriété)
//    - Chaque valeur a un seul propriétaire
//    - Quand le propriétaire sort du scope, la valeur est supprimée
//
// 2. Move (Déplacement)
//    - Assigner une variable à une autre la déplace
//    - L'original devient inutilisable
//    - Concerne les types non-Copy: String, Vec, etc.
//
// 3. Copy
//    - Les types simples: i32, f64, bool, char
//    - Ils se copient implicitement
//
// 4. Borrowing (Emprunt)
//    - & → emprunt immutable (lecture seule)
//    - &mut → emprunt mutable (modification)
//    - Plusieurs emprunts immuables OK
//    - UN SEUL emprunt mutable
//
// 5. Lifetime
//    - Durée de vie d'une référence
//    - Rust garantit qu'une référence ne pointe pas vers une valeur supprimée
