use std::collections::HashMap;

type SessionResult = HashMap<String, usize>;
fn new_hashmap() -> SessionResult {
    let mut letter_hashmap: SessionResult = HashMap::new();
    letter_hashmap.insert("red".to_string(), 0);
    letter_hashmap.insert("blue".to_string(), 0);
    letter_hashmap.insert("green".to_string(), 0);
    letter_hashmap
}

fn main() {
    let file_name = std::env::args().nth(1).unwrap_or("day2".to_string());
    let file = std::fs::read_to_string(file_name).expect("No such file on directory");

    let lines = file.lines();
    let mut answer: usize = 0;
    for line in lines {
        let splited_line = line
            .split(':')
            .map(|line| line.trim())
            .collect::<Vec<&str>>();
        let game_id = splited_line[0]
            .split(' ')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();

        let sessions = splited_line[1].split(';').collect::<Vec<&str>>();
        let mut temp: usize = 0;
        for session in &sessions {
            let mut result = new_hashmap();
            let element = session.split(',').map(|l| l.trim()).collect::<Vec<&str>>();
            element.iter().for_each(|game| {
                let splited_game = game.split(' ').collect::<Vec<&str>>();
                let count = splited_game.first().unwrap().parse::<usize>().unwrap();
                let target = splited_game.last().unwrap().to_string();
                *result.entry(target).or_insert(0) += count;
            });
            let is_possible = result.iter().all(|(key, value)| match key.as_str() {
                "red" => *value <= 12,
                "green" => *value <= 13,
                "blue" => *value <= 14,
                _ => false,
            });
            if is_possible {
                temp += 1;
            }
            if temp == sessions.len() {
                answer += game_id;
            }
        }
    }
    println!("{}", answer)
}
