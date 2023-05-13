fn main() {
    let mut a = 15;

    // +,-,*,/, % (Modulo)

    // Les opérateur arithmétique
    // a = a +15;
    // a = a - 25;
    // a = a * 5;
    // a = a / 5;
    // a = a % 5; // Le reste de la division d'une division entoère - "Modulo"
            // Version simplifié
            a += 15;
            a -= 25;
            a *= 5;
            a /= 5;
            a %= 5;
            
    // = -> Opérateur d'affectation
    
    // Les opérateur logique
    // ==, !=, <, >; <=, >=;
    // ||, &&, !; -> Servent a tester l'état de deux expression

    let b = !(a != 0); // true, false (BOOLEAN)
    /*
        true && true -> true
        * -> false
         \_> Opérateur &&

        true || true -> true
        true || false -> true
        false || true -> true
        * -> false
         \_> Opérateur ||

        ! true -> false
        ! false -> true
         \_> Opérateur !
    */

    /* 
        Opérateur binaire
        &, | , ^, <<, >>, !
    */

    let c = 0b0000_0001 << 3; // -> Décalage du bit 1 de trois bit
    
    println!("a = {}, b = {}, c = {:08b}", a, b, c);
}