use std::fmt::Display;
use std::ops::Add;
use std::fmt::Debug;

struct Par<T> {
    x: T,
    y: T,
}

impl<T: Display> ToString for Par<T> {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl<T> Par<T>
where
    T: Add<Output = T>,
    T: Copy,
{
    fn sestej(&self, other: &Self) -> Par<T> {
        Par {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct AritmeticnoZaporedje<T>
where 
T : Debug,
{
    a0: T,
    d: T,
    ai: T,
}

impl<T> Sequence<T> for AritmeticnoZaporedje<T>
where 
T: Debug + Clone + std::ops::Sub + PartialOrd, {
    fn new(a00:T, dd: T) -> Self 
    {
        Self{a0 :a00.clone(), d : dd, ai : a00}
    }

    fn start(&self) -> T {
        unimplemented!();
    }

    fn k_th(self: &Self, _n:usize) -> Option<T> {
        unimplemented!();
        //&(self.a0 + self.d * n)
    }

    fn contains(&self, item : T) -> bool {
        if (((*self).d)-item) >= 0 {

        }
    }
    fn name(&self) -> String {
        format!("a0: {:?}, d : {:?}", self.a0, self.d)
    }
}

    /*
    fn reset(self: &mut Self) -> () {
        self.ai = self.a0;
    }

    fn current(self: &Self) -> i32 {
        self.ai
    }

    fn sum(self: &Self, n: i32) -> i32 {
        let mut vsota: i32 = 0;
        let mut trenutni_clen = self.a0;
        for i in (1..(n + 1)) {
            vsota += self.n_th(i);
        }
        vsota
    }

    fn vsota(self: &Self, other: &Self) -> Self {
        let novo_zap = AritmeticnoZaporedje {
            a0: self.a0 + other.a0,
            d: self.d + other.a0,
            ai: self.ai + other.ai,
        };
        novo_zap
    }

    fn produkt(self: &Self, other: &Self) -> Self {
        unimplemented!();
    }
}
*/
//////////////////////////////////////////////////////////////////////////////
enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}

impl Izraz {
    fn eval(self: &Self) -> u32 {
        match self {
            Izraz::Operacija(spremenljivka1, operacija, spremenljivka2) => match operacija {
                BinOperacija::Plus => (&spremenljivka1).eval() + (&spremenljivka2).eval(),
                BinOperacija::Minus => (&spremenljivka1).eval() - (&spremenljivka2).eval(),
                BinOperacija::Times => (&spremenljivka1).eval() * (&spremenljivka2).eval(),
            },
            Izraz::Konstanta(i) => *i,
        }
    }

    fn collect(self: &Self) -> u32 {
        match self {
            Izraz::Operacija(levi_izraz, _, desni_izraz) => {
                (levi_izraz).collect() + (desni_izraz).collect()
            }
            Izraz::Konstanta(_) => 1,
        }
    }}
//////////////////////////////////////////////////////////////////////////////
trait Sequence<T>
where
T: Debug,{
fn new(a : T, b: T) -> AritmeticnoZaporedje<T>;
fn name(&self) -> String;
fn k_th(&self, k:usize) -> Option<T>;
fn contains(&self, item : T) -> bool;
fn start(&self) -> T;
}
// na ta način je zaporedje izračunljivo in neprazno

struct Constant<T> {
    c : T
}

struct ConstantInteger {
    c: i64
}

impl<T> Constant<T> {
    fn new(c: T) -> Constant<T>{
        Constant {c}
    }

}

/*
impl<T> Sequence<T> for Constant<T>
{
fn name(&self) -> String {
    format!("Constant")
}
fn k_th(nekaj:&Self, k:usize) -> Option<T> {
    return Some(nekaj.c);
}

fn contains(&self, item : T) -> bool {
    return self.c == item
}

fn start(&self) -> T;
}
*/

impl Sequence<i64> for Constant<i64> {
    fn new(a00 : i64, d:i64) -> AritmeticnoZaporedje<i64>{a0 : a00, d : 0, ai : a00}
    fn name(&self) -> String {
        format!("Constant")
    }
    fn k_th(&self, k:usize) -> Option<i64> {
        return Some(self.c);
    }
    
    fn contains(&self, item : i64) -> bool {
        return self.c == item
    }
    fn start(&self) -> i64 {
        return self.c
    }
    }

impl ConstantInteger {
    fn new(c: i64) -> ConstantInteger {

    }
}
//////////////////////////////////////////////////////////////////////////////


fn main() {
    println!("Hello, world!");
}
