fn main() {

    // Déclaration d'une variable entière de type i32 (entier 32 bits)
    let i: i32 = 56; 

    // Déclaration d'une variable booléenne (vrai ou faux)
    let b: bool = true; 

    // Déclaration d'une variable de type char, représentant un caractère Unicode
    let c: char = 'e'; 

    // Déclaration d'une variable flottante (f64), pour les nombres à virgule
    let f: f64 = 5.6; // nombre float (à virgule)

    // Déclaration d'une référence à une variable i32 (pas une copie, juste un pointeur vers i)
    let p: &i32 = &i; // référence à i (& signifie "référence") utilise la valeur de l'autre variable

    // Déclaration d'une variable de type chaîne de caractères
    let s: &str = "chaine de caractère"; // chaîne de caractères, type &str

    // Déclaration d'un tableau avec 5 éléments de type i32
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // tableau de 5 éléments i32

    // Déclaration d'un tuple avec 3 éléments de types différents : char, bool, i32
    let t: (char, bool, i32) = ('c', true, 5); // tuple (char, bool, i32)

    // Affichage d'un texte simple dans la console
    println!("Hello World"); // afficher un texte normal

    // Affichage de la valeur de la variable i (i32)
    println!("{}", i); // affiche la valeur de i

    // Affichage de la valeur de la variable b (bool)
    println!("{}", b); // affiche la valeur de b (true/false)

    // Affichage de la valeur de la variable c (char)
    println!("{}", c); // affiche la valeur de c (un seul caractère)

    // Affichage de la valeur de la variable f (f64)
    println!("{}", f); // affiche la valeur de f (nombre à virgule flottante)

    // Affichage de la valeur pointée par la référence p
    println!("{}", p); // affiche la valeur de i à laquelle p fait référence

    // Affichage de la chaîne de caractères s
    println!("{}", s); // affiche la chaîne s

    // Affichage du tableau a en format debug (utilise {:?} pour afficher les tableaux)
    println!("{:?}", a); // affiche le tableau a sous forme de liste avec des crochets

    // Affichage du tuple t en format debug
    println!("{:?}", t); // affiche le tuple t sous forme de (char, bool, i32)

}
