const GIFTS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five golden rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];
const ORDINAL_DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

fn compose_lyrics(gifts: [&str; 12], ordinal_days: [&str; 12]) -> Vec<String> {
    let mut lyrics: Vec<String> = Vec::new();

    for (index, day) in ordinal_days.into_iter().enumerate() {
        let mut verse: String = String::new();
        verse.push_str(&format!(
            "on the {0} day of christmas my true love sent to me ",
            day
        ));
        if index == 0 {
            verse.push_str(&format!("{0}", gifts[0]));
            for (_ind, element) in verse.split_whitespace().enumerate() {
                lyrics.push(element.to_string());
            }
            continue;
        }
        for (i, _gift) in gifts.into_iter().enumerate() {
            if i <= index && (index - i) != 0 {
                verse.push_str(&format!("{0} ", gifts[index - i]));
            }
        }
        verse.push_str(&format!("and {0}", gifts[0]));
        for (_ind, element) in verse.split_whitespace().enumerate() {
            lyrics.push(element.to_string());
        }
    }
    split_into_syllables(lyrics)
}
fn split_into_syllables(lyrics: Vec<String>) -> Vec<String> {
    let syllabized_lyrics: Vec<String> = Vec::new();
    for (_index, word) in lyrics.into_iter().enumerate() {
        // split on hyphen
        let hyphenated_word: Vec<&str> = word.split("-").collect();
        println!("{:?}", hyphenated_word);
        // println!("{0}", word);
    }
    syllabized_lyrics
}
fn sing_song(lyrics: Vec<String>) {
    for syllable in lyrics {
        todo!("{}", syllable);
    }
}

fn main() {
    sing_song(compose_lyrics(GIFTS, ORDINAL_DAYS));
}
