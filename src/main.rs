mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

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
    let solver = match date {
        1 => |x| {println!("{:#?}", day_1::solve(x))},
        2 => |x| {println!("{:#?}", day_2::solve(x))},
        3 => |x| {println!("{:#?}", day_3::solve(x))},
        4 => |x| {println!("{:#?}", day_4::solve(x))},
        5 => |x| {println!("{:#?}", day_5::solve(x))},
        6 => |x| {println!("{:#?}", day_6::solve(x))},
        7 => |x| {println!("{:#?}", day_7::solve(x))},
        
        _ => |_| {println!("Boringgggg")}
    };

    solver(input_file_path.as_path());
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
