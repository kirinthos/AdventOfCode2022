use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

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

fn distance_matrix(graph: &DirectedGraph<String, i32>) -> (HashMap<String, usize>, Vec<Vec<i32>>) {
    let number_of_nodes = graph.nodes.len();
    let mut ordered_node_labels = graph.nodes.keys().collect::<Vec<_>>();
    ordered_node_labels.sort();

    let mut v: Vec<Vec<i32>> =
        std::iter::repeat(std::iter::repeat(1000).take(number_of_nodes).collect())
            .take(number_of_nodes)
            .collect();

    (0..number_of_nodes).for_each(|i| {
        v[i][i] = 0;
    });

    ordered_node_labels
        .iter()
        .enumerate()
        .for_each(|(i, &node_label)| {
            graph
                .nodes
                .get(node_label)
                .unwrap()
                .next
                .borrow()
                .iter()
                .for_each(|next_node| {
                    let next_i = ordered_node_labels
                        .iter()
                        .position(|&label| label == &next_node.label)
                        .unwrap();
                    v[i][next_i] = 1;
                });
        });

    // floyd-warshall algorithm
    for k in 0..number_of_nodes {
        for i in 0..number_of_nodes {
            for j in 0..number_of_nodes {
                v[i][j] = v[i][j].min(v[i][k] + v[k][j]);
            }
        }
    }

    (
        ordered_node_labels
            .into_iter()
            .enumerate()
            .map(|(i, label)| (label.clone(), i))
            .collect(),
        v,
    )
}

fn find_max_pressure(
    graph: &[Vec<i32>],
    viable_valves: &HashMap<usize, i32>,
    current_valve: usize,
    time: i32,
    opened: i64,
) -> i32 {
    let mut max_pressure = 0;
    for (&valve, rate) in viable_valves.iter() {
        let valve_position = 0x1 << valve;
        // already opened
        if opened & valve_position != 0 {
            continue;
        }

        let cost = graph[current_valve][valve] + 1;
        if time - cost >= 0 {
            max_pressure = max_pressure.max(
                rate * (time - cost)
                    + find_max_pressure(
                        graph,
                        viable_valves,
                        valve,
                        time - cost,
                        opened | valve_position,
                    ),
            )
        }
    }

    max_pressure
}

fn find_max_pressure_2(
    graph: &[Vec<i32>],
    viable_valves: &HashMap<usize, i32>,
    valve_person: usize,
    time_person: i32,
    valve_elephant: usize,
    time_elephant: i32,
    opened: i64,
) -> i32 {
    let mut max_pressure = 0;
    for (&valve, rate) in viable_valves.iter() {
        let valve_position = 0x1 << valve;
        // already opened
        if opened & valve_position != 0 {
            continue;
        }

        // person does it
        let (current_valve, time) = (valve_person, time_person);
        let cost = graph[current_valve][valve] + 1;
        if time - cost >= 0 {
            max_pressure = max_pressure.max(
                rate * (time - cost)
                    + find_max_pressure_2(
                        graph,
                        viable_valves,
                        valve,
                        time - cost,
                        valve_elephant,
                        time_elephant,
                        opened | valve_position,
                    ),
            )
        }

        // elephant does it
        let (current_valve, time) = (valve_elephant, time_elephant);
        let cost = graph[current_valve][valve] + 1;
        if time - cost >= 0 {
            max_pressure = max_pressure.max(
                rate * (time - cost)
                    + find_max_pressure_2(
                        graph,
                        viable_valves,
                        valve_person,
                        time_person,
                        valve,
                        time - cost,
                        opened | valve_position,
                    ),
            )
        }
    }

    max_pressure
}

pub struct Problem16;
impl Problem for Problem16 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let graph = read_input(lines);

        let (label_to_index, matrix) = distance_matrix(&graph);
        let viable_valves = &label_to_index
            .iter()
            .map(|(k, _)| graph.nodes.get(k).unwrap())
            .filter(|node| node.item > 0)
            .map(|n| (*label_to_index.get(&n.label).unwrap(), n.item))
            .collect();
        find_max_pressure(&matrix, viable_valves, 0, 30, 0).to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        let graph = read_input(lines);

        let (label_to_index, matrix) = distance_matrix(&graph);
        let viable_valves = &label_to_index
            .iter()
            .map(|(k, _)| graph.nodes.get(k).unwrap())
            .filter(|node| node.item > 0)
            .map(|n| (*label_to_index.get(&n.label).unwrap(), n.item))
            .collect();
        find_max_pressure_2(&matrix, viable_valves, 0, 26, 0, 26, 0).to_string()
    }
}
