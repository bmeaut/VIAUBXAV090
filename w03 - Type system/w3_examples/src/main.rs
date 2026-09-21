//#region primitives

use std::{ffi::CString, fmt::Display, vec};

#[test]
fn literals() {
    let default_number = 42;
    let unsigned_32bit_number = 42u32;
    let signed_64bit_number = 42i64;

    let float = 42.0;
    let float_32 = 42f32;
}

#[test]
fn strings() {
    let simple_str: &str = "asdf1324😂";
    let simple_str_ptr = simple_str.as_ptr(); //ezt jellemzően nem célszerű átadni C függvényeknek
    println!("The length of simple_str: {}", simple_str.len());

    let heap_str = String::from("asdf");
    let heap_str = String::from(simple_str);

    let byte_str = b"asdf1234:D";
    //Ez a string típus kompatibilis a C/C++ stringekkel!
    let c_str = CString::new(byte_str).expect("byte_str was 0!");
    let c_str_ptr = c_str.as_ptr(); //ezt átadhatjuk C interop híváskor

    let and_another_one = "another one";
    let formatted_literal = format!("{} and {}", simple_str, and_another_one);
}

//#endregion primitives

//#region enums

enum YesOrNo {
    Yes,
    No,
}

enum IpAddr {
    Ipv4Addr([u8; 4]),
    Ipv6Addr([u16; 8]),
}

impl IpAddr {

    //ez itt egy inherent method
    fn get_bytes(&self) -> Vec<u8> {
        match self {
            IpAddr::Ipv4Addr(addr) => addr.to_vec(),
            IpAddr::Ipv6Addr(addr) => addr.iter().map(|a| a.to_ne_bytes()).flatten().collect(),
        }
    }
}

#[test]
fn enums() {
    let addr1 = IpAddr::Ipv4Addr([192, 168, 0, 1]);
}

//#endregion


//#region trait_examples

//Ez itt egy trait
trait Printable {
    fn print(&self);
    fn println(&self) {
        self.print();
        println!();
    }
}

//Ez pedig az implementációja egy típushoz
impl Printable for i32 {
    fn print(&self) {
        print!("{} ", self);
    }
}

trait PrintTwice {
    fn print2(&self);
}

impl<T: Printable> PrintTwice for T {
    fn print2(&self) {
        self.print();
        self.println();
    }
}

#[test]
fn test_printable() {
    let mut v1: i32 = 0;
}

//#endregion trait_examples

//#region conversion

#[test]
fn conversion() {
    let mut a: u32 = 42;
    let b = 42u64;
    a = b as u32;

    println!("a: {}", a);

    let b: String = "asdf".into();
    println!("B: {}", b);

    let c: String = String::from("asdf");
    println!("C: {}", c);


}

//#endregion conversion


//#region union
union IntOrFloatOrSlice {
    v0: [u8;4],
    v1: u32,
    v2: f32
} //Nem garantáltan olyan a reprezentáció mint C-ben! https://github.com/rust-lang/unsafe-code-guidelines/blob/c138499c1de03b908dfe719a41193c84f8146883/reference/src/layout/unions.md

union RandomUnion {
    v0: u64,
    v1: u32
}

//#endregion



fn main() {
    println!("Hello, world!");
}
