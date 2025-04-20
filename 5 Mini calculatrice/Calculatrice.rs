use std::io::{self, Write};

fn main() {
    println!("Bienvenue sur la calculatrice ! ");

    let mut nombre1 = String::new(); //Pour l'entrer utilisateur nb1
    let mut nombre2 = String::new(); //Pour l'entrer utilisateur nb2

    //entrer utilisateur
    print!("Entrez le premier nombre : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nombre1).unwrap();

    print!("Entrez le deuxième nombre : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nombre2).unwrap();

    //convertir en float
    let nombre1: f64 = nombre1.trim().parse().unwrap();
    let nombre2: f64 = nombre2.trim().parse().unwrap();

    //calculer les résultat
    let resultat_add = nombre1 + nombre2;
    let resultat_sub = nombre1 - nombre2;
    let resultat_mul = nombre1 * nombre2;
    let resultat_div = nombre1 / nombre2;

    //afficher les résultat
    println!("_____________________________________________________");
    println!("{} + {} = {}",nombre1, nombre2, resultat_add);
    println!("{} - {} = {}",nombre1, nombre2, resultat_sub);
    println!("{} x {} = {}",nombre1, nombre2, resultat_mul);
    println!("{} ÷ {} = {}",nombre1, nombre2, resultat_div);
    println!("_____________________________________________________");


    // Pause à la fin pour empêcher la fermeture immédiate
    println!("\nAppuyez sur Entrée pour quitter...");
    io::stdout().flush().unwrap();
    let mut pause = String::new();
    io::stdin().read_line(&mut pause).unwrap();
}
