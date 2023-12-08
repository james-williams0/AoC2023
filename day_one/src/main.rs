use std::
{
    fs::File,
    io::{prelude::*, self},
    path::Path, collections::HashMap
};

fn main()
{
    if let Ok(lines) = read_lines("input.txt")
    {
        let spelled_out_numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let mut count: u32 = 0;
        for line in lines
        {
            if let Ok(input) = line
            {
                let first_spelled_number = spelled_out_numbers
                    .iter()
                    .map(|&spelled_out_number| input.find(spelled_out_number).map(|index| (index, spelled_out_number)))
                    .filter_map(|option| option.as_ref().map(|&(index, first_spelled_number)| (index, first_spelled_number)))
                    .min_by(|spelled_out_number, another_spelled_out_number| spelled_out_number.0.cmp(&another_spelled_out_number.0))
                    .map(|option| (option.0, parse_spelled_number_to_int(option.1)));

                let last_spelled_number = spelled_out_numbers
                    .iter()
                    .map(|&word| input.rfind(word).map(|start| (start, word)))
                    .filter_map(|option| option.as_ref().map(|&(usize_val, str_ref)| (usize_val, str_ref)))
                    .min_by(|word, another_word| another_word.0.cmp(&word.0))
                    .map(|option| (option.0, parse_spelled_number_to_int(option.1)));

                let first_digit_index = input.chars().position(|c| c.is_numeric()).unwrap();
                let last_digit_reverse_index = input.chars().rev().position(|c| c.is_numeric()).unwrap();
                let last_numeric_index = input.len() - last_digit_reverse_index - 1;
                
                let first_numeric_char = input.chars().nth(first_digit_index).unwrap();
                let last_numeric_char = input.chars().rev().nth(last_digit_reverse_index).unwrap();

                let concat: String;

                match (first_spelled_number, last_spelled_number)
                {
                    (None, None) =>
                    {
                        concat = format!("{}{}", first_numeric_char, last_numeric_char);
                    },
                    (Some(first_spelled_number), None) =>
                    {
                        if first_spelled_number.0 < first_digit_index
                        {
                            concat = format!("{}{}", first_spelled_number.1, last_numeric_char);
                        }
                        else
                        {
                            concat = format!("{}{}", first_numeric_char, last_numeric_char);
                        }
                    },
                    (None, Some(last_spelled_number)) =>
                    {
                        if last_spelled_number.0 > last_numeric_index
                        {
                            concat = format!("{}{}", first_numeric_char, last_spelled_number.1);
                        }
                        else
                        {
                            concat = format!("{}{}", first_numeric_char, last_numeric_char);
                        }
                    },
                    (Some(first_spelled_number), Some(last_spelled_number)) =>
                    {
                        if first_spelled_number.0 < first_digit_index && last_spelled_number.0 > last_numeric_index
                        {
                            concat = format!("{}{}", first_spelled_number.1, last_spelled_number.1);
                        }
                        else if first_spelled_number.0 > first_digit_index && last_spelled_number.0 > last_numeric_index
                        {
                            concat = format!("{}{}", first_numeric_char, last_spelled_number.1);
                        }
                        else if first_spelled_number.0 < first_digit_index && last_spelled_number.0 < last_numeric_index
                        {
                            concat = format!("{}{}", first_spelled_number.1, last_numeric_char);
                        }
                        else
                        {
                            concat = format!("{}{}", first_numeric_char, last_numeric_char);
                        }
                    }
                }
                let calibration_value = concat.parse::<u32>().unwrap();
                count = count + calibration_value;
            }
        }
        println!("{}", count)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_spelled_number_to_int(spelled: &str) -> u32
{
    let spelled_number_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    spelled_number_map[spelled]
}