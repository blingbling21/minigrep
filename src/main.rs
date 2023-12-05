use std::{env, process};

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("参数解析错误：{err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("应用错误： {e}");
        process::exit(1);
    }
}
