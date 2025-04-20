fn main() {
    let mut compteur = 0;

    //boucle infinit sauf si casser avec break
    loop {
        println!("Compteur : {}", compteur);
        compteur += 1;

        if compteur == 5 {
            println!("On arrête !");
            break;
        }
    }

    compteur = 0; //remettre le conteur a 0
    
    // boucle while elle tourne tant que un condition est vraie
    while compteur < 5 {
        println!("Compteur : {}", compteur);
        compteur += 1;
    }

    //boucle for
    for i in 1..5 {
        println!("i = {}", i); // Affiche de 1 à 4
    }

    for i in 1..=5 {
        println!("i = {}", i); // Affiche de 1 à 5
    }

    
}