// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
// taking advantage of therepetition in the song.
fn main() {

    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
                "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let verses = ["Twelve drummers drumming", "Eleven pipers piping",
                "Ten lords a-leaping", "Nine ladies dancing", "Eight maids a-milking",
                "Seven swans a-swimming", "Six geese a-laying", "Five golden rings",
                "Four calling birds", "Three french hens", "Two turtle doves, and",
                "A partridge in a pear tree"];

    let mut countdown = verses.len() - 1;
    for day in days {
        println!("In the {day} day of Christmas\nMy true love sent to me");

        for index in countdown..12 {
            println!("{}", verses[index]);
        }

        println!("");
        println!("");
        if countdown > 0 { countdown -= 1 }
    }

}
