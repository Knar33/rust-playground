use std::rc::Rc;
use std::cell::RefCell;

fn main() { 
    println!("{:?}", roman_to_int("MDCCLXXVI".to_string()));
}

pub fn roman_to_int(s: String) -> i32 {
    let mut ret = 0;
    let mut prev = 0;
    for (i, current) in s.chars().enumerate() {
    let current_int = match current {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0
    };
    if i > 0 {
        if (current_int > prev) {
        ret -= prev
        } else {
        ret += prev
        }
    }
    prev = current_int;
    }
    ret += prev;
    ret
}