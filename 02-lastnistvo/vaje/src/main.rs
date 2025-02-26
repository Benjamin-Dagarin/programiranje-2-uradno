use std::time::{Duration, Instant};

fn time_it<F: FnOnce() -> R, R>(f: F) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

fn on_stack() {
    // Narišite shemo spreminjanja sklada in kopice
    // Za vsako vrstico napiši, kolikokrat se v pomnilniku pojavi 13?
    let mut a = [13; 100]; // array, ki ima sto trinajstk. Računalnik da to na sklad, ker je array fiksne dolžine in računalnik ve, koliko prostora mora rezervirati.
    let mut b = a; // tu ne gre za prenos lastništva, ker a ne leži v kopici. Namesto tega se a kopira v b. Torej se cel array skopira in je v pomnilniku zdaj 200 trinajstk
    let q = String::from("13"); // q živi na kopici, tj. q kaže na neko mesto v kopici, kjer je shranjena ena trinajstka (201)
    println!("{}", q); // tukaj gledamo referenco in ne prenašamo lastništva. lahko bi napisali tudi z *
    let r = q; // prevzem lastništva (201)
    let p = &r; // tukaj naredimo referenco, tj. zdaj p samo kaže na isto vrednost kot r, vendar je r še vedno lastnik (201)
    a[0] = 1; //ker smo kopirali a v b, se je spremenil samo a, zato je zdaj v pomnilniku (200) trinajstk
              // uporaba *
    {
        let c = &b;
        println!("{}", c[0]);
    }
    println!("{}", b[0]);
    println!("{}", a[0]);
    println!("{}", p);
    println!("{}", r);
    // println!("{}", q); // Razloži, zakaj to ne deluje -> ker q nima več lastništva in nima ničesar notri.
    // na koncu imamo v pomnilniku 200 trinajstk
}

/// Napišite funkcijo `swap`, ki zamenja vrednosti dveh celoštevilskih spremenljivk.
fn swap(x: &mut i32, y: &mut i32) {
    //mut pomeni, da bomo argumenta spreminjali, & pa, da bomo uporabili referenco
    // return (b,a);
    let c = *x; // *x - odpakirali smo x
    *x = *y;
    *y = c;
}

fn test_swap() {
    let mut a = 13;
    let mut b = 42;
    println!("a: {}, b: {}", a, b);
    //let (a, b) = swap(a, b);

    println!("a: {}, b: {}", a, b);

    // V spremenljivko `a` shranite vrednost 13, v spremenljivko `b` pa vrednost 42.

    // println!("a: {}, b: {}", a, b);
    // Izpiše `a: 13, b: 42`.

    // Naredite swap s pomočjo pomožne funkcije `swap`.
    // ...
    //

    // println!("a: {}, b: {}", a, b);
    // Izpiše `a: 42, b: 13`.
}

/// Popravite zakomentiran del spodnje funkcije, da bo deloval
fn str_own() {
    let x = String::from("Hello world"); // na kopici se ustvari string "Hello world"
    let y = &x;
    println!("{}, {}", x, y);
}

/// Popravite brez uporabe funkcije `clone`
/// Namig: sklad in kopiranje na skladu - kodo lahko spremenite
fn str_own2() {
    // z refernciranjem x
    let x = (1, 2, (), String::from("Hello world"));
    let y = &x;
    println!("{:?}, {:?}", x, y);

    //brez referenciranja x
    //let x = (1, 2, (), "Hello world"); // "Hello world" je tipa &str, ki je v bistvu array znakov
    //let y = x;
    //println!("{:?}, {:?}", x, y);
}

/// Popravite spodnji dve funkciji, da bosta delovali

//fn wrong() {
// Nekaj sem narobe rešil!!!
//    let s = String::from("Hello World");
//    // ena opcija: print_str(s.clone()); kopiranje je potratno v primerjavi z referencami
//    print_str(&s);
//    println!("{}", s);
//}

fn print_str(s: String) {
    println!("{}", s)
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala
fn fn1() {
    let s = String::from("Hello ");
    let mut s1 = s;
    s1.push_str("World!");
    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala

fn fn2() {
    let x = Box::new(5);

    // Popravite zgolj tukaj vmes
    let mut y = Box::new(42);
    //// //
    *y = 4;
    //
    assert_eq!(*x, 5);
    //
    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------

fn fn3() {
    let t = (
        String::from("hello"),
        String::from("world"),
        String::from("!"),
    );

    let _s = t.1; //_s pomeni, da rustu damo vedeti, da s-a ne bomo uporabili // _s prevzame lastništvo

    // Izpišite čim večji del t-ja.
    println!("{}{}", t.0, t.2); // t v tem trenutku ne moremo več naprintati, ker si ne lasti vseh svojih komponent
}

/// ------------------------------------------------------------------------------------------------

fn fn4() {
    let x = 5;
    // Izpišite naslov spremenljivke x
    println! {"{:p}", &x};
}

/// ------------------------------------------------------------------------------------------------

fn fn5() {
    let x = 13;
    let y = &x;

    // Popravite spodnjo vrstico, da bo bo enakost držala
    assert_eq!(13, *y);
}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper` se mora poklicati čim bolj učinkovito.
fn fn6() {
    let s = String::from("hello, ");

    helper(&s); //tako ne rabimo kopirati ali nekaj takega?

    println!("Success!");
}

// Te funkcije ne spreminjajte
fn helper(s: &String) {}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper2` se mora poklicati čim bolj učinkovito.
fn fn7() {
    let mut s = String::from("hello, ");

    helper2(&mut s); //neučinkovit način: helper(&mut s.clone())

    println!("Success!");
}
// Te funkcije ne spreminjajte
fn helper2(s: &mut String) {
    s.push_str("world")
}

/// ------------------------------------------------------------------------------------------------

/// Pojasnite, zakaj spodnja koda ne deluje - smo že popravili kodo
fn fn8() {
    let mut s = String::from("hello, ");

    {
        let p = &mut s;

        p.push_str("world"); // problem je, ker p ni mutable
    }
    println!("Success! {}", s);
    println!("Success! {}", &mut s);
    s.push_str("!");
}

//Opomba: rust običajno * sam postavlja tja, kamor spadajo

/// ------------------------------------------------------------------------------------------------
/// Pojasnite, zakaj spodnja koda ne deluje in jo popravite
/// Narobe je, da smo si dvakrta sposodili s
/// Pojasnite tudi zakaj je popravek ok

fn fn9() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        print!("{r1}")
    }
    let r2 = &mut s;

    println!(", {}", r2);

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------
fn fn10() {
    // // Popravite spodnjo vrstico
    // let s = String::from("hello, ");

    // helper3(&mut s);

    // println!("Success!");
}

fn helper3(s: &mut String) {}

/// ------------------------------------------------------------------------------------------------

fn main() {
    fn8();
}
