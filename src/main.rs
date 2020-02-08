use std::io;

struct LargeNumber {
    chars: Vec<usize>,
}

impl LargeNumber {

    pub fn new(palindrome_number: String) -> LargeNumber {
        let characters: Vec<usize> = palindrome_number
            .split("")
            .filter(|char| char != &"")
            .map(|char| char.parse().expect("Failed to split string"))
            .collect();
            
        let first_non_zero = characters
            .iter()
            .position(|char| char != &0)
            .unwrap();
            
        let final_index = characters.len();
        
        let characters = characters[first_non_zero..final_index].to_vec();
            
        return LargeNumber { chars: characters };
    }
    
    fn next_palindrome(&mut self) -> LargeNumber {
        
        // returnable
        let mut next = LargeNumber{ chars: self.chars.to_vec() };
        
        // params
        if next.get_length() == 1 {
            next.inc_at(0);
            next.mirror();
            return next;
        }
        
        // indexes where right side digit is lesser and greater than left side digit
        let break_index = self.find_break_index();
        let safe_index = self.find_safe_index();
        let is_palindrome = break_index == next.get_length() && safe_index == next.get_length();
        
        if break_index < safe_index || is_palindrome {
            let half_length = next.get_length() / 2;
            if next.get_length() % 2 == 0 {
                next.inc_at(half_length - 1);
            } else {
                next.inc_at(half_length);
            }
        }
        
        next.mirror();
        return next;
    }

    fn is_palindrome(&mut self) -> bool {
        let length = self.get_length();
        let break_index = self.find_break_index();
        let safe_index = self.find_safe_index();
        
        return break_index == length && safe_index == length;
    }
    
    fn find_break_index(&mut self) -> usize {
        let length = self.get_length();
        let mut index = length / 2;
        while index < length {
            let left_digit = self.get_digit(length - 1 - index);
            let right_digit = self.get_digit(index);
            if left_digit < right_digit {
                break;
            }
            index = index + 1;
        }
        return index;
    }
    
    fn find_safe_index(&mut self) -> usize {
        let length = self.get_length();
        let mut index = length / 2;
        while index < length {
            let left_digit = self.get_digit(length - 1 - index);
            let right_digit = self.get_digit(index);
            if left_digit > right_digit {
                break;
            }
            index = index + 1;
        }
        return index;
    }
    
    fn mirror(&mut self) {
        let length = self.get_length();
        let half_length = length / 2;
        for i in half_length..length {
            let new_digit = self.get_digit(length - i - 1);
            self.update_digit(i, new_digit);
        }
    }
    
    fn printable(&mut self) -> String {
        let joined: String = self.chars
            .iter()
            .map(|x| x.to_string())
            .collect();
            
        return joined;
    }

    fn inc_at(&mut self, index: usize) {
        let digit: usize = self.get_digit(index);
        
        if digit == 9 {
        // needs to increase next most significat digit
            if index == 0 {
            // adds one more digit to the left
                self.update_digit(index, 0);
                self.add_one_left_most();
            } else {
            // increases next most significant digit
                self.update_digit(index, 0);
                self.inc_at(index - 1);
            }
        } else {
        // just increases digit at specified index
            self.update_digit(index, digit + 1);
        }
    }
    
    fn add_one_left_most(&mut self) {
        self.chars.insert(0, 1);
    }
    
    fn get_digit(&mut self, index: usize) -> usize {
        return self.chars[index];
    }

    fn get_length(&mut self) -> usize {
        return self.chars.len();
    }
    
    fn update_digit(&mut self, index: usize, value: usize) {
        self.chars[index] = value;
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
fn palindrome_update_digit() {
    let mut foo = LargeNumber::new("111".to_string());
    foo.update_digit(1, 3);
    foo.update_digit(2, 5);
    assert!(foo.get_digit(1) == 3);
    assert!(foo.get_digit(2) == 5);
}

#[test]
fn palindrome_safe_index() {
    assert!(LargeNumber::new("1".to_string()).find_safe_index() == 1);
    assert!(LargeNumber::new("10".to_string()).find_safe_index() == 1);
    assert!(LargeNumber::new("11".to_string()).find_safe_index() == 2);
    assert!(LargeNumber::new("12".to_string()).find_safe_index() == 2);
    assert!(LargeNumber::new("1111".to_string()).find_safe_index() == 4);
    assert!(LargeNumber::new("1110".to_string()).find_safe_index() == 3);
    assert!(LargeNumber::new("11101".to_string()).find_safe_index() == 3);
    assert!(LargeNumber::new("11120".to_string()).find_safe_index() == 4);
    assert!(LargeNumber::new("11010".to_string()).find_safe_index() == 4);
}

#[test]
fn palindrome_break_index() {
    assert!(LargeNumber::new("1".to_string()).find_break_index() == 1);
    assert!(LargeNumber::new("10".to_string()).find_break_index() == 2);
    assert!(LargeNumber::new("11".to_string()).find_break_index() == 2);
    assert!(LargeNumber::new("12".to_string()).find_break_index() == 1);
    assert!(LargeNumber::new("1111".to_string()).find_break_index() == 4);
    assert!(LargeNumber::new("1112".to_string()).find_break_index() == 3);
    assert!(LargeNumber::new("11121".to_string()).find_break_index() == 3);
    assert!(LargeNumber::new("11120".to_string()).find_break_index() == 3);
    assert!(LargeNumber::new("11012".to_string()).find_break_index() == 4);
}

#[test]
fn palindrome_is_palindrome() {
    assert!(LargeNumber::new("1".to_string()).is_palindrome());
    assert!(LargeNumber::new("11".to_string()).is_palindrome());
    assert!(LargeNumber::new("111".to_string()).is_palindrome());
    assert!(LargeNumber::new("131".to_string()).is_palindrome());
    assert!(LargeNumber::new("1221".to_string()).is_palindrome());
    assert!(LargeNumber::new("12521".to_string()).is_palindrome());
    assert!(LargeNumber::new("11111111111111111111111111111111111".to_string()).is_palindrome());
}

#[test]
fn palindrome_mirror_after_2_digit() {
    let mut foo = LargeNumber::new("10".to_string());
    foo.mirror();
    assert_eq!(foo.printable(), "11");
}

#[test]
fn palindrome_mirror_after_3_digit() {
    let mut foo = LargeNumber::new("100".to_string());
    foo.mirror();
    assert_eq!(foo.printable(), "101");
}

#[test]
fn palindrome_next() {
    assert_eq!(LargeNumber::new("1".to_string()).next_palindrome().printable(), "2");
    assert_eq!(LargeNumber::new("5".to_string()).next_palindrome().printable(), "6");
    assert_eq!(LargeNumber::new("9".to_string()).next_palindrome().printable(), "11");
    assert_eq!(LargeNumber::new("11".to_string()).next_palindrome().printable(), "22");
    assert_eq!(LargeNumber::new("99".to_string()).next_palindrome().printable(), "101");
    assert_eq!(LargeNumber::new("111".to_string()).next_palindrome().printable(), "121");
    assert_eq!(LargeNumber::new("324".to_string()).next_palindrome().printable(), "333");
    assert_eq!(LargeNumber::new("186542".to_string()).next_palindrome().printable(), "186681");
    assert_eq!(LargeNumber::new("972692378".to_string()).next_palindrome().printable(), "972696279");
    assert_eq!(LargeNumber::new("929993774".to_string()).next_palindrome().printable(), "929999929");
    assert_eq!(LargeNumber::new("12111221".to_string()).next_palindrome().printable(), "12122121");
    assert_eq!(LargeNumber::new("0486445".to_string()).next_palindrome().printable(), "486684");
    assert_eq!(LargeNumber::new("00486445".to_string()).next_palindrome().printable(), "486684");
    assert_eq!(LargeNumber::new("87248992".to_string()).next_palindrome().printable(), "87255278");
    assert_eq!(LargeNumber::new("9999999999".to_string()).next_palindrome().printable(), "10000000001");
}

#[test]
fn foo() {}

fn main() {
    let total_test: u128 = get_input().parse()
        .expect("Failed to convert integer");
        
    for _i in 0..total_test {
        let input: String = get_input().parse().expect("Failed to parse string");
        println!("{}", LargeNumber::new(input).next_palindrome().printable());
    }
}
