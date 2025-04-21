use std::io::{self, Write};

fn main() {
    println!("Bienvenue sur la calculatrice !");

    let mut choix = String::new();
    let mut nombre1 = String::new();
    let mut nombre2 = String::new();

    loop {
        println!("\n#########################");
        println!("# 1 - Addition          #");
        println!("# 2 - Soustraction      #");
        println!("# 3 - Multiplication    #");
        println!("# 4 - Division          #");
        println!("# 5 - Quitter           #");
        println!("#########################");

        print!("Faites un choix : ");
        io::stdout().flush().unwrap();
        choix.clear();
        io::stdin().read_line(&mut choix).unwrap();
        let choix = choix.trim();

        if choix == "5" {
            println!("Au revoir !");
            break;
        }

        // Entrée du premier nombre
        nombre1.clear();
        print!("Entrez le premier nombre : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nombre1).unwrap();
        let nombre1: f64 = match nombre1.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("❌ Entrée invalide pour le premier nombre !");
                continue;
            }
        };

        // Entrée du deuxième nombre
        nombre2.clear();
        print!("Entrez le deuxième nombre : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nombre2).unwrap();
        let nombre2: f64 = match nombre2.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("❌ Entrée invalide pour le deuxième nombre !");
                continue;
            }
        };

        // Match des opérations avec gestion d’erreur
        let resultat = match choix {
            "1" => Some(addition(nombre1, nombre2)),
            "2" => Some(soustraction(nombre1, nombre2)),
            "3" => Some(multiplication(nombre1, nombre2)),
            "4" => {
                if nombre2 == 0.0 {
                    println!("🚫 Division par zéro interdite !");
                    None
                } else {
                    Some(division(nombre1, nombre2))
                }
            }
            _ => {
                println!("❌ Choix invalide. Veuillez réessayer.");
                None
            }
        };

        if let Some(res) = resultat {
            println!("✅ Résultat = {}", res);
        }
    }
}

// Fonctions des opérations
fn addition(a: f64, b: f64) -> f64 {
    a + b
}

fn soustraction(a: f64, b: f64) -> f64 {
    a - b
}

fn multiplication(a: f64, b: f64) -> f64 {
    a * b
}

fn division(a: f64, b: f64) -> f64 {
    a / b
}
