fn main() {
    println!("Ce programme vous permet de savoir entre quelle âge et quelle âge vous avez.")

    let mut age = String::new();

    print!("Entrer votre âge : ")
    io::stdout().flush().unwrap(); // forcer l'affichage
    io::stdin().read_line(&mut age).unwrap().expect("Échec de la leceture du texte de l'utilisateur")
    print!("Votre âge est : {}" age)

    if age >=0 && age >9 {
        println!("Vous avez entre 0 et 10 ans")
    }
    else if age >=10 && age >=19 {
        println!("Vous avez entre 10 et 20 ans")
    }
    if age >=20 && age >29 {
        println!("Vous avez entre 20 et 30 ans")
    }
    else if age >=30 && age >=39 {
        println!("Vous avez entre 30 et 40 ans")
    }
    else if age >=40 && age >49 {
        println!("Vous avez entre 40 et 50 ans")
    }
    else if age >=50 && age >=59 {
        println!("Vous avez entre 50 et 60 ans")
    }
    else if age >=60 && age >69 {
        println!("Vous avez entre 60 et 70 ans")
    }
    else if age >=70 && age >79 {
        println!("Vous avez entre 70 et 80 ans")
    }
    else if age >=80 && age >89 {
        println!("Vous avez entre 80 et 90 ans")
    }
    else if age >=90 && age >99 {
        println!("Vous avez entre 90 et 100 ans")
    }
    else if age >=100 {
        println!("Vous avez plus de 100ans !")
    }
}