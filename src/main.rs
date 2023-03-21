use std::{fs, collections::{HashMap, HashSet}};
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Operation { Read, Write }

#[derive(Debug, PartialEq, Eq)]
struct Item(String);

#[derive(Debug)]
struct Statement {
    trans_id: u32,
    op_type: Operation,
    data_item: Item,
}

#[derive(Debug)]
struct Graph(HashMap::<u32, HashSet<u32>>);

impl Graph {
    fn new() -> Graph {
        Graph(HashMap::new()) 
    }

    fn add_edge(&mut self, to: u32, from: u32) {
        let adjacency_list = &mut self.0;
        adjacency_list.entry(to).or_default().insert(from);
    }

    fn detect_cycle(&self) {
         
    }
}

fn main() {
    let raw_schedule = match fs::read_to_string("schedule.txt") {
        Ok(file) => file,
        Err(_) => panic!("Unable to read the specified file"),
    };

    let mut schedule: Vec<Statement> = Vec::new();
    for raw_statement in raw_schedule.lines() {
        let re = Regex::new(r"T(?P<trans_id>[0-9]+).(?P<op_type>read|write)\((?P<data_item>[a-zA-Z]+)\)").expect("Invalid regular expression");
        let captures = re.captures(&raw_statement).expect("Something went wrong when capturing regex groups");
        
        let trans_id: u32 = captures["trans_id"].parse().expect("Invalid trans_id (SHOULD NOT HAPPEN)");
        let op_type: Operation = match &captures["op_type"] {
            "read" => Operation::Read,
            "write" => Operation::Write,
            _ => panic!("Invalid op_type (SHOULD NOT HAPPEN)")
        };
        let data_item: Item = Item(captures["data_item"].to_string());

        let statement = Statement{ trans_id, op_type, data_item };
        schedule.push(statement);
    }

    let mut graph = Graph::new();
    for i in 0..schedule.len() {
        for j in (i+1)..schedule.len() {
            if schedule[i].data_item != schedule[j].data_item {
                continue;
            }
            if schedule[i].op_type == Operation::Read && schedule[j].op_type == Operation::Read {
                continue;
            }
            if schedule[i].trans_id == schedule[j].trans_id {
                continue;
            }

            graph.add_edge(schedule[i].trans_id, schedule[j].trans_id);
        }
    }

    println!("{:?}", graph);
}

