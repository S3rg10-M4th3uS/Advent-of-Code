use std::{collections::HashMap, num::ParseIntError};

#[derive(Debug)]
struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    id: HashMap<u16, Bag>,
}
impl Game {
    fn new(bags: Bag, id: u16) -> Game {
        let mut hasher = HashMap::new();
        hasher.insert(id, bags);
        Game { id: hasher }
    }
}
fn main() {
    let line = include_str!("sample.txt");
    let mut all_games = Vec::new();
    for lines in line.lines() {
        let bag = find_counts_by_color::<ParseIntError>(lines).unwrap();
        all_games.push(bag);
    }

    let mut keys: u16 = 1;
    let mut result = Vec::new();
    for value in all_games.iter() {
        result.push(
            value
                .id
                .get(&keys)
                .filter(|bags| bags.red <= 12 && bags.green <= 13 && bags.blue <= 14),
        );
        keys += 1;
    }
    let mut final_result = 0;
    let mut counter = 1;
    for value in result.iter() {
        match value {
            Some(_) => final_result += counter,
            None => (),
        }
        counter += 1;
    }
    println!("{:?}", final_result);
}

fn find_counts_by_color<E>(input: &str) -> Result<Game, E>
where
    E: std::convert::From<std::num::ParseIntError>,
{
    let colors = ["red", "green", "blue"];
    let mut counts_by_color: Vec<(String, Vec<usize>)> = colors
        .iter()
        .map(|color| (color.to_string(), Vec::new()))
        .collect();

    for game in input.split(';') {
        for (color, values) in counts_by_color.iter_mut() {
            values.extend(game.split(',').filter_map(|item| {
                let parts: Vec<&str> = item.split_whitespace().collect();
                if let Some(index) = parts.iter().position(|&x| x == *color) {
                    parts
                        .get(index.checked_sub(1)?)
                        .and_then(|s| s.parse::<usize>().ok())
                } else {
                    None
                }
            }));
        }
    }
    let mut bag = Bag {
        red: 0,
        green: 0,
        blue: 0,
    };
    for (color, values) in counts_by_color {
        if let Some(max_value) = values.iter().max() {
            match color.as_str() {
                "red" => bag.red = *max_value,
                "green" => bag.green = *max_value,
                "blue" => bag.blue = *max_value,
                _ => {}
            }
        }
    }
    let id = input
        .chars()
        .skip_while(|x| !x.is_ascii_digit())
        .take_while(|&x| x.is_ascii_digit())
        .collect::<String>()
        .parse::<u16>()?;
    Ok(Game::new(bag, id))
}
