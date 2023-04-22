use core::num;

fn main() {
    /*
        i8,i16,i32,i64,i128 entiers signés
        u8,u16,u32,u64,u128 entiers non-signés

        i -> -(2^(n-1)) à 2^(n-1)-1
        i8 -> -(2^7) à 2^(7) - 1
              -128 à 127
            
        u8 -> 0 à 2^(n) -1
            -> 0 à 2^8 -1
               0 à 255

        isize, usize -> indexation 
        sur combien de bit il sont stocké. Depend de l'architecture du PC

        f32,f64 -> Nombre à virgule, nombre flottant

        bool -> un octet -> 8 bits

        char -> 4 octets -> 32 bits
        
        &str -> string litterals

        Tuple -> Chaine de plusieurs type de valeur

        array -> tableau de valeur

        shadowing - Result

        Result -> Ok(nombre)
               -> Err(erreur)
     */

    // let a = 0b1111_1111_1111u32; // Binaire
    // let b = 0xffff_ffffu32; // Hexadecimal
    // let c = 0o777_777_777u32; // Base octal

    // let a = b'A';
    // let a = 5_000.252_525; // Nombre flotant
    // let a = false; // Boolean sur "false"
    // let b = true; // Boolean sur "true"

    // let a = 'ぱ'; // Caractère Char encodé avec UTF-8 (4 Octet)
    // let a = "Kisakay apprend le rust 🧑‍💻"; // String &str dont le contenue n'est pas modifiable
    
    // let a = (5, 'a', 57.5, "sqdqsqsq"); // Tuple contenant plusieurs type de valeur
    // let (b,mut c,d,e) = a; // Assignation de chaque valeur du tuple à une variable externe

    // let a = ['a', 'a']; // Tableau de valeur // Simple tableau
    // let a : [i32;4]; // Simple tableau de valeur qui réplique un "i32" 4 fois

    // let number = "s36";
    // let mut number = number.parse::<u32>().expect("\nErreur lors du parse\n");
    // number = number +1;
    // println!("{}", number);

    // let a : i32 = 3;
    // let b = [3;5];
    // print!("{}", b[a as usize]);

    // println!(" \"\t\n \\")
}
