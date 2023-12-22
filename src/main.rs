use std::{fs::File, io::{BufReader, BufRead}};
use std::cmp::Ordering;

const ACCEPTED: char = 'A';
const REJECTED: char = 'R';

#[derive(Debug)]
struct Rule {
    category: char,
    comparison: Ordering,
    value: usize,
    destination: String,
}

#[derive(Debug)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
    default: String,
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn get_workflow(line: String) -> Workflow {
    let mut parts = line.split("{");
    
    let id = parts.next().unwrap().to_string();
    let mut rule_list = parts.next().unwrap().to_string();
    rule_list.pop().unwrap(); // remove trailing '}'
    let mut rule_strings = rule_list.split(",").collect::<Vec<&str>>();
    let default = rule_strings.pop().unwrap().to_string();
    let mut rules = Vec::new();
    for rule in rule_strings.iter() {
        let mut split = rule.split(":");
        let mut chars = split.next().unwrap().chars();
        let destination = split.next().unwrap().to_string();
        let category = chars.next().unwrap();
        let comparison = match chars.next().unwrap() {
            '<' => Ordering::Less,
            '>' => Ordering::Greater,
            '=' => Ordering::Equal,
            _ => panic!("Invalid comparison operator"),
        };
        let value = chars.take_while(|x| x.is_numeric()).collect::<String>().parse::<usize>().unwrap();
        rules.push(Rule { category, comparison, value, destination });
    }
    Workflow {
        id,
        rules,
        default,
    }

}

fn get_part(line: String) -> Part {
    let mut line = line.replace("{", "");
    line = line.replace("}", "");
    let mut x = 0;
    let mut m = 0;
    let mut a = 0;
    let mut s = 0;
    for rating in line.split(",").into_iter() {
        let mut chars = rating.chars();
        let category = chars.next().unwrap();
        chars.next().unwrap();
        let value = chars.as_str().parse::<usize>().unwrap();
        match category {
            'x' => x = value,
            'm' => m = value,
            'a' => a = value,
            's' => s = value,
            _ => panic!("Invalid category"),
        }
    }
    Part {
        x,
        m,
        a,
        s,
    }
}

fn parse_input(filename: &str) -> (Vec<Workflow>, Vec<Part>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut line = lines.next().unwrap().unwrap();
    let mut workflows = Vec::new();
    while line != "" {
        workflows.push(get_workflow(line));
        line = lines.next().unwrap().unwrap();
    }
    let mut parts = Vec::new();
    lines.next().unwrap().unwrap();
    while let Some(Ok(line)) = lines.next() {
        parts.push(get_part(line));
    }
    (workflows, parts)
}

fn solution(filename: &str) -> usize {
    let (workflows, parts) = parse_input(filename);
    let mut sum = 0;
    for part in parts {
        let mut workflow = workflows.iter().find(|x| x.id == "in").unwrap();
        'outer: loop {
            for rule in &workflow.rules {
                let value = match rule.category {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => panic!("Invalid category"),
                };
                if value.cmp(&rule.value) == rule.comparison {
                    if rule.destination == ACCEPTED.to_string() {
                        sum += part.x + part.m + part.a + part.s;
                        break 'outer;
                    }
                    if rule.destination == REJECTED.to_string() {
                        break 'outer;
                    }
                    workflow = workflows.iter().find(|x| x.id == rule.destination).unwrap();
                    break;
                }
            }
            if workflow.default == ACCEPTED.to_string() {
                sum += part.x + part.m + part.a + part.s;
                break;
            }
            if workflow.default == REJECTED.to_string() {
                break;
            }
            workflow = workflows.iter().find(|x| x.id == workflow.default).unwrap();
        }

    }
    sum
}

fn main() {
    assert_eq!(solution("example.txt"), 19114);
    assert_eq!(solution("input.txt"), 0);
}
