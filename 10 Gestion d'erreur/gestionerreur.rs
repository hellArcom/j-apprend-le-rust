fn main() {
    println!("Gestion d'erreur ! ");

    match division(10.0, 0.0) {
        Ok(resultat) => println!("Résultat = {}", resultat),
        Err(msg) => println!("Erreur : {}", msg),
    }   
}

fn division(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division par zéro"))
    } else {
        Ok(a / b)
    }
}
