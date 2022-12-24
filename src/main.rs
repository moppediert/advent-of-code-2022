mod day_1;
use day_1::solve;

use std::{env, path::Path};

fn main() {
    let home =
        env::var("HOME").expect("Cannot find home directory. Make sure the env var HOME is set.");
    let input_dir_str = format!("{home}/workspace/advent-of-code-2022/input");
    let input_dir_path = Path::new(&input_dir_str);
    let date = parse_arg();
    let result = solve(
        input_dir_path
            .join(Path::new(&format!("day_{date}.txt")))
            .as_path(),
    );
    println!("{}", result);
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
