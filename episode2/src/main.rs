fn main() {
    /* 

    Regle a respecter :

    1) Les noms doivent être obliguatoirement être de l'alphabet sans aucun accent. Avec le "-" ou "_" avec des chiffre de 0 à 9
    2) Toujours commencer le noms d'une variable avec soit un caractères spéciaux ou des lettre, jamais un chiffre
    3) Les noms sont sensibles à la casse

    let -> déclare une variable immutable (qui ne peut pas être modifiée après affectation)
    let mut -> déclare une variable mutable
    */
    let mut age = 20;

    println!("age = {}", age);

    age = age + 1;
    
    // let age = age + 1;

    println!("age = {}", age);
}