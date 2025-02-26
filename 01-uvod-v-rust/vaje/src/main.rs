use core::panic;

/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(a0: u32, a1: u32, n: u32) -> u32 {
    let mut prvi_clen = a0;
    let mut drugi_clen = a1;
    let mut index = 0;
    loop {
        if index >= n {
            return prvi_clen;
        }
        let vsota = prvi_clen + drugi_clen;
        prvi_clen = drugi_clen;
        drugi_clen = vsota;
        index += 1;
    }
    // let a :u32 = loop {
    // if index >= n {
    //    return prvi_clen;
    // }
    //let vsota = prvi_clen + drugi_clen;
    //prvi_clen = drugi_clen;
    //drugi_clen = vsota;
    //index += 1;
    // potem se v a shrani vrednost (torej zanke vračajo vrednost, kar je drugače kot pri drugih
    //programskih jezikih)
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);
fn je_veljaven_datum(datum: Date) -> bool {
    if (datum.1 == 1)
        || datum.1 == 3
        || datum.1 == 5
        || datum.1 == 7
        || datum.1 == 8
        || datum.1 == 10
        || datum.1 == 12
    {
        if datum.0 > 0 && datum.0 <= 31 {
            true
        } else {
            return false;
        }
    } else if datum.1 == 4 || datum.1 == 6 || datum.1 == 9 || datum.1 == 11 {
        if datum.0 > 0 && datum.0 <= 30 {
            true
        } else {
            return false;
        }
    } else if datum.1 == 2 {
        if datum.0 > 0 && datum.0 < 29 {
            return true;
        } else if datum.0 == 29 {
            if datum.2 % 4 == 0 && datum.2 % 100 != 0 {
                return true;
            } else if datum.2 % 100 == 0 && datum.2 % 400 != 0 {
                return false;
            } else if datum.2 % 400 == 0 {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

fn iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    while cond(start) != true {
        start = fun(start);
    }
    start
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b] glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

fn bisekcija(mut a: f64, mut b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    if fun(a) * fun(b) > 0. {
        panic!("Funkcija nima ničel!");
    } else if fun(a) == 0. {
        a
    } else if fun(b) == 0. {
        b
    } else if a == b {
        panic! {"a = b je točka!!!"};
    } else {
        loop {
            let c: f64 = (a + b) / 2.;
            if fun(c).abs() < prec {
                break c;
            } else if (a - b).abs() < prec {
                break c;
            } else if fun(c) == 0. {
                break c;
            } else {
                if fun(a) * fun(c) < 0. {
                    b = c;
                } else {
                    a = c;
                }
            }
        }
    }
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.
use std::cmp::Ordering;
use std::io;
use std::ptr::with_exposed_provenance;
fn guessing_game() {
    println!("Guess the number!");
    let mut first: Option<i32> = None;
    let mut second: Option<i32> = None;
    let mut diff: Option<i32> = None;

    loop {
        println!("Please input your number.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match first {
            Some(n1) => {
                match second {
                    Some(n2) => {
                        if guess - n2 != diff.unwrap() {
                            println!("Vnešeni člen ni naslednji člen aritmetičnega zaporedja!");
                            return;
                            second = Some(guess);
                        }
                    }
                    None => {
                        second = Some(guess);
                        diff = Some((guess - first.unwrap())) //unwrap pridobi vrednost iz Some-a
                    }
                }
            }
            None => {
                first = Some(guess);
            }
        };
    }
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    panic!("Not implemented");
}

fn vsebuje<T: PartialEq>(v: &Vec<T>, x: &T) -> bool {
    for y in v {
        if x == y {
            return true;
        }
    }
    return false;
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(arr: &mut [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(arr: &mut [u32]) {}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido višine `n` iz zvezdic

fn pyramid(n: u32) {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite program, ki izpiše piramido črk angleške abecede višine `n`, lahkom predpostavite, da bo n največ 26.
///      A
///    A B A
///   A B C B A
/// A B C D C B A
/// Napišite funkcijo `fn selection_sort(mut arr: [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn povecaj_za_ena(x: u32) -> u32 {
    x + 1
}

fn vecje_od_sto(x: u32) -> bool {
    println! {"Trenutno število je {x}"};
    x > 100
}

fn identiteta(x: f64) -> f64 {
    x
}

fn fun1(x: f64) -> f64 {
    {
        3. * x * x + 2. * x - 15.
    }
}

fn main() {
    //println!("{}", fib(0, 1, 5));
    //println!("{}", je_veljaven_datum((31, 1, 2025)));
    //println!("{}", je_veljaven_datum((29, 4, 2025)));
    //println!("{}", je_veljaven_datum((30, 2, 2025)));
    //println!("{}", je_veljaven_datum((29, 2, 1600)));
    //println!("{}", je_veljaven_datum((29, 2, 1700)));
    //println!("{}", je_veljaven_datum((29, 2, 2024)));
    //println!("{}", je_veljaven_datum((31, 4, 2025)));
    //println!("{}", iteracija(3, povecaj_za_ena, vecje_od_sto));
    println!("{}", bisekcija(-10., 13.4, identiteta, 0.02));
    println!("{}", bisekcija(0., 13.4, fun1, 0.02));
    println!("{}", bisekcija(0., 0., identiteta, 0.02));
    println!("{}", bisekcija(0., 0., fun1, 0.02));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        assert_eq!(result, ());
    }

    #[test]
    fn test_fib() {}
}
