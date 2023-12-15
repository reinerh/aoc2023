use std::collections::VecDeque;

static DAY: u8 = 15;

fn main() {
    let input = advent::read_file(DAY);
    println!("{DAY}a: {}", hash_sum(&input));
    println!("{DAY}b: {}", focusing_power(&input));
}

fn hash(input: &str) -> usize {
    let mut value = 0;

    for c in input.chars() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }

    value
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

enum Operation {
    Assign(Lens),
    Remove(String),
}

impl Operation {
    fn from(input: &str) -> Operation {
        if let Some((label, value)) = input.split_once('=') {
            Operation::Assign(Lens {
                label: label.to_string(),
                focal_length: value.parse().unwrap()
            })
        } else if let Some((label, _)) = input.split_once('-') {
            Operation::Remove(label.to_string())
        } else {
            panic!("invalid operation");
        }
    }

    fn label(&self) -> String {
        match self {
            Operation::Assign(lens) => lens.label.clone(),
            Operation::Remove(label) => label.to_string(),
        }
    }
}

fn hash_sum(input: &str) -> usize {
    let input = input.trim_end();
    input.split(',')
         .map(hash)
         .sum()
}

fn init_sequence(boxes: &mut [VecDeque::<Lens>], operations: &[Operation]) {
    for operation in operations {
        let box_nr = hash(&operation.label());
        match operation {
            Operation::Assign(lens) => {
                if let Some(boxed_lens) = boxes[box_nr].iter_mut().find(|l| l.label == lens.label) {
                    boxed_lens.focal_length = lens.focal_length;
                } else {
                    boxes[box_nr].push_back(lens.clone());
                }
            },
            Operation::Remove(label) => {
                boxes[box_nr].retain(|lens| lens.label != *label);
            }
        }
    }
}

fn focusing_power(input: &str) -> usize {
    let input = input.trim_end();
    let operations = input.split(',')
                          .map(Operation::from)
                          .collect::<Vec<_>>();

    let mut boxes = vec![VecDeque::<Lens>::new(); 256];
    init_sequence(&mut boxes, &operations);

    boxes.iter()
         .enumerate()
         .map(|(box_nr, lenses)| lenses.iter().enumerate().map(|(slot, lens)| (box_nr + 1) * (slot + 1) * lens.focal_length).sum::<usize>())
         .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash_sum(input), 1320);
        assert_eq!(focusing_power(input), 145);
    }
}
