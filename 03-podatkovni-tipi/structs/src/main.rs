//fn add_suffix(s: &mut String) -> &String {
//    s.push_str(" world");
//
//    s
//}
//
//fn main() {
//    let mut s = String::from("hello");
//
//    let s2 = add_suffix(&mut s);
//
//    println!("{}", s2);
//}

//## Osnovne strukture

// 1.

#[derive(Debug)]
struct AritmeticnoZaporedje {
    a0: i32,
    d: i32,
    ai: i32,
}

// 2.
// Pri aritmetičnem zaporedju lahko izračunamo naslednji člen zaporedja, pa
// n-ti člen zaporedja, pa dobimo prvi člen zaporedja in razliko, aritmetična
// zaporedja lahko tudi seštevamo, odštevamo, množimo ter dobimo artimetična zaporedja.

// 3.

impl AritmeticnoZaporedje {
    fn new(a0: i32, d: i32) -> Self {
        Self { a0, d, ai: a0 }
    }

    fn next(self: &mut Self) -> i32 {
        self.ai += self.d;
        self.ai
    }

    fn n_th(self: &Self, n: i32) -> i32 {
        self.a0 + self.d * n
    }

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

fn main() {
    let mut enostavno_zap = AritmeticnoZaporedje::new(0, 1);
    let c = AritmeticnoZaporedje::next(&mut enostavno_zap);
    println!("{c:?}");
    println!("{}", enostavno_zap.sum(10));
    let izraz1: Izraz = Izraz::Operacija(
        Box::new(Izraz::Konstanta(6)),
        BinOperacija::Times,
        Box::new(Izraz::Konstanta(7)),
    );
    //let izraz2: Izraz = Izraz::Operacija(
    //    Box::new(Izraz::Operacija)
    //)
}

// 5. Ne, pri splošnih zaporedjih tega ne moremo storiti, npr. ne moremo smiselno sešteti
// dveh zaporedij avtomobilov.

//##AST

enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}

const STEVILO: u8 = 3;
//let izraz1 : Izraz = Izraz::Operacija(Izraz::Konstanta(6), BinOperacija::Times, Izraz::Konstanta(7));

// 1. Tip Izraz brez boxov ne deluje, ker bi moral imeti Izraz določeno
// velikost, da bi jo lahko lahko shranil na sklad, ker pa je deficija
// tipa rekurzivna, moram zadevo z boxom shraniti na kopico. Torej je
// box potreben
