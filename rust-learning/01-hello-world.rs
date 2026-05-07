// ============================================
// LEÇON 1: HELLO WORLD - Débuter avec Rust
// ============================================

fn main() {
    // println! est une macro qui affiche du texte suivi d'une nouvelle ligne
    println!("Hello, World!");

    // Tu peux afficher du texte simple
    println!("Bienvenue dans l'apprentissage du Rust!");

    // Les macros en Rust se terminent par !
    // println! est la macro d'affichage standard

    // Exemple avec plusieurs affichages
    let name = "Rust";
    println!("Je suis {}", name);
    println!("J'aime {} et c'est génial!", name);
}

// CONCEPTS CLÉS:
// 1. fn main() - c'est le point d'entrée du programme
// 2. Les blocs de code sont entre {} (accolades)
// 3. println! - affiche du texte
// 4. {} - placeholder pour insérer des variables
// 5. Chaque instruction se termine par ;

// EXÉCUTION:
// Pour compiler et exécuter ce programme:
// rustc 01-hello-world.rs
// ./01-hello-world
