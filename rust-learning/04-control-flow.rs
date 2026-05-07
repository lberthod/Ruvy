// ============================================
// LEÇON 4: CONTRÔLE DE FLUX
// ============================================

fn main() {
    // === IF / ELSE ===
    let number = 5;

    if number > 0 {
        println!("Le nombre est positif");
    } else if number < 0 {
        println!("Le nombre est négatif");
    } else {
        println!("Le nombre est zéro");
    }

    // if comme expression
    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("value = {}", value);

    // === BOUCLE LOOP (infinie) ===
    let mut counter = 0;
    loop {
        println!("counter = {}", counter);
        counter += 1;
        if counter >= 3 {
            break;  // Sortir de la boucle
        }
    }

    // loop retourne une valeur
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 3 {
            break count * 2;  // Retourner 6
        }
    };
    println!("Loop returned: {}", result);

    // === BOUCLE WHILE ===
    let mut number = 3;
    while number > 0 {
        println!("{}", number);
        number -= 1;
    }

    // === BOUCLE FOR ===
    for i in 1..=5 {
        println!("i = {}", i);  // 1 à 5 inclus
    }

    // for avec tableau
    let array = [10, 20, 30, 40, 50];
    for element in &array {
        println!("element = {}", element);
    }

    // for avec indice
    for (index, value) in array.iter().enumerate() {
        println!("[{}] = {}", index, value);
    }

    // === MATCH (pattern matching) ===
    let number = 42;
    match number {
        1 => println!("Un"),
        2 => println!("Deux"),
        3..=10 => println!("Entre 3 et 10"),
        _ => println!("Autre"),  // Par défaut
    }

    // match retourne une valeur
    let result = match number {
        n if n % 2 == 0 => "Pair",
        _ => "Impair",
    };
    println!("{}", result);

    // === CHAÎNER DES CONDITIONS ===
    let age = 25;
    let has_license = true;

    if age >= 18 && has_license {
        println!("Vous pouvez conduire!");
    }

    if age < 13 || age > 65 {
        println!("Tarif spécial");
    }

    // === CONTINUE et BREAK ===
    for i in 1..=5 {
        if i == 2 {
            continue;  // Passer à la prochaine itération
        }
        if i == 4 {
            break;  // Sortir de la boucle
        }
        println!("{}", i);  // Affiche 1, 3
    }
}

// CONCEPTS CLÉS:
// 1. if, else if, else → branchements
// 2. if peut être une expression qui retourne une valeur
// 3. loop → boucle infinie, break pour sortir
// 4. while condition → boucle tant que
// 5. for in → itération (gammes, tableaux, itérateurs)
// 6. 1..5 → range exclusif (1 à 4)
// 7. 1..=5 → range inclusif (1 à 5)
// 8. match → pattern matching puissant
// 9. _ → wildcard (par défaut)
// 10. if guards → if n % 2 == 0 dans match
