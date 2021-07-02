use rand::Rng;
use std::collections::HashSet;
use std::io;
// use std::time::Instant;

fn main() {
    // Generation d'une liste contenant 255 entiers aleatoire uniques

    let mut array: [i32; 255] = [0; 255];
    let mut hashset: HashSet<i32> = HashSet::new();

    // let now = Instant::now();

    // Array way
    //
    // for i in 1..255 {
    //     let rd = loop {
    //         let x = rand::thread_rng().gen_range(1..1001);
    //         if !(array.contains(&x)) {
    //             break x;
    //         }
    //     };
    //     array[i] = rd;
    // }
    //
    // Temps d'execution moyen : 1,5 ms

    // HashMap way
    //
    let mut k: usize = 0;
    let mut hash_size: i16 = 0;
    while hash_size != 255 {
        hashset.insert(rand::thread_rng().gen_range(1..1001));
        hash_size = hashset.len() as i16;
    }
    for nb in hashset.iter() {
        array[k] = *nb;
        k += 1;
    }
    //
    // Temps d'execution moyen : 500 µs

    // let new_now = Instant::now();
    // println!("Temps d'execution: {:?}", new_now.duration_since(now));

    // Trier la liste d'entiers

    array.sort();

    // Demande d'un entier compris entre minimum et maximum de la liste a l'utilisateur

    let mut number_str = String::new();

    let nb_int = loop {
        println!("Please enter an integer > 0 and <= 1000 :");
        number_str.clear();
        io::stdin()
            .read_line(&mut number_str)
            .expect("Cannot read your entry.");
        let number: i32 = match number_str.trim().parse() {
            Ok(n) => {
                if n > 1000 || n <= 0 {
                    continue;
                }
                n
            }
            Err(_) => continue,
        };
        break number;
    };

    // Recherche et affichage de l'indice dans l'array du nombre correspondant a l'entree utilisateur

    // let now = Instant::now();

    // Methode par defaut
    //
    // let result: i32 = match array.iter().position(|&x| x == nb_int) {
    //     Some(nb) => nb as i32,
    //     None => -1,
    // };
    //
    // Temps moyen de recherche : 5 µs

    // Methode dichotomique
    //
    let result: i32 = search(nb_int, array);
    //
    // Temps moyen de recherche : 600 ns

    // let new_now = Instant::now();
    // println!("Temps de recherche: {:?}", new_now.duration_since(now));

    if result != -1 {
        println!("L'element est a la position : {}.", result);
    } else {
        println!("L'element est introuvable.");
    }
}

// FN Recherche dichotomique prenant en parametre l'entier a trouver, sa liste et retourne sa position.
fn search(nb: i32, array: [i32; 255]) -> i32 {
    let mut start: i16 = 0;
    let mut end: i16 = 254;
    let mut mid: i16 = 0;
    let mut found: bool = false;

    while !found && start <= end {
        mid = (start + end) / 2;
        if array[mid as usize] == nb {
            found = true;
        } else {
            if nb > array[mid as usize] {
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }
    }
    if found {
        return mid as i32;
    } else {
        return -1;
    }
}
