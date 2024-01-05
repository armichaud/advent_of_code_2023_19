mod solution_1;
use solution_1::*;

#[derive(Debug, Clone, Copy)]
struct RatingRange {
    min: usize,
    max: usize,
}

impl RatingRange {
    fn get_range_size(&self) -> usize {
        self.max - self.min + 1
    }
}


#[derive(Debug)]
struct RangePart {
    x: RatingRange,
    m: RatingRange,
    a: RatingRange,
    s: RatingRange,
}

fn solution_2(file: &str) -> usize {
    let (workflows, _) = parse_input(file);
    //println!("workflows: {:?}", workflows);
    let mut stack = Vec::from([(
        RangePart {
            x: RatingRange { min: 1, max: 4000 },
            m: RatingRange { min: 1, max: 4000 },
            a: RatingRange { min: 1, max: 4000 },
            s: RatingRange { min: 1, max: 4000 },
        },
        "in",
    )]);
    let mut accepted: Vec<RangePart> = Vec::new();
    'outer: while stack.len() > 0 {
        let (mut part, workflow_id) = stack.pop().unwrap();
        if workflow_id == ACCEPTED.to_string() {
            accepted.push(part);
            continue;
        }
        if workflow_id == REJECTED.to_string() {
            continue;
        }
        //println!("id: {}", workflow_id);
        let workflow = workflows.iter().find(|worflow| worflow.id == workflow_id).unwrap();
        for rule in &workflow.rules {
            match rule.category {
                'x' => {
                    let min = part.x.min;
                    let max = part.x.max;
                    if (rule.comparison == std::cmp::Ordering::Greater && min > rule.value) || (rule.comparison == std::cmp::Ordering::Less && max < rule.value) {
                        stack.push((part, rule.destination.as_ref()));
                        continue 'outer;
                    }
                    if rule.comparison == std::cmp::Ordering::Greater && max > rule.value {
                        stack.push((
                            RangePart {
                                x: RatingRange { min: rule.value + 1, max },
                                m: part.m,
                                a: part.a,
                                s: part.s,
                            }, 
                            rule.destination.as_ref()
                        ));
                        part.x.max = rule.value;
                    } else if rule.comparison == std::cmp::Ordering::Less && min < rule.value {
                        stack.push((
                            RangePart {
                                x: RatingRange { min, max: rule.value - 1 },
                                m: part.m,
                                a: part.a,
                                s: part.s,
                            }, 
                            rule.destination.as_ref()
                        ));
                        part.x.min = rule.value;
                    }
                }
                'm' => {
                    let min = part.m.min;
                    let max = part.m.max;
                    if (rule.comparison == std::cmp::Ordering::Greater && min > rule.value) || (rule.comparison == std::cmp::Ordering::Less && max < rule.value) {
                        stack.push((part, rule.destination.as_ref()));
                        continue 'outer;
                    }
                    if rule.comparison == std::cmp::Ordering::Greater && max > rule.value {
                        stack.push((
                            RangePart {
                                x: part.x,
                                m: RatingRange { min: rule.value + 1, max },
                                a: part.a,
                                s: part.s,
                            }, 
                            rule.destination.as_ref()
                        ));
                        part.m.max = rule.value;
                    } else if rule.comparison == std::cmp::Ordering::Less && min < rule.value {
                        stack.push((
                            RangePart {
                                x: part.x,
                                m: RatingRange { min, max: rule.value - 1 },
                                a: part.a,
                                s: part.s,
                            }, 
                            rule.destination.as_ref()
                        ));
                        part.m.min = rule.value;

                    }
                }
                'a' => {
                    let min = part.a.min;
                    let max = part.a.max;
                    if (rule.comparison == std::cmp::Ordering::Greater && min > rule.value) || (rule.comparison == std::cmp::Ordering::Less && max < rule.value) {
                        stack.push((part, rule.destination.as_ref()));
                        continue 'outer;
                    }
                    if rule.comparison == std::cmp::Ordering::Greater && max > rule.value {
                        stack.push((
                            RangePart {
                                x: part.x,
                                m: part.m,
                                a: RatingRange { min: rule.value + 1, max },
                                s: part.s,
                            }, 
                            rule.destination.as_ref()
                        ));
                        part.a.max = rule.value;
                    } else if rule.comparison == std::cmp::Ordering::Less && min < rule.value {
                        stack.push((
                            RangePart {
                                x: part.x,
                                m: part.m,
                                a: RatingRange { min, max: rule.value - 1 },
                                s: part.s,
                            }, 
                            rule.destination.as_ref()
                        ));
                        part.a.min = rule.value;
                    }
                }
                's' => {
                    let min = part.s.min;
                    let max = part.s.max;
                    if (rule.comparison == std::cmp::Ordering::Greater && min > rule.value) || (rule.comparison == std::cmp::Ordering::Less && max < rule.value) {
                        stack.push((part, rule.destination.as_ref()));
                        continue 'outer;
                    }
                    if rule.comparison == std::cmp::Ordering::Greater && max > rule.value {
                        stack.push((
                            RangePart {
                                x: part.x,
                                m: part.m,
                                a: part.a,
                                s: RatingRange { min: rule.value + 1, max },
                            }, 
                            rule.destination.as_ref()
                        ));
                        part.s.max = rule.value;
                    } else if rule.comparison == std::cmp::Ordering::Less && min < rule.value {
                        stack.push((
                            RangePart {
                                x: part.x,
                                m: part.m,
                                a: part.a,
                                s: RatingRange { min, max: rule.value - 1 },
                            }, 
                            rule.destination.as_ref()
                        ));
                        part.s.min = rule.value;
                    }
                }
                _ => panic!("Invalid category"),
            };
        }
        if workflow.default == ACCEPTED.to_string() {
            accepted.push(part);
            continue;
        }
        if workflow.default == REJECTED.to_string() {
            continue;
        }
        stack.push((part, workflow.default.as_ref()));
    }
    accepted.iter().fold(0, |a, part| a + (part.x.get_range_size() * part.m.get_range_size() * part.a.get_range_size() * part.s.get_range_size()))
}

   



fn main() {
    //assert_eq!(solution_1("example.txt"), 19114);
    //assert_eq!(solution_1("input.txt"), 432427);
    //assert_eq!(solution_2("example.txt"), 167409079868000);
    assert_eq!(solution_2("input.txt"), 144400479962759);
}
