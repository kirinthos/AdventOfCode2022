use std::cmp::Ordering;

use crate::Problem;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Element {
    Number(i32),
    List(Vec<Element>),
}

impl Element {
    fn lift(self) -> Self {
        Element::List(vec![self])
    }
}

impl From<i32> for Element {
    fn from(v: i32) -> Self {
        Element::Number(v)
    }
}

impl From<Vec<Element>> for Element {
    fn from(v: Vec<Element>) -> Self {
        Element::List(v)
    }
}

fn parse_line(l: &str) -> Element {
    let mut lists_stack = vec![vec![]];
    let mut iter = l.chars().peekable();
    iter.next();
    iter.next_back();
    while iter.peek().is_some() {
        let c = iter.next().unwrap();
        match c {
            '[' => {
                lists_stack.push(vec![]);
            }
            ']' => {
                // closing of the outermost list
                if lists_stack.len() == 1 {
                    break;
                }

                let v = lists_stack.pop().unwrap();
                lists_stack.last_mut().unwrap().push(v.into());
            }
            ',' => {}
            _ => {
                let mut v = vec![c];
                while iter.peek().map_or(false, |c| c.is_digit(10)) {
                    v.push(iter.next().unwrap());
                }
                let s: String = v.into_iter().collect();
                lists_stack
                    .last_mut()
                    .unwrap()
                    .push(s.parse::<i32>().unwrap().into());
            }
        }
    }

    Element::List(lists_stack.into_iter().next().unwrap())
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // cases that translate into List v. List
            (n @ Element::Number(_), l @ Element::List(_)) => n.clone().lift().cmp(l),
            (l @ Element::List(_), n @ Element::Number(_)) => l.cmp(&n.clone().lift()),

            // actual cases
            (Element::Number(one), Element::Number(two)) => one.cmp(two),
            (Element::List(one), Element::List(two)) => {
                let first_inequal = one
                    .iter()
                    .zip(two.iter())
                    .map(|(left, right)| left.cmp(right))
                    .find(|o| !matches!(o, Ordering::Equal));

                match first_inequal {
                    Some(o) => o,
                    None => one.len().cmp(&two.len()),
                }
            }
        }
    }
}

pub struct Problem13;
impl Problem for Problem13 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let elements: Vec<_> = lines
            .iter()
            .filter(|l| !l.is_empty())
            .map(|l| parse_line(l))
            .collect();

        elements
            .iter()
            .step_by(2)
            .zip(elements.iter().skip(1).step_by(2))
            .enumerate()
            .filter_map(|(i, (one, two))| match one <= two {
                true => Some(i + 1),
                false => None,
            })
            .sum::<usize>()
            .to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let divider_packets: [Element; 2] = [
            Element::from(2_i32).lift().lift(),
            Element::from(6_i32).lift().lift(),
        ];
        let mut elements: Vec<_> = lines
            .iter()
            .filter(|l| !l.is_empty())
            .map(|l| parse_line(l))
            .chain(divider_packets.clone().into_iter())
            .collect();

        elements.sort();

        elements
            .into_iter()
            .enumerate()
            .filter_map(|(i, element)| {
                match element == divider_packets[0] || element == divider_packets[1] {
                    true => Some(i + 1),
                    false => None,
                }
            })
            .product::<usize>()
            .to_string()
    }
}
