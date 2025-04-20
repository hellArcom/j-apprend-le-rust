fn main() {
    let mut pseudo = "Arcom";

    println!("Les fonction !");
    bonjour(pseudo);
    println!("Au revoir !");
}

fn bonjour(pseudo: &str) {
    println!("Bonjour {}", pseudo);
}