pub const TERMINAL_WIDTH: usize = 120;

#[macro_export] macro_rules! run_example {
    ($expression:expr) => {
        let as_str = stringify!($expression);
        println!("BEGIN {:-^1$}", as_str, TERMINAL_WIDTH - 5);
        let res = $expression;
        println!("END {:-^1$}", as_str, TERMINAL_WIDTH - 3);
        
    };
}

const TEST1: u32 = 42;

#[test]
fn ref_and_ptr(){
    let x = 42;
    let r_x = &x;
    let ptr_x: *const i32 = &x;

    //?: a 
    println!("x: {}, r_x: {}, ptr_x: {:?}", x, r_x, ptr_x);
}

#[test]
fn string_slice(){
    let mystring = String::from("value");
    let slc = &mystring[1..4];
    println!("Slc: {}", slc);
}


fn shadowing() {
    let n = 42;
    println!("n1: {n}");
    let n = 43;
    println!("n2: {n}");
}

fn func_with_mutable_arg(a1: &mut i32) {
    *a1 = 42;
}

fn call_with_mutable_arg() {
    let mut a1 = 0;
    func_with_mutable_arg(&mut a1);
    println!("a1: {}", a1);
}

fn match_simple() {
    let n = 42;
    match n {
        3 => println!("Matched π"),
        42 => println!("Matched 42"),
        43 | 45 => println!("Matched 43 or 45"),
        n1 @ 4..=5 => println!("Matched {}", n1),
        n2 @ 6..=8 if n2 % 2 == 0 => println!("Matched {}", n2), //guard
        _ => {
            println!("not 42 or π")
        }
    }
}

fn match_as_expr(n: i32) {
    let res = match n {
        42 => 3,
        3 => 42,
        _ => n,
    };
    println!("{}", res);
}

fn loop_as_expr() {
    let res = loop {
        break 42;
    };

    println!("loop: {}", res);

    //But not while/for loops!
    // let mut n = 1;
    // let res = while n < 42{
    //     n += 1;
    //     break n;
    // };
}

fn if_simple() {
    let mut n = 42;

    if n < 42 {
        println!("n is less than 42");
    } else {
        println!("n is not less than 42");
    }
}

fn if_as_expr() {
    let res = if 41 < 42 { true } else { false };
    println!("res: {}", res);
}

fn let_patterns() {
    let (x, y, z) = (3.0, 42, "asdf");
    let (x, mut y, z) = (3.0, 42, "asdf");
    let (x1, ..) = (3.0, 42, "asdf");
    let (x2, _, z2) = (3.0, 42, "asdf");
    y = 43;
    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn match_1() {
    let pontszám = 42;
    match pontszám {
        0..50 => println!("elégtelen"),
        50..60 => println!("elégséges"),
        60..75 => println!("közepes"),
        75..85 => println!("jó"),
        85..=100 => println!("jeles"),
        _ => println!("nem_teljesítette"),
    }
}

struct ToDestructure {
    x: u32,
    y: isize,
    w: String,
}

fn match_2() {
    let to_destructure = ToDestructure {
        x: 0,
        y: 0,
        w: "42".to_string(),
    };
    //warningot eredmenyez, mert biztosan true az if feltetele
    if let ToDestructure { x, y, .. } = to_destructure {
        println!("x: {x}, y: {y}");
    }

    if let ToDestructure { x: 0, y, .. } = to_destructure {
        println!("x was 0, y: {y}");
    }
}

pub fn match_multiple() {
    let x = 42;
    match x {
        42 | 45 => println!("true"),
        _ => {}
    }
}

#[test]
pub fn run_all() {
    run_example!(call_with_mutable_arg());

    run_example!(match_simple());
    run_example!(match_as_expr(42));

    run_example!(if_as_expr());

    run_example!(loop_as_expr());

    run_example!(shadowing());

    run_example!(match_1());
}

fn main(){}