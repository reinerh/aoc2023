use std::collections::HashMap;

static DAY: u8 = 19;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", rating_numbers(&input));
    println!("{DAY}b: {}", 0);
}

#[derive(Clone)]
enum WorkflowResult {
    Accept,
    Reject,
    NextWorkflow(String),
}

impl WorkflowResult {
    fn from(input: &str) -> WorkflowResult {
        match input {
            "A" => WorkflowResult::Accept,
            "R" => WorkflowResult::Reject,
            next => WorkflowResult::NextWorkflow(next.to_string()),
        }
    }
}

enum Rule {
    CmpGt(char, u32, WorkflowResult),
    CmpLt(char, u32, WorkflowResult),
    Result(WorkflowResult),
}

impl Rule {
    fn new(input: &str) -> Rule {
        if input.contains('<') {
            let tokens = input.split(|x| x == '<' || x == ':').collect::<Vec<_>>();
            let c = tokens[0].chars().next().unwrap();
            let val = tokens[1].parse().unwrap();
            let result = WorkflowResult::from(tokens[2]);
            Rule::CmpLt(c, val, result)
        } else if input.contains('>') {
            let tokens = input.split(|x| x == '>' || x == ':').collect::<Vec<_>>();
            let c = tokens[0].chars().next().unwrap();
            let val = tokens[1].parse().unwrap();
            let result = WorkflowResult::from(tokens[2]);
            Rule::CmpGt(c, val, result)
        } else {
            Rule::Result(WorkflowResult::from(input))
        }
    }

    fn matches(&self, xmas: &XmasValues) -> Option<WorkflowResult> {
        match self {
            Rule::CmpGt(variable, value, result) => match variable {
                'x' => if xmas.x > *value { Some(result.clone()) } else { None },
                'm' => if xmas.m > *value { Some(result.clone()) } else { None },
                'a' => if xmas.a > *value { Some(result.clone()) } else { None },
                's' => if xmas.s > *value { Some(result.clone()) } else { None },
                _ => panic!("invalid char"),
            },
            Rule::CmpLt(variable, value, result) => match variable {
                'x' => if xmas.x < *value { Some(result.clone()) } else { None },
                'm' => if xmas.m < *value { Some(result.clone()) } else { None },
                'a' => if xmas.a < *value { Some(result.clone()) } else { None },
                's' => if xmas.s < *value { Some(result.clone()) } else { None },
                _ => panic!("invalid char"),
            }
            Rule::Result(result) => Some(result.clone()),
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(input: &str) -> Workflow {
        let (name, rules_str) = input[0..input.len() - 1].split_once('{').unwrap();
        let rules = rules_str.split(',')
                             .map(Rule::new)
                             .collect::<Vec<_>>();
        let name = name.to_string();

        Workflow { name, rules }
    }

    fn matches(&self, xmas: &XmasValues) -> WorkflowResult {
        for rule in &self.rules {
            if let Some(result) = rule.matches(xmas) {
                return result.clone();
            }
        }
        panic!("no matching rule found");
    }
}

struct XmasValues {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl XmasValues {
    fn new(input: &str) -> XmasValues {
        let mut xmas = XmasValues { x: 0, m: 0, a: 0, s: 0 };
        for string in input[1 .. input.len() - 1 ].split(',') {
            let (variable, value) = string.split_once('=').unwrap();
            let value = value.parse::<u32>().unwrap();
            match variable {
                "x" => xmas.x = value,
                "m" => xmas.m = value,
                "a" => xmas.a = value,
                "s" => xmas.s = value,
                _ => panic!("invalid variable"),
            }
        }
        xmas
    }

    fn rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

struct Workflows {
    workflows: HashMap<String, Workflow>,
}

impl Workflows {
    fn new(input: &[String]) -> Workflows {
        let workflows = input.iter()
                             .map(|workflow| Workflow::new(workflow))
                             .map(|workflow| (workflow.name.clone(), workflow))
                             .collect();
        Workflows { workflows }
    }

    fn is_accepted(&self, xmas: &XmasValues) -> bool {
        let mut workflow = &self.workflows["in"];
        loop {
            match workflow.matches(xmas) {
                WorkflowResult::Accept => return true,
                WorkflowResult::Reject => return false,
                WorkflowResult::NextWorkflow(next) => workflow = &self.workflows[&next],
            }
        }
    }
}

fn rating_numbers(input: &[String]) -> u32 {
    let blocks = input.split(|line| line.is_empty()).collect::<Vec<_>>();
    let (workflows, values) = (Workflows::new(blocks[0]), blocks[1]);
    let values = values.iter()
                       .map(|value| XmasValues::new(value))
                       .collect::<Vec<_>>();

    values.iter()
          .filter(|&value| workflows.is_accepted(value))
          .map(|value| value.rating())
          .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "px{a<2006:qkq,m>2090:A,rfg}",
            "pv{a>1716:R,A}",
            "lnx{m>1548:A,A}",
            "rfg{s<537:gd,x>2440:R,A}",
            "qs{s>3448:A,lnx}",
            "qkq{x<1416:A,crn}",
            "crn{x>2662:A,R}",
            "in{s<1351:px,qqz}",
            "qqz{s>2770:qs,m<1801:hdj,R}",
            "gd{a>3333:R,R}",
            "hdj{m>838:A,pv}",
            "",
            "{x=787,m=2655,a=1222,s=2876}",
            "{x=1679,m=44,a=2067,s=496}",
            "{x=2036,m=264,a=79,s=2244}",
            "{x=2461,m=1339,a=466,s=291}",
            "{x=2127,m=1623,a=2188,s=1013}",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();
        assert_eq!(rating_numbers(&input), 19114);
    }
}
