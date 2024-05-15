use std::io;
use std::io::BufRead;


fn main() {
    exercise3();
}

// Scrivere un'implementazione che permetta la lettura di un numero intero e stampi a video se pari o meno
fn exercise1() {
    println!("Enter a number:");
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let parsed_line = line.parse::<i32>();
    if parsed_line.is_ok() {
        if parsed_line.unwrap() % 2 == 0 {
            println!("The number dialed is even");
        } else {
            println!("The number dialed is odd");
        }
    } else {
        println!("The number you dialed is not valid!")
    }
}

// Scrivere un'implementazione che permetta la lettura di un numero intero
// e che stampi a video se multiplo di 3, 5, entrambi o nessuno dei due.
fn exercise2() {
    println!("Enter a number:");
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let parsed_line = line.parse::<i32>();
    if parsed_line.is_err() {
        println!("The number you dialed is not valid!")
    } else {
        match parsed_line.unwrap() {
            x if (x % 3 == 0) & (x % 5 == 0) => println!("The a multiple of 3 and 5"),
            x if x % 3 == 0 => println!("The a multiple of 3"),
            x if x % 5 == 0 => println!("The a multiple of 5"),
            _ => println!("The number you dialed is not multiple of 3 or 5")
        }
    }
}

// Leggere un numero intero positivo,
// stampare dei `#` crescenti (1,2,3...) ad ogni riga fino ad arrivare
// ad un numero di `#` pari al numero inserito usando le classi

// Es. con 5:
// #
// ##
// ###
// ####
// #####

trait PrintHashes {
    fn print_hashes(&self);
}

struct HashPrinter {
    number_line: i32,
}

impl PrintHashes for HashPrinter {
    fn print_hashes(&self) {
        if self.number_line <= 0 {
            eprintln!("The number can't be less or equal to zero")
        }
        for n in 1..=self.number_line {
            print!("{}. ", n);
            for _m in 0..n {
                print!("#")
            }
            println!()
        }
    }
}

fn exercise3() {
    println!("Enter a number:");
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let parsed_line = line.parse::<i32>();
    if parsed_line.is_err() {
        eprintln!("The number you dialed is not valid!")
    } else {
        let hash_printer = HashPrinter { number_line: parsed_line.unwrap() };
        hash_printer.print_hashes();
    }
}