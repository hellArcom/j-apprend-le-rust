fn main() {
    println!("Hello World");

    let nb1: i32 = 15;
    let nb2: i32 = 10;

    let add: i32 = nb1 + nb2;
    let sou: i32 = nb1 - nb2;
    let mul: i32 = nb1 * nb2;
    let div: i32 = nb1 / nb2;
    let modl: i32 = nb1 % nb2;

    println!("Addition ({} + {}) = {}", nb1, nb2, add);
    println!("Soustraction ({} - {}) = {}", nb1, nb2, sou);
    println!("Multiplication ({} * {}) = {}", nb1, nb2, mul);
    println!("Division ({} / {}) = {}", nb1, nb2, div);
    println!("Modulo ({} % {}) = {}", nb1, nb2, modl);
}