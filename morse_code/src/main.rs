#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

/// Represents a single character
type Letter = Vec<Pulse>;

/// Represents a string of characters
type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        use Pulse::{Long, Short};

        let mut message = vec![];

        for character in self.chars() {
            message.push(match character.to_ascii_lowercase() {
                'a' => vec![Short, Long],
                'b' => vec![Long, Short, Short, Short],
                'c' => vec![Long, Short, Long, Short],
                'd' => vec![Long, Short, Short],
                'e' => vec![Short],
                'f' => vec![Short, Short, Short, Long, Short],
                'g' => vec![Long, Long, Short],
                'h' => vec![Short, Short, Short, Short],
                'i' => vec![Short, Short],
                'j' => vec![Short, Long, Long, Long],
                'k' => vec![Long, Short, Long],
                'l' => vec![Short, Long, Short, Short],
                'm' => vec![Long, Long],
                'n' => vec![Long, Short],
                'o' => vec![Long, Long, Long],
                'p' => vec![Short, Long, Long, Short],
                'q' => vec![Long, Long, Short, Long],
                'r' => vec![Short, Long, Short],
                's' => vec![Short, Short, Short],
                't' => vec![Long],
                'u' => vec![Short, Short, Long],
                'v' => vec![Short, Short, Short, Long],
                'w' => vec![Short, Long, Long],
                'x' => vec![Long, Short, Short, Long],
                'y' => vec![Long, Short, Long, Long],
                'z' => vec![Long, Long, Short, Short],
                '1' => vec![Short, Long, Long, Long, Long],
                '2' => vec![Short, Short, Long, Long, Long],
                '3' => vec![Short, Short, Short, Long, Long],
                '4' => vec![Short, Short, Short, Short, Long],
                '5' => vec![Short, Short, Short, Short, Short],
                '6' => vec![Long, Short, Short, Short, Short],
                '7' => vec![Long, Long, Short, Short, Short],
                '8' => vec![Long, Long, Long, Short, Short],
                '9' => vec![Long, Long, Long, Long, Short],
                '0' => vec![Long, Long, Long, Long, Long],
                _ => continue,
            });
        }

        message
    }
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Short => write!(f, "."),
            Self::Long => write!(f, "_"),
        }
    }
}

fn print_morse_code(code: &Message) {
    for letter in code {
        for pulse in letter {
            print!("{pulse}");
        }
        print!(" ");
    }
    println!();
}

fn main() {
    let greeting = "Hello, world".to_string().to_morse_code();

    print_morse_code(&greeting);
}

#[test]
fn hello_world() {
    use Pulse::*;

    let expected = vec![
        vec![Short, Short, Short, Short],
        vec![Short],
        vec![Short, Long, Short, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Long, Long],
        vec![Short, Long, Long],
        vec![Long, Long, Long],
        vec![Short, Long, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Short, Short],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

    alphabet.to_morse_code();
    alphabet.to_uppercase().to_morse_code();
}
