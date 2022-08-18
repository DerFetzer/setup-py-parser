use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use regex::Regex;
use serde::Serialize;

const COMPILER: &str = "cl.exe";

#[derive(Debug, Serialize)]
struct CommandObject {
    directory: String,
    arguments: Vec<String>,
    file: String,
}
fn main() {
    let buf_reader = BufReader::new(File::open("bdist_wheel_output.txt").unwrap());
    let include_regex = Regex::new(r"-I[^:\s]{2,}").unwrap();
    let file_regex = Regex::new(r"/T[pc]([^:\s]+)").unwrap();
    let cwd = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let cos: Vec<_> = buf_reader
        .lines()
        .into_iter()
        .flatten()
        .filter(|l| l.contains(COMPILER))
        .map(|l| {
            let line = &l.replace('\\', "/");
            println!("{line}");
            let includes: Vec<_> = include_regex
                .find_iter(line)
                .map(|m| m.as_str().to_string())
                .collect();
            let file = file_regex
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();

            let mut arguments = Vec::with_capacity(includes.len() + 1);
            arguments.push(COMPILER.to_string());
            arguments.extend(includes.into_iter());
            CommandObject {
                directory: cwd.clone(),
                arguments,
                file,
            }
        })
        .collect();

    let mut output = File::create("compile_commands.json").unwrap();
    output
        .write_all(&serde_json::to_vec(&cos).unwrap())
        .unwrap();
}
