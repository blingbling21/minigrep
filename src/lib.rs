use std::{error::Error, fs, env};

use regex::Regex;

// 配置相关结构体
pub struct Config {
    query_string: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    // 参数初始化
    pub fn build(args: &[String]) -> Result<Config, &str> {
        let q_exist = args.iter().any(|arg| Regex::new(r"^-q=").unwrap().is_match(arg));
        if !q_exist {
            return Err("缺少-q=")
        }
        let f_exist = args.iter().any(|arg| Regex::new(r"^-f=").unwrap().is_match(arg));
        if !f_exist {
            return Err("缺少-f=")
        }
        let query_vec = &(args[1][..]).split("=").collect::<Vec<&str>>();
        let query_string = query_vec[1].to_string();
        let file_path_vec = &(args[2][..]).split("=").collect::<Vec<&str>>();
        let file_path = file_path_vec[1].to_string();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        println!("ignore_case: {ignore_case}");

        Ok(Self { query_string, file_path, ignore_case })
    }
}

// 主要逻辑
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query_string, &content)
    } else {
        search(&config.query_string, &content)
    };

    for line in results {
        println!("line: {line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}

// 字符串搜索
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

// 字符串搜索 -- 大小写不敏感
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}
