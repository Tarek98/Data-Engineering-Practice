// To compile: rustc starter.rs
// To run: ./starter

use std::io;
use std::thread;
use std::time::Duration;
use std::vec;
use std::sync::mpsc;

fn main() {
    println!("\n");
    println!("Hello World!");
    println!("\n");
    print_number();
    println!("\n");
    shadowing();
    println!("\n");
    ownership_transfer();
    println!("\n");
    string_slice();
    println!("\n");
    spawn_hello_threads();
    println!("\n");
    thread_data_transfer();
}

fn thread_data_transfer() {
    // (1) Capturing: reference variables declared outside the current closure & thread context.
    //                compiler will try figure out how to make it work e.g. by borrowing or moving?
    //                or you can specify yourself how we can make it work e.g. move as follows:
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("vector v = {:?}", v);
    });
    handle.join().unwrap();
    
    // (2) Message Passing: safer than shared memory as it's harder to race or access inappropriate locations.
    //                      if we want to have multiple senders use clone on tx.
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // (3) Shared State 
    // ...?
}

fn spawn_hello_threads() {
    // Main & spawned thread run concurrently, so order of prints is not always same.
    
    let thread_handle = thread::spawn(|| {
        for i in 1..3 {
            println!("Hello #{} from spawned thread", i);
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..3 {
        println!("Hello #{} from main thread", i);
        thread::sleep(Duration::from_millis(1))
    }

    // Wait for spawned thread to finish
    thread_handle.join().unwrap();
}

fn ownership_transfer() {
    // (1) Copy Semantics: for simple types (e.g. int, bool, float), y makes a copy of x. In Rust, copy semantics is generally the exception not the rule.
    let x = 5;
    let y = x;
    dbg!(x,y);

    // (2) Move Semantics: for heap-allocated types, s1 & s2 point to the same thing. BUT s1 can no longer be used as ownership moved to s2.
    let s1 = String::from("hello");
    let _s2 = s1;

    // (3) Borrowing: Acquire the variable s3 owns by reference (&) but promise to give it back (compiler's borrow checker forces you to uphold the promise).
    //                s4 points to s3 which points to "sharing is caring".
    let s3 = String::from("sharing is caring");
    let len = calc_length(&s3);
    println!("Length of '{}' is {}", s3, len);
    /*---*/
    let mut base = String::from("share the workload");
    extend_str_with_length(&mut base);
    println!("{}",base)
}

fn calc_length(s4: &String) -> usize {
    // s4 is an immutable reference : we cannot change it even if underlying data is mutable.
    s4.len()
}

fn extend_str_with_length(base: &mut String) {
    // base is a mutable reference : for 1 variable there can only be 1 mutable reference at a time & 0 immutable references while that is the case.
    // there can be many immutable references at the same time if there are 0 mutable references.
    let extension = format!(" (len = {})", base.len());
    base.push_str(&extension)
}

fn string_slice() {
    let s = String::from("good day");
    let good = &s[0..4];
    let day = &s[5..8];
    println!("{}", good);
    println!("{}", day)
}

fn print_number() {
    // Variables are immutable by default & semicolons are only needed for separating lines.
    let a = 5;
    println!("{}", a);
}

fn shadowing() {
    // Shadowing: feature allowing you to reuse variable names; Old variable can no longer be used.

    // Variables inside the block do not exist yet here.
    {
        let mut input = String::new();
        println!("Please input your favorite number. It will be parsed from String to Int.");
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let _input: u32 = input.trim().parse()
            .expect("Failed : please type a number");
    }
    // Variables inside the block are dropped when we get here.

    // Lots of rust functions return Result types; They return Ok with type we expect, or Err with an error description.
    // To unpack the result: 3 ways to do this: match expression, unwrap(), and expect().
    // expect() lets us add an error message to make it easy to find where things go wrong.

    // Rust doesn't have exceptions, so we have to use something like expect().
}
