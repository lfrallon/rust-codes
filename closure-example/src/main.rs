use std::thread;

impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}

fn main() {
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    // let example_closure = |x| x;

    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");

    // let only_borrows = || println!("From closure: {list:?}");

    // println!("Before calling closure: {list:?}");
    // only_borrows();
    // println!("After calling closure: {list:?}");

    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");

    // let mut borrows_mutably = || list.push(7);

    // borrows_mutably();
    // println!("After calling closure: {list:?}");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
