use std::collections::HashMap;
use itertools::Itertools;

pub fn main() {
    let data = adapt(include_str!("../input.txt"));
    let graph = create_graph(data);
    let mut path: Vec<String> = Vec::new();
    let mut paths: Vec<Vec<String>> = Vec::new();
    dfs(&graph, String::from("start"), &mut path , &mut paths);
    println!("Found {} paths.", &paths.len());
}

//Adapt function that converts input data into a Vec<String>
fn adapt(data: &str) -> Vec<String> {
    return data.lines().map(|i| i.to_string()).collect();
}

fn create_edges(data: Vec<String>) -> Vec<(String, String)> {
    let edges: Vec<(String, String)> = data.iter().map(|l| l.split("-").map(|s| String::from(s)).collect_tuple().unwrap()).collect();
    let start = String::from("start");
    let end = String::from("end");
    return append_v(
        edges.to_vec().into_iter().filter(|e| e.0.ne(&end) && e.1.ne(&start)).collect(),
        edges.to_vec().into_iter().filter(|e| e.1.ne(&end) && e.0.ne(&start)).map(|e| (e.1, e.0)).collect::<Vec<(String, String)>>())
        .into_iter()
        .unique()
        .collect();
}

fn create_graph(data: Vec<String>) -> HashMap::<String, Vec<String>> {
    return create_edges(data)
        .into_iter()
        .fold(HashMap::<String, Vec<String>>::new(), |mut graph, e| {
            if !graph.entry(e.0.to_string()).or_default().contains(&e.1) {
                graph.entry(e.0.to_string()).or_default().push(e.1);
            }
            graph
        });
}

fn is_lowercase(item: &String) -> bool {
    return item.chars().next().unwrap().is_lowercase();
}

fn dfs(graph: &HashMap<String, Vec<String>>, node: String, path: &mut Vec<String>, paths: &mut Vec<Vec<String>>) {
    path.push(node.to_string());
    if node.eq("end") {
        paths.push(path.to_vec());
    } else {
        let last = if path.is_empty() { "start" } else { path.last().expect("Path should not be empty!")};
        let occurances = path.iter().fold(HashMap::<String, usize>::new(), |mut m, x| {
            *m.entry(x.to_string()).or_default() += 1;
            m
        });

        //Just keep lowercase elements that have been visited more than once
        let has_two_occurances = occurances.clone().into_iter().filter(|e| is_lowercase(&e.0) && &e.1 >= &(2 as usize)).count() > 0;
        let next: Vec<String> = graph.get(last).unwrap().to_vec();
        next.into_iter()
            .filter(|e| "start".ne(e) && (!is_lowercase(e)
                                          || !has_two_occurances
                                          || occurances.get(e).unwrap_or(&(0 as usize)) == &(0 as usize)))
            .for_each(|e| dfs(graph, e.to_string(), &mut path.to_vec(), paths));
    }
}

fn append_v<T: std::clone::Clone>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = left.to_vec();
    result.extend(right);
    return result;
}
