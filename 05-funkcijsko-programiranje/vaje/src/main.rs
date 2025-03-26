// Napišite različne funkcije, ki kot argument sprejmejo zaprtje (closure) in ga pokličejo z različnimi argumenti, ki ustrezajo sledečim ocaml tipom:
// Tip zaprtja naj bo čim bolj splošen (Fn, FnOnce, FnMut).
//  apply_int: (int -> int) -> int -> int
//  apply: ('a -> 'b) -> 'a -> 'b
//  apply2: ('a -> 'a -> 'b) -> 'a -> 'a -> 'b
//  map: ('a -> 'b) -> 'a list -> 'b list  (Uporabite Vec<T> namesto list, predpostavite, da funkcija ne spremeni elementov seznama)
//  ponavljaj: int -> ('a -> 'a) -> 'a -> 'a // Ponovi funkcijo n-krat
//  filter: ('a -> bool) -> 'a list -> 'a list // Vrne seznam elementov, ki zadoščajo pogoju - uporabite Vec<T> namesto list in že vgrajeno funkcijo filter

// Vzemite zaporedja iz prejšnjih vaj in naredite nov objekt, ki sprejme zaporedje in ga naredi iterabilnega

// Iteratorji

// Napišite funkcijo, ki sprejme vektor XYZ in s pomočjo iteratorja naredi W
// števil in izpiše vsako v svojo vrstico
// nizov in izpiše njihove dolžine
// nizov in vrne vsoto njihovih dolžin
// vektor parov (i32, i32) in vrne vsoto njihovih pozitivnih produktov
// dva vektorja <i32> in vrne vektor, ki vsebuje vsote parov
// dva vektorja <i32> in vrne vsoto poparjenih pozitivni produktov s pomočjo ene izmed prejšnjih nalog
// vektor Option<T> in izpiše vse T-je
// vektor Option<T> in vrne število Some-ov
// odfiltrira števila deljena s 3

// Popravite zaporedja iz prejšnjih vaj, da bodo `iterabilna`

// Dopolnite spodnjo funkcijo, da vrne niz, kjer so vse prve črke posameznih besed velike
// ["Just,", " ", "hello", " ", "world", "!"] -> "Just, Hello World", "!"
// pub fn capitalize_words_string(words: &[&str]) -> String {
//     panic!("Not implemented");
// }
// Napišite funkcijo `fakulteta`, ki izračuna fakulteto števila n. Uporabite iteratorje (torej brez lastne for zanke, rekurzije)
// Namig: fold, reduce, `..`...

// -------------------------------------------------------------------------------------------------
// Dodatno:
// Koda vzeta iz googlvih rust vaj:
// Vse se da lepo narediti samo z iteratorji (brez indeksov, brez for zank, brez mutabilnosti)
/*
/// Calculate the differences between elements of `values` offset by `offset`,
/// wrapping around from the end of `values` to the beginning.
///
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
fn offset_differences<N>(offset: usize, values: Vec<N>) -> Vec<N>
where
    N: Copy + std::ops::Sub<Output = N>,
{
    unimplemented!()
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_custom_type() {
    assert_eq!(
        offset_differences(1, vec![1.0, 11.0, 5.0, 0.0]),
        vec![10.0, -6.0, -5.0, 1.0]
    );
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}



*/

// Apply

fn apply_int(f: impl Fn(i64) -> i64, x : i64) -> i64 {
f(x)
}

fn apply_int2<F>(f: F, x:i64) -> i64
where 
F: Fn(i64) -> i64,
{
    f(x)
}

fn apply_int_dyn(f: &dyn Fn(i64) -> i64, x:i64) -> i64 {
    return f(x)
}

fn apply_int_dyn2<T,U>(f: &dyn Fn(T) -> U, x:T) -> U {
    return f(x)
}


fn apply_int3<T, U>(f: impl Fn(T) -> U, x : T) -> U {
    f(x)
    }

/* Razlika med impl in &dyn:
pri impl rabimo struct, ki implementira Fn(T) -> U, 
pri &dyn pa kažemo na nekaj, kar kaže na nekaj, kar se zna odzvati na 
Fn(T) -> U
&dyn je počasnejši
*/

//map

fn map<T>(f : impl Fn(&T) -> T, vc : Vec<T>) -> Vec<T>
where 
T: std::fmt::Debug, {
    let vct: Vec<T> = vc.iter().map(f).collect();
    println!("Vektor vc je: {:?}", vc);
    vct

}

// .iter vzame referenco in ne prevzame lastništva

//ponavljaj

fn ponavljaj<T>(n: i64, f: impl Fn(T) -> T, x: T) -> T {
    unimplemented!()
}

fn main(){
let a = |x:i64| {x*2};
println!("impl: {}, dyn: {}", 
apply_int(a, 10), 
apply_int_dyn(&a, 10));
let b = |x: i64| {x*10};
let fncs = vec![a, b];
println!("impl: {}, dyn: {}", 
apply_int(fncs[0], 10), 
apply_int_dyn(&fncs[1], 10));

let v1 = vec![1, 2, 3];
let v2 = map(|x| {x+1}, v1);
println!{"{:?}", v2};
/*println!{"{:?}", v1}; 
Tega ne moremo napisati, ker map prevzame lastništvo*/
}
