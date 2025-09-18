fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a leaping",
        "Nine ladies dancing",
        "Eight maids a milking",
        "Seven swans a swimming",
        "Six geese a laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "A partridge in a pear tree",
    ];

    for i in 0..12 {
        println!("\nOn the {} day of Christmas, my true love gave to me:", days[i]);

        for j in (12 - i - 1)..12 {
            if j == 11 && i != 0 {
                println!("And {}", gifts[j]);
            } else {
                println!("{}", gifts[j]);
            }
        }
    }
}
