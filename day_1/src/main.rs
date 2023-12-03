fn extract_calibration_values(document: &str) -> i32 {
    let mut sum = 0;

    for line in document.lines() {
        let first_digit = line.chars().find(|c| c.is_ascii_digit());
        let last_digit = line.chars().rfind(|c| c.is_ascii_digit());

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            let value = format!("{}{}", first, last).parse::<i32>().unwrap_or(0);
            sum += value;
        }
    }

    sum
}

fn main() {
    let calibration_document = include_str!("sample.txt");

    let result = extract_calibration_values(calibration_document);
    println!("Sum of calibration values: {}", result);
}
