fn main() {
    println!("Les conditions !");

    let age = 99;

    if age >= 18 {
        println!("Vous êtes majeur");
    } else if age >= 100 {
        println!("Vous êtes trops vieux pour ouvrir cette app");
    } else {
        println!("Vous êtes mineur");
    }

}