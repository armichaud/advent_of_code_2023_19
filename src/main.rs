use std::{fs::File, io::{BufReader, BufRead}};
use std::cmp::Ordering;

const ACCEPTED: char = 'A';
const REJECTED: char = 'R';

struct Rule {
    category: char,
    comparison: Ordering,
    value: usize,
}

struct Workflow {
    id: String,
    steps: Vec<Rule>,
    default: String,
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn get_workflow(line: String) -> Workflow {}

fn get_part(line: String) -> Part {}

fn parse_input(filename: &str) -> (Vec<Workflow>, Vec<Part>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut line = lines.next().unwrap().unwrap();
    let mut workflows = Vec::new();
    while line != "\n\n" {
        workflows.push(get_workflow(line));
        line = lines.next().unwrap().unwrap();
    }
    let mut parts = Vec::new();
    line = lines.next().unwrap().unwrap();
    while (line != "\n\n") {
        parts.push(get_part(line));
        line = lines.next().unwrap().unwrap();   
    }
    (workflows, parts)
}

fn solution(filename: &str) -> usize {
    0
}

fn main() {
    assert_eq!(solution("example.txt"), 19114);
    assert_eq!(solution("input.txt"), 0);
}
