mod day_1;

use std::{env, path::Path};

fn main() {
    let home =
        env::var("HOME").expect("Cannot find home directory. Make sure the env var HOME is set.");
    let input_dir_str = format!("{home}/workspace/advent-of-code-2022/input");
    let input_dir_path = Path::new(&input_dir_str);

    let date = parse_arg();
    let input_file_name_str = format!("day_{date}.txt");
    let input_file_name = Path::new(&input_file_name_str);

    let input_file_path = input_dir_path.join(input_file_name);
    let result = day_1::solve(input_file_path.as_path());
    println!("{}, {}", result.0, result.1);
}

fn parse_arg() -> u8 {
    let err_msg = "Please input a date between 1 and 31";

    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 1, "{}", err_msg);

    let date_str = &args[1];
    let date = str::parse::<u8>(date_str).expect(err_msg);
    assert!(date >= 1 && date <= 31, "{}", err_msg);

    date
}
