use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
}

type PNode = Rc<Node<String, i32>>;

struct DirectedGraph<Label, T> {
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

    DirectedGraph { nodes: node_map }
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

fn powerset<T>(s: &[T]) -> Vec<Vec<&T>> {
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element)
                .collect()
        })
        .collect()
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
            .collect::<HashMap<_, _>>();

        // finding the other open valves
        let complement: i64 = (1 << (viable_valves.iter().map(|(v, _)| v).max().unwrap() + 1)) - 1;
        let valid_valves: i64 = viable_valves.keys().fold(0, |acc, n| 1 << n | acc);

        let mut pressures = HashMap::new();
        for v in powerset(&viable_valves.iter().collect::<Vec<(_, _)>>()) {
            let opened = valves_to_bits(&v);
            pressures.insert(
                opened,
                find_max_pressure(&matrix, viable_valves, 0, 26, opened),
            );
        }

        println!("finding best combination");
        let mut max_pressure = 0;
        for (valves, pressure) in pressures.iter() {
            let other_pressure = pressures
                .get(&((valves ^ complement) & valid_valves))
                .unwrap();
            max_pressure = max_pressure.max(pressure + other_pressure)
        }

        max_pressure.to_string()
    }
}

fn valves_to_bits(valves: &[&(&usize, &i32)]) -> i64 {
    valves.iter().fold(0, |acc, (&n, _)| (1 << n as i64) | acc)
}
