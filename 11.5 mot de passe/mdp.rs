use std::io::{self, Write};

fn main() {
    let mdp = "LeSuperMotDePasse";

    println!("Le but est de mettre le bon mot de passe");

    println!("Entrer le mot de passe : ");
    io::stdout().flush().unwrap(); // Forcer l'affichage du prompt
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == mdp {
        println!("Mot de passe correct !");
    } else {
        println!("Mot de passe incorrect !");
    }
}

