use std::io;

struct LargeNumber {
    chars: Vec<String>,
}

impl LargeNumber {
    // pub fn is_palindrome(&mut self) -> bool {
    //     let length = self.get_length();
    //     let half_length = length / 2;
    //     for i in 1..half_length {
    //         let left_digit = self.get_digit(i);
    //         let right_digit = self.get_digit(length - i);
    //         if left_digit != right_digit { return false; }
    //     }
    //     return true;
    // }

    pub fn new(palindrome_number: String) -> LargeNumber {
        let characters: Vec<String> = palindrome_number
            .split("")
            .filter(|char| char != &"")
            .map(|char| char.parse().expect("Failed to split string"))
            .collect();

        return LargeNumber { chars: characters };
    }

    fn get_digit(&mut self, index: usize) -> usize {
        let digit: usize = self
            .chars
            .get(index)
            .unwrap()
            .parse()
            .expect("Failed to parse string to integer");
        return digit;
    }

    fn get_length(&mut self) -> usize {
        return self.chars.len();
    }

    fn update_digit(&mut self, index: usize, value: usize) {
        let character: String = value.to_string();
        self.chars[index] = character;
    }
}

fn get_input() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed at read_line");
    return input.trim().parse().expect("Failed to parse");
}

#[test]
fn palindrome_length() {
    let mut foo1 = LargeNumber::new("1".to_string());
    let mut foo2 = LargeNumber::new("12".to_string());
    assert!(foo1.get_length() == 1);
    assert!(foo2.get_length() == 2);
}

#[test]
fn palindrome_digit() {
    let mut foo = LargeNumber::new("1234".to_string());
    assert!(foo.get_digit(0) == 1);
    assert!(foo.get_digit(1) == 2);
    assert!(foo.get_digit(2) == 3);
    assert!(foo.get_digit(3) == 4);
}

#[test]
fn Palindrome_update_digit() {
    let mut foo = LargeNumber::new("111".to_string());
    foo.update_digit(1, 3);
    foo.update_digit(2, 5);
    assert!(foo.get_digit(1) == 3);
    assert!(foo.get_digit(2) == 5);
}

fn main() {
    // let total_test: u128 = get_input().parse()
    //     .expect("Failed to convert integer");

    println!("Hello, world!");
}
