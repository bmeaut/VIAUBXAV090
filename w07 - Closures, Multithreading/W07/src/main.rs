use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use std::thread::spawn;
use std::time::Instant;

fn main() {

    println!("Hello, world!");


    let handle = thread::spawn(||{
       for i in 1..10 {
           println!("number {} from the spawned thread!",i);
       } 
    });

    for i in 1..5 {
        println!("number {} from the main thread!",i);
    }
    
    handle.join().expect("Could not join thread!");




    let (ch1tx, ch1rx) = mpsc::channel::<String>();

    let ch1tx1 = ch1tx.clone();
    let ch1tx2 = ch1tx.clone();

    let handle1 = thread::spawn(move || {
       ch1tx1.send(String::from("Hello")).unwrap();
    });

    let handle2 = thread::spawn(move || {
        ch1tx2.send(String::from("world!")).unwrap();
    });

    let handle3 = thread::spawn(move || {
        // loop{
        //     let recv_val = ch1rx.recv().unwrap();
        //     println!("Received: {}",recv_val);
        //     if recv_val == "Exit" {
        //         break;
        //     }
        // }
        for received in ch1rx {
            println!("Got: {}", received);
        }
    });


    handle1.join().expect("Could not wait for thread!");
    handle2.join().expect("Could not wait for thread!");

    ch1tx.send(String::from("Exit")).unwrap();
    drop(ch1tx); //így el tudunk dobni kézzel is értékeket

    handle3.join().expect("Could not wait for thread!");



    //#region mutex_single
    let mtx1 = Mutex::new(String::from(""));

    {
        //Nem sikerülne, ha már lenne máshol zár
        let mut handle_mtx1 = mtx1.lock().unwrap();
        *handle_mtx1 = String::from("Hello");
    }



    let arcmut1 = Arc::new(Mutex::new(String::from("")));
    let mut handles = vec![];

    for i in 0..10 {
        let arcmut1_t = arcmut1.clone();
        let hndl = spawn(move || {
            let mut mutl = arcmut1_t.lock().unwrap();
            mutl.push_str(&(i.to_string() + " "));
        });
        handles.push(hndl);
    }

    for hndl in handles {
        hndl.join().expect("Could not join thread!");
    }


    println!("{:?}", arcmut1.lock().unwrap());


    let handle_ret = spawn(move || {
        "42"
    });

    let res = handle_ret.join().expect("Could not join thread!");
    println!("Received result {:?}", res);

    let mut c = NoCopy{c: String::from("42")};

    let now1 = Instant::now();

    take_fn_once_as_arg(|a|{
        let mut c = c;
        println!("Got a {} and captured {:?}", a, c);
        c.c.push_str("42");
        a
    });

    println!("{:?}", now1.elapsed().as_secs_f64());
    // take_fn_mut_as_arg(|a|{
    //     println!("Got a {} and captured {:?}", a, c);
    //     c.c.push_str("42");
    //     a
    // });

    // c.c.push_str("42");

}
#[derive(Debug)]
struct NoCopy{
    c: String
}

fn take_fn_as_arg(a1: impl Fn(i32) -> i32) -> i32 {
    a1(10);
    return a1(10);
}


fn take_fn_mut_as_arg(mut a1: impl FnMut(i32) -> i32) -> i32 {
    a1(10);
    return a1(10);
}

fn take_fn_once_as_arg(a1: impl FnOnce(i32) -> i32) -> i32 {
    return a1(10);
}
