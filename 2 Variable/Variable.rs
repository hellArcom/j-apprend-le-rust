fn main() {
      /////////////////////////////
     //// Variable modifiable ////
    ////////////////////////////
    let mut age = 20; // si on mais pas "mut" la valeur ne peut pas être modifier
    age = 21; // changer la vriable
    println!("Age : {}", age); // afficher la variable + texte

    /////////////////////////////
    // Variable non modifiable //
    ////////////////////////////

    let nom = "Arcom"; //non modifiable
    println!("Nom : {}", nom); // afficher la variable + texte

    ///////////////////////////////////////////////////////////////////////////////////////////////////
    ////////TYPES DSE DONNER : ////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////
    ///////Convertion de donner numérique //////
    ////////////////////////////////////////////
    
    // /!\ Attention à :
    // let x: u8 = 255;
    // let y = x as i8;
    // println!("{}", y); // Affiche : -1 
    // i8 va de -128 à 127 donc 255 dépasse ce qui affiche -1 :/

    let a: i32 = 10; // définir a en i32 (entier 32 bits)
    let b: f64 = a as f64; // défnir b en tant que f64 a a changer en f64 (f64 = nombre a virgule 64 bitss)
    println!("a en f64 : {}", b);

    ///////////////////////////////
    /////// De str a nombre ///////
    ///////////////////////////////

    let chain = "42";
    let nombre: i32 = chain.parse().unwrap();
    println!("Nombre : {}", nombre);

    let age = 20;
    let age_texte = age.to_string();
    println!("Âge en texte : {}", age_texte);

}

