```rust
fn main() {

    let x = 10;
    x = 20; 

    let s1 = String::from("hello");
    let s2 = s1;
    println!("s1: {}", s1);

    let mut data = vec![1, 2, 3];
    let first = &data[0];
    data.push(4);
    println!("The first element is: {}", first);

    let mut number = 5;
    let r1 = &mut number;
    let r2 = &mut number;
    *r1 += 1;
    *r2 += 1;

    let text = String::from("Rust");
    let text_ref = &text;
    text_ref.push_str(" is great!");
    
    let _dangling = gr1();
}

fn gr1<'a>() -> &'a String {
    let local_string = String::from("temporary");
    &local_string
}
```