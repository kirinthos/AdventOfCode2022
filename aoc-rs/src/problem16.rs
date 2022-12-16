use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;
use regex;

use crate::Problem;

struct Node<Label, T> {
    label: Label,
    item: T,
    next: RefCell<Vec<Rc<Node<Label, T>>>>,
}

impl<Label, T> Node<Label, T>
where
    Label: PartialEq + Clone,
{
    fn new(label: Label, item: T) -> Self {
        Self {
            label,
            item,
            next: RefCell::new(vec![]),
        }
    }

    fn add_next(&self, next: &Rc<Node<Label, T>>) {
        self.next.borrow_mut().push(Rc::clone(next));
    }

    // slow version of this, i imagine
    fn path_to_node(&self, label: &Label) -> Vec<Label> {
        let mut paths = self.path_to_node_inner(label, Vec::new());
        paths.sort_by_key(|p| p.len());
        paths.into_iter().next().unwrap()
    }

    fn path_to_node_inner(&self, label: &Label, mut current_path: Vec<Label>) -> Vec<Vec<Label>> {
        // cycle
        if current_path.contains(&self.label) {
            return vec![];
        }

        current_path.push(self.label.clone());
        match current_path.contains(label) {
            true => vec![current_path],
            false => self
                .next
                .borrow()
                .iter()
                .flat_map(|next_node| next_node.path_to_node_inner(label, current_path.clone()))
                .collect(),
        }
    }
}

type PNode = Rc<Node<String, i32>>;

struct DirectedGraph<Label, T> {
    start: Rc<Node<Label, T>>,
    nodes: HashMap<String, Rc<Node<Label, T>>>,
}

fn read_input(lines: &[String]) -> DirectedGraph<String, i32> {
    let re =
        regex::Regex::new(r#"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)"#)
            .unwrap();
    let (nodes, nexts): (Vec<PNode>, Vec<&str>) = lines
        .iter()
        .map(|l| {
            let captures = re.captures(l).unwrap();
            let (valve, flow_rate, next_valves) = (
                captures.get(1).unwrap().as_str().to_owned(),
                captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(3).unwrap().as_str(),
            );

            let node = Rc::new(Node::new(valve, flow_rate));
            (Rc::clone(&node), next_valves)
        })
        .unzip();

    let node_map: HashMap<_, _> = nodes
        .iter()
        .map(|n| (n.label.clone(), Rc::clone(n)))
        .collect();

    nodes
        .iter()
        .zip(nexts.into_iter())
        .for_each(|(node, next_valves)| {
            next_valves
                .split(", ")
                .for_each(|name| node.add_next(node_map.get(name).unwrap()));
        });

    let start = Rc::clone(node_map.get("AA").unwrap());
    DirectedGraph {
        start,
        nodes: node_map,
    }
}

/* Brute force, sloooowww
fn open_valves(graph: &DirectedGraph<String, i32>) -> Vec<i32> {
    let valuable_nodes: HashSet<_> = graph
        .nodes
        .values()
        .filter(|n| n.item > 0)
        .map(|n| n.label.as_str())
        .collect();

    open_valves_inner(&valuable_nodes, &graph.start, HashSet::new(), 0, 0)
}

fn open_valves_inner(
    valuable_nodes: &HashSet<&str>,
    current_node: &PNode,
    opened_valves: HashSet<&str>,
    counter: i32,
    released: i32,
) -> Vec<i32> {
    if (valuable_nodes - &opened_valves).is_empty() || counter >= 30 {
        return vec![released];
    }

    let mut v = Vec::new();
    // otherwise we could open the current valve
    if current_node.item > 0 && !opened_valves.contains(current_node.label.as_str()) {
        let mut opened_valves = opened_valves.clone();
        opened_valves.insert(&current_node.label);
        v.extend(
            open_valves_inner(
                valuable_nodes,
                current_node,
                opened_valves,
                counter + 1,
                released + current_node.item * (30 - counter),
            )
            .into_iter(),
        );
    }

    // or we could move to another valve
    v.extend(current_node.next.borrow().iter().flat_map(|next_node| {
        open_valves_inner(
            valuable_nodes,
            next_node,
            opened_valves.clone(),
            counter + 1,
            released,
        )
        .into_iter()
    }));

    v
}
*/

fn open_valves(graph: &DirectedGraph<String, i32>) -> Vec<i32> {
    let valuable_nodes: HashSet<_> = graph
        .nodes
        .values()
        .into_iter()
        .filter_map(|n| match n.item > 0 {
            true => Some(n.label.clone()),
            false => None,
        })
        .collect();

    let v = open_valves_inner(
        graph,
        &valuable_nodes,
        vec![graph.start.label.clone()],
        30,
        0,
    );
    for (path, released) in v.iter() {
        println!("released {} on path {:?}", released, path);
    }
    v.into_iter().map(|(_, a)| a).collect()
}

fn open_valves_inner(
    graph: &DirectedGraph<String, i32>,
    valuable_nodes: &HashSet<String>,
    current_path: Vec<String>,
    counter: i32,
    released: i32,
) -> Vec<(Vec<String>, i32)> {
    if counter <= 0
        || valuable_nodes
            .iter()
            .all(|label| current_path.contains(label))
    {
        return vec![(current_path, released)];
    }

    let current_node = graph.nodes.get(current_path.last().unwrap()).unwrap();

    valuable_nodes
        .iter()
        .filter(|n| !current_path.contains(n))
        .map(|n| current_node.path_to_node(n))
        .flat_map(|next_path| {
            let mut p = current_path.clone();
            let next_node = graph.nodes.get(next_path.last().unwrap()).unwrap();
            let counter = counter - next_path.len() as i32;

            p.extend(next_path.into_iter().skip(1));
            open_valves_inner(
                graph,
                valuable_nodes,
                p,
                counter,
                released + next_node.item * counter,
            )
        })
        .collect()
}

pub struct Problem16;
impl Problem for Problem16 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let graph = read_input(lines);

        open_valves(&graph).into_iter().max().unwrap().to_string()
    }

    fn solve_part2(&mut self, _lines: &[String]) -> String {
        todo!()
    }
}
