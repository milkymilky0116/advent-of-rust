use std::collections::HashMap;
fn new_hashmap() -> HashMap<String, String> {
    let mut letter_hashmap: HashMap<String, String> = HashMap::new();
    letter_hashmap.insert("one".to_string(), "1".to_string());

    letter_hashmap.insert("two".to_string(), "2".to_string());

    letter_hashmap.insert("three".to_string(), "3".to_string());

    letter_hashmap.insert("four".to_string(), "4".to_string());

    letter_hashmap.insert("five".to_string(), "5".to_string());

    letter_hashmap.insert("six".to_string(), "6".to_string());

    letter_hashmap.insert("seven".to_string(), "7".to_string());

    letter_hashmap.insert("eight".to_string(), "8".to_string());

    letter_hashmap.insert("nine".to_string(), "9".to_string());

    letter_hashmap
}
fn main() {
    let letter_hashmap = new_hashmap();
    let file_name = std::env::args().nth(1).unwrap_or("day1".to_string());
    let file = std::fs::read_to_string(file_name).expect("No such file on directory");
    let result: usize = file
        .lines()
        .map(|line| {
            println!("{}", line);
            let mut stack: Vec<char> = vec![];
            for letter in line.chars() {
                stack.push(letter);
                let mut stack_str: String = stack.iter().collect();
                for key in letter_hashmap.keys() {
                    stack_str = stack_str.replace(key, letter_hashmap.get(key).unwrap())
                }
                stack = stack_str.chars().collect();
            }
            let result = stack.iter().collect::<String>();
            result
        })
        .map(|line| {
            line.chars()
                .filter(|letter| letter.is_numeric())
                .collect::<Vec<char>>()
        })
        .map(|letters| {
            let mut result: usize = 0;
            match letters.len() {
                0 => {}
                _ => {
                    let first = letters.first().unwrap();
                    let last = letters.last().unwrap();
                    let num = format!("{}{}", first, last).parse::<usize>().unwrap();
                    result = num
                }
            }
            result
        })
        .sum();
    println!("{:?}", result)
}
