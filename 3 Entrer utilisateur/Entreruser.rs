use std::io::{self, Write}; // Ajout de Write pour flush()

fn main() {
    let mut entrer_user = String::new();

    print!("Entrez quelque chose : ");
    io::stdout().flush().unwrap(); // Forcer l'affichage du prompt
    io::stdin().read_line(&mut entrer_user).expect("Échec de la lecture");
    println!("Vous avez entré : {}", entrer_user.trim());

    entrer_user.clear();

    print!("Entrez quelque chose : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrer_user).expect("Échec de la lecture");
    println!("Vous avez entré : {}", entrer_user.trim());
}
