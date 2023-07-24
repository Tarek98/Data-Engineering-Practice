// To compile: rustc starter.rs
// To run: ./starter

use std::io;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::vec;

fn main() {
    if false {
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
    println!("\n");
    lifetimes();
}

// IMPORTANT: We read &a' str as "a string slice that lives at least as long as the lifetime 'a" -> 'a is inferred by the compiler.
// IMPORTANT: If a data type stores borrowed data, it must be annotated with a lifetime.
fn lifetimes() {
    // Rust compiler is usually able to determine how long variables will live.
    let s1 = String::from("abcd");
    let s2 = "xy";
    /*
    // In this case it cannot:
    let result = longest_broken(s1.as_str(), s2);
    */
    let result = longest(s1.as_str(), s2);
    // So sometimes we have to help the compiler by specifying lifetimes using annotations.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        // Lifetime annotations are written with apostrophe usually followed by a short name;
        // The 1st appearance says all params & return vals must have the same lifetime, then we say we accept strings that live at least as long as the 'a lifetime i.e. at least as long as the smallest of x & y.
        if x.len() > y.len() {
            x
        } else {
            y
        }
        // We're not breaking any rules here;
        // But we would be if we specify a lifetime as 'static i.e. the variable could live entire duration of program but it doesn't necessarily have to.
        // 'static can be used correctly to tell the compiler that a particular reference will always be valid.
        // IMPORTANT: you could use 'static to bandaid compiler errors even if suggested by compiler but you shouldn't. You should instead apply the correct
        //            lifetime annotations or fix the would-be dangling reference. The compiler could be complaining rightfully so because a reference isn't
        //            appropriate & you need to move, copy, or use something like Arc for the data instead.
        // WHY: Memory that's kept around forever that is no longer useful is like a memory leak.
    }
    println!("Longest string is '{}'", result);
    /*
       fn longest_broken(x: &str, y: &str) -> &str {
           // Compile error saying it can't figure out if the return value is the borrowing of x or y; It's not sure how long those string live.
           if x.len() > y.len() {
               x
           } else {
               y
           }
       }
    */
}
// IMPORTANT: If a data type stores borrowed data, it must be annotated with a lifetime. When possible, make data structures own their data directly.
#[derive(Debug)]
struct Highlight<'a>(&'a str);

// TODO: @Tarek: Continue review here.
fn thread_data_transfer() {
    // (1) Capturing:
    // Reference variables declared outside the current closure & thread context.
    // Compiler will try figure out how to make it work e.g. by borrowing or moving?
    // Or you can specify yourself how we can make it work e.g. move as follows:
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("vector v = {:?}", v);
    });
    handle.join().unwrap();
    println!("\n");

    // (2) Message Passing:
    // Safer than shared memory as it's harder to race or access inappropriate locations.
    // If we want to have multiple senders use clone on tx.
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    println!("\n");

    // (3) Shared State : Mutual Exclusion:
    // We want more than one thread to be able to modify a value concurrently.
    // Thread has to have the mutex to access the data.
    let m = Mutex::new(5);
    {
        // Mutex lock is acquired in current manual scope (braces open)
        let mut num = m.lock().unwrap();
        *num = 6;
        // Mutex lock is released after scope exits (braces close)
    }
    println!("m = {:?}", m);
    println!("\n");

    // (3) Shared State : Arc Mutex:
    // Use of mutex above is unnecessary since there's only 1 thread.
    // The intended use is to "move" it into multiple threads but that violates our rule about having only 1 owner.
    // So we have to break a rule! We need the ability to share ownership of some memory; This is done using the type Arc<T>.
    let quit = Arc::new(Mutex::new(false));
    let handler_quit = Arc::clone(&quit);
    ctrlc::set_handler(move || {
        // Dedicated signal handling thread where we execute the handler each time we receive a Ctrl+C signal.
        let mut b = handler_quit.lock().unwrap();
        *b = true;
        thread::sleep(Duration::from_millis(3000));
    })
    .expect("Error setting up the Ctrl-C handler");
    let mut x = 0;
    while !(*quit.lock().unwrap()) {
        println!(
            "Hello #{}, I'll stop this in 3 seconds if you hit Ctrl-C.",
            x
        );
        x += 1;
        thread::sleep(Duration::from_millis(1000));
    }
    println!("Good Bytes!");
    // There still exists possibility of deadlock in rust with mutexes. Nothing prevents thread 1 from acquiring mutex A then B,
    // and thread 2 from concurrently acquiring B then A.
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
    dbg!(x, y);

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
    println!("{}", base)
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
    println!("{}", day);
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
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let _input: u32 = input.trim().parse().expect("Failed : please type a number");
    }
    // Variables inside the block are dropped when we get here.

    // Lots of rust functions return Result types; They return Ok with type we expect, or Err with an error description.
    // To unpack the result: 3 ways to do this: match expression, unwrap(), and expect().
    // expect() lets us add an error message to make it easy to find where things go wrong.

    // Rust doesn't have exceptions, so we have to use something like expect().
}
