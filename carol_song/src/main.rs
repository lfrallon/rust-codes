fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    for (i, &day) in days.iter().enumerate() {
        println!("On the {} day of Christmas, my true love sent to me", day);
        for j in (0..=i).rev() {
            if i > 0 {
                if j == 0 {
                    println!("and ");
                }
            }
            if j == 1 {
                print!("{} ", gifts[j]);
            } else {
                println!("{} ", gifts[j]);
            }
        }
        println!();
    }
}
