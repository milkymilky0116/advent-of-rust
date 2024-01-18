fn main() {
    let file_name = std::env::args().nth(1).unwrap_or("day1".to_string());
    let file = std::fs::read_to_string(file_name).expect("No such file on directory");
    let lines = file
        .lines()
        .map(|value| {
            value
                .chars()
                .filter(|letter| letter.is_ascii_digit())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let mut result: usize = 0;
    for letters in lines {
        match letters.len() {
            0 => {}
            _ => {
                let first_letter = letters.first().unwrap();
                let last_letter = letters.last().unwrap();
                let str_num = format!("{}{}", first_letter, last_letter);
                println!("{}", str_num);
                let num = str_num.parse::<usize>().unwrap();
                result += num;
            }
        }
    }
    println!("{}", result)
}
