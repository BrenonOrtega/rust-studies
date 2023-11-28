// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::thread::{self, Thread};
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");
    // 1a. Between the .iter() and the .sum() add a .filter() with a closure to keep any even
    // number (`x % 2` will be 0 for even numbers).
    // 1b. Between the .filter() and the .sum() add a .map() with a closure to square the values
    // (multiply them by themselves)
    //
    // In the closures for both the .filter() and .map() the argument will be a reference, so you'll
    // either need to dereference the argument once in the parameter list like this: `|&x|` or you
    // will need to dereference it each time you use it in the expression like this: `*x`
    v.iter()
        // .filter() goes here
        // .map() goes here
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    // 2. Spawn a child thread and have it call `expensive_sum(my_vector)`.  Store the returned
    // join handle in a variable called `handle`. Once you've done this you should be able to run
    // the code and see the Child thread output in the middle of the main thread's letters
    //
    let handle = thread::spawn(|| { expensive_sum(my_vector) });
    let other_handle = thread::spawn(|| {
        let a = expensive_sum(vec![3, 5, 7, 9]);
        let b = expensive_sum(vec![0, 3, 2, 4, 6, 8]);
        println!("a value: {:?}", a);
        println!("b value: {:?}", b);

        return if a > 0 { a } else { b }
    });

    // While the child thread is running, the main thread will also do some work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }
    
    println!("Handle value: {:?}", handle);

    // 3. Let's retrieve the value returned by the child thread once it has exited.  Using the
    // `handle` variable you stored the join handle in earlier, call .join() to wait for the thread
    // to exit with a `Result<i32, Err>`.  Get the i32 out of the result and store it in a `sum`
    // variable.  Uncomment the println.  If you did 1a and 1b correctly, the sum should be 20.
    //
    let sum = handle.join();
    println!("The child thread's expensive sum is {}", sum.unwrap_or(0));

    // Time for some fun with threads and channels!  Though there is a primitive type of channel
    // in the std::sync::mpsc module, I recommend always using channels from the crossbeam crate,
    // which is what we will use here.
    //
    // 4. Uncomment the block comment below (Find and remove the `/*` and `*/`).  Examine how the
    // flow of execution works.  Once you understand it, alter the values passed to the `pause_ms()`
    // calls so that both the "Thread B" outputs occur before the "Thread A" outputs.

    
    let (tx, rx) = channel::unbounded();
    // Cloning a channel makes another variable connected to that end of the channel so that you can
    // send it to another thread.
    let tx2 = tx.clone();

    let handle_a = thread::spawn(move || {
        pause_ms(0);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    pause_ms(100); // Make sure Thread A has time to get going before we spawn Thread B

    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // gets closed.  A Receiver channel is automatically closed once all Sender channels have been
    // closed.  Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    // Join the child threads for good hygiene.
    handle_a.join().unwrap();
    handle_b.join().unwrap();
    
    reproduce_previous_exercise_ordering_messages();

    // Challenge: Make two child threads and give them each a receiving end to a channel.  From the
    // main thread loop through several values and print each out and then send it to the channel.
    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
    // the child threads.
    println!("Main thread: Exiting.");

    let (tx, rx) = channel::unbounded::<&str>();

    let other_rx = rx.clone();

    let handle_1 = std::thread::spawn(move || loop {
        for msg in &rx {
            println!("{}", msg);

            if msg == "close1" {
            println!("closing channel 1");
            break;
            }
        }
    });

    let handle_2 = std::thread::spawn(move || loop {
        for msg in &other_rx {
            println!("{}", msg);

            if msg == "close2" {
            println!("closing channel 2");
            break;
            }
        }
    });

    for value in vec!("op1", "op2","op3","close1","op4","op5", "close2") {
        tx.send(value).unwrap();
    }

    drop(tx);

    handle_1.join().unwrap();
    handle_2.join().unwrap();
        
}

fn reproduce_previous_exercise_ordering_messages() {
    let (tx, rx) = channel::unbounded::<String>();

    let other_tx: channel::Sender<String> = tx.clone();
    let handle_1: thread::JoinHandle<()> = thread::spawn(move || {
        let msg = "{ \"id\": 1, \"aircraftId\": \"SPRKA\", \"flightLegs\": [1, 2, 3], \"operation\": \"Move\" }";

        tx.send(msg.to_string()).unwrap();
        pause_ms(50);

        let msg = &"{ \"id\": 3, \"aircraftId\": \"SPRKA\", \"flightLegs\": [6, 7], \"operation\": \"Move\" }";
        tx.send(msg.to_string()).unwrap();
        pause_ms(50);
    });

    let handle_2 = thread::spawn(move || {
        pause_ms(25);
        let msg = "{ \"id\": 2, \"aircraftId\": \"SPRKA\", \"flightLegs\": [4, 5], \"operation\": \"Move\" }".to_string();
        other_tx.send(msg).unwrap();
        pause_ms(35);
    
        let msg = "{ \"id\": 4, \"aircraftId\": \"SPRKA\", \"flightLegs\": [8, 9, 10], \"operation\": \"Move\" }".to_string();
        other_tx.send(msg).unwrap();
    });

    for msg in rx {
        println!("Executing operation on {}", msg);
    }

    handle_1.join().unwrap();
    handle_2.join().unwrap();
}