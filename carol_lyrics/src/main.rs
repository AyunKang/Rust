// "The Twelve Days of Christmas" 가사 출력
fn main() {
    //첫번째 가사 출력
    println!(
        "\n\nOn the first day of Christmas,\nmy true love gave to me\nA partridge in a pear tree.\n\n"
    );

    // 반복될 가사 배열
    let base_lyrics = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days = [
        "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
        "eleventh", "twelfth",
    ];

    let mut str = String::new();

    // 12번 반복하며 가사 출력
    for i in 0..11 {
        println!(
            "On the {} day of Christmas,\nmy true love gave to me",
            days[i]
        );
        if i >= 1 {
            str = str + "\n" + base_lyrics[i] + ",";
        } else {
            str = str + base_lyrics[i] + ",";
        }
        println!("{}", str);
        println!("And a partridge in a pear tree.\n\n");
    }
}
