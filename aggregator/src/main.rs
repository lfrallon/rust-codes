use std::fmt::Display;
use aggregator::{SocialPost, Summary, NewsArticle};

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        username: String::from("horse_ebooks"),
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result1 = longest(string1.as_str(), string2);
    println!("The longest string is {result1}");

    let string3 = String::from("long string is long");

    {
        let string4 = String::from("xyz");
        let result2 = longest(string3.as_str(), string4.as_str());
        let result2_1 = longest_no_lifetime(string3.as_str(), string4.as_str());
        println!("The longest string is {result2}");
        println!("The longest no lifetime string is {result2_1}");
    }

    // lifetime error
    // let string5 = String::from("long string is long");
    // let result3;
    // // let string6; // fix
    // {
    //     let string6 = String::from("xyz");
    //     result3 = longest(string5.as_str(), string6.as_str());
    // }
    // println!("The longest string is {result3}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let important_excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    let important_excerpt_level = important_excerpt.level();
    let announce_and_return_part = important_excerpt.announce_and_return_part("Paldo this upcoming event.");

    println!("Important excerpt is {} and level {important_excerpt_level} with an announcement of {announce_and_return_part}", important_excerpt.part);

    let static_lifetime: &'static str = "I have a static lifetime.";

    println!("Static lifetime is {}", static_lifetime);

    let longest_announcement = longest_with_an_announcement("How could this be the longest announcement!", "John being mocked by some culted people!", "I just cannot grasp the audacity to excoriate someones feelings like that!");
    println!("{}", longest_announcement);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// no need to specify a lifetime if the first param is always returned
fn longest_no_lifetime<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}