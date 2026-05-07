# 🦀 Apprendre le Rust - Guide Complet

Bienvenue! Ce dossier contient un cours complet pour apprendre le Rust, du débutant à l'intermédiaire.

## 📚 Structure du Cours

### Leçon 1: Hello World (`01-hello-world.rs`)
**Concepts:** Syntaxe de base, macros, println!

- ✅ Point d'entrée `fn main()`
- ✅ Macros avec `!`
- ✅ Affichage avec `println!`

**À faire:**
```bash
rustc 01-hello-world.rs
./01-hello-world
```

### Leçon 2: Variables et Types (`02-variables-and-types.rs`)
**Concepts:** Variables, types, immuabilité

**Points clés:**
- Les variables sont **immuables par défaut**
- `let mut` pour les variables mutables
- Types primitifs: `i32`, `u64`, `f64`, `bool`, `char`
- `String` vs `&str`
- Tuples, Arrays, Vectors

**Exemple:**
```rust
let x = 5;        // Immuable
let mut y = 10;   // Mutable
y = 20;

let tuple = (42, "hello", true);
let [a, b, c] = [1, 2, 3];
let vec = vec![1, 2, 3];
```

### Leçon 3: Fonctions (`03-functions.rs`)
**Concepts:** Déclaration de fonctions, paramètres, retours

**Syntaxe:**
```rust
fn nom(param: Type) -> ReturnType {
    // Code
    valeur  // Pas de ; = return implicite
}
```

**Points clés:**
- Types des paramètres obligatoires
- Retours implicites (dernière expression sans `;`)
- Peut retourner des tuples
- Les fonctions sont des expressions

### Leçon 4: Contrôle de Flux (`04-control-flow.rs`)
**Concepts:** if/else, boucles, match, ranges

**Structures:**
```rust
if condition {
    // ...
} else if {
    // ...
} else {
    // ...
}

while condition { }
for i in 1..=5 { }  // Inclusif
for i in 1..5 { }   // Exclusif
loop { break; }

match value {
    1 => println!("Un"),
    2..=5 => println!("2 à 5"),
    _ => println!("Autre"),
}
```

**Points clés:**
- `if` est une expression (retourne une valeur)
- `match` pour le pattern matching puissant
- Ranges: `1..5` (exclusif) vs `1..=5` (inclusif)
- Guard patterns dans match: `if n % 2 == 0`

### Leçon 5: Propriété et Emprunt (`05-ownership.rs`)
**LE CONCEPT CLÉ DE RUST** 🔑

#### Les 3 Règles:
1. Chaque valeur a UN propriétaire
2. Quand le propriétaire sort du scope, la valeur est supprimée
3. On peut emprunter sans devenir propriétaire

#### Déplacement (Move):
```rust
let s1 = String::from("Hello");
let s2 = s1;  // s1 est DÉPLACÉ
// s1 n'existe plus ici ❌
```

#### Copy:
Les types simples se copient implicitement:
- i32, f64, bool, char, tuples...

#### Emprunt Immutable (`&`):
```rust
let s = String::from("Hello");
let r1 = &s;  // Emprunt immutable
let r2 = &s;  // OK: plusieurs emprunts
println!("{}", s);  // Toujours propriétaire ✅
```

#### Emprunt Mutable (`&mut`):
```rust
let mut s = String::from("Hello");
let r = &mut s;  // Emprunt mutable
r.push_str(" World");  // Modifier via emprunt
// UN SEUL emprunt mutable à la fois ❌❌
```

**Points clés:**
- `String` (propriété) vs `&String` (emprunt)
- Move vs Copy
- Plusieurs emprunts immuables OK
- UN SEUL emprunt mutable
- Rust compile GARANTIT aucun data race!

### Leçon 6: Structs et Enums (`06-structs-and-enums.rs`)
**Concepts:** Types personnalisés, méthodes, énumérations

#### Structs:
```rust
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn display(&self) {  // &self = lecture
        println!("{}", self.name);
    }
    
    fn birthday(&mut self) {  // &mut self = modification
        self.age += 1;
    }
}

let person = Person {
    name: String::from("Alice"),
    age: 30,
};
person.display();
```

#### Enums:
```rust
enum Direction {
    Up,
    Down,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Text(String),
}

match message {
    Message::Quit => println!("Bye"),
    Message::Move { x, y } => println!("({}, {})", x, y),
    Message::Text(s) => println!("{}", s),
}
```

**Points clés:**
- Structs groupent les données
- Impl ajoute les méthodes
- Enums ont des variantes (peut avoir des données)
- Pattern matching traite toutes les variantes

### Leçon 7: Gestion des Erreurs (`07-error-handling.rs`)
**Concepts:** Option, Result, ? operator

#### Option<T>:
```rust
// Remplace null qui n'existe pas en Rust
match value {
    Some(x) => println!("Value: {}", x),
    None => println!("No value"),
}

// Raccourci
if let Some(x) = value {
    println!("Value: {}", x);
}

// Unwrap (attention!)
let n = Some(42).unwrap();  // OK
let n = None.unwrap();  // PANIC!
let n = None.unwrap_or(0);  // 0 (par défaut)
```

#### Result<T, E>:
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division par zéro"))
    } else {
        Ok(a / b)
    }
}

match divide(10, 2) {
    Ok(r) => println!("Résultat: {}", r),
    Err(e) => println!("Erreur: {}", e),
}
```

#### ? Operator (Propagation):
```rust
fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("test.txt")?;  // Retourner si erreur
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

**Points clés:**
- `Option<T>`: Some(T) ou None
- `Result<T, E>`: Ok(T) ou Err(E)
- `unwrap()` panique si None/Err
- `?` propage l'erreur (syntaxe courte)
- Pas d'exceptions, gestion explicite!

---

## 🎯 Exécuter les Leçons

### Installer Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Compiler et exécuter:
```bash
rustc 01-hello-world.rs
./01-hello-world

# Ou avec cargo
cargo new mon-projet
cd mon-projet
cargo run
```

### Avec Cargo (recommandé):
```bash
cargo new --name learn-rust .
cargo run --example 01-hello-world
```

---

## 📖 Concepts Importants Résumés

### Immuabilité
Les variables sont **immuables par défaut** → moins de bugs

### Propriété (Ownership)
Gestion automatique de la mémoire **sans garbage collector** → performance

### Emprunt (Borrowing)
Partager des données **sans transférer la propriété** → flexibilité

### Pattern Matching
**match** traite tous les cas possible → code sûr

### Type System
Rust infère les types mais vérifie **à la compilation** → zéro erreur de type à runtime

### ? Operator
Propagation d'erreur simple → code propre

---

## 🚀 Prochaines Étapes

1. **Traits** - Abstractions réutilisables
2. **Lifetimes** - Gestion avancée des références
3. **Closures** - Fonctions anonymes
4. **Modules** - Organisation du code
5. **Génériques** - Fonctions et types paramétrés
6. **Collections** - Vec, HashMap, BTreeMap
7. **I/O** - Fichiers, entrée utilisateur
8. **Async/Await** - Programmation asynchrone
9. **Trait Objects** - Polymorphisme dynamique
10. **Macros** - Métaprogrammation

---

## 💡 Conseils Importants

### ✅ Fais:
- Lire les messages d'erreur du compilateur (très utiles!)
- Expérimenter dans le REPL ou dans main()
- Utiliser `cargo check` pour vérifier rapidement
- Lire le code d'autres projets Rust
- Faire les exercices et petits projets

### ❌ Ne fais pas:
- Ne pas utiliser `unwrap()` en production
- Ne pas ignorer les warnings du compilateur
- Ne pas forcément utiliser `mut` partout
- Ne pas chercher à cloner partout (inefficace)

---

## 🔗 Ressources

- **Official Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rust Playground**: https://play.rust-lang.org/
- **Rustlings** (exercices): https://github.com/rust-lang/rustlings

---

## ❓ Questions Fréquentes

**Q: Pourquoi Rust est difficile?**
A: Le compilateur est strict mais pour ta sécurité! Les erreurs détectées à la compilation évitent les bugs en production.

**Q: Quand utiliser &, &mut, et owned?**
A: 
- `&` (immutable borrow) → par défaut, lecture
- `&mut` (mutable borrow) → modification temporaire
- Owned → nécessaire rarement

**Q: Rust ou Go?**
A: Rust = sécurité à la compilation, Go = simplicité. Pour système bas-niveau: Rust. Pour réseau: Go.

**Q: Rust est plus rapide?**
A: Aussi rapide que C/C++. Zéro-cost abstractions!

---

## 🎓 Plan d'Apprentissage Recommandé

**Semaine 1:**
- Leçons 1-3: Syntaxe de base

**Semaine 2:**
- Leçons 4-5: Contrôle de flux + propriété (LE POINT CLIL!)

**Semaine 3:**
- Leçons 6-7: Structs + gestion d'erreurs

**Semaine 4:**
- Petits projets: todo-list, mini-jeu, web scraper

---

Bon apprentissage! 🦀✨
