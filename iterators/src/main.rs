use iterators::{shoes_in_size};
use iterators::Shoe;

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    let shoes = vec![
            Shoe {
                size: 5,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

    let in_my_size = shoes_in_size(shoes, 10);
    println!("Shoes in my size: {:#?}", in_my_size);
}
