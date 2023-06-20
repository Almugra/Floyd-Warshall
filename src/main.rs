use std::io::Write;
use std::{env, time::SystemTime};

const INF: i32 = 3_000_000;

fn main() {
    let now = SystemTime::now();
    let mut args = env::args();
    args.next();

    let path = args.next().unwrap();

    let input = std::fs::read_to_string(path).unwrap();

    let mut input_lines = input.lines();

    input_lines.next();

    let line = input_lines.next().unwrap();
    let number_of_nodes: usize = line
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    println!("n = {number_of_nodes}");

    let mut matrix: Vec<Vec<i32>> = vec![vec![INF; number_of_nodes]; number_of_nodes];

    for line in input_lines {
        let mut line_split = line.split_whitespace().collect::<Vec<&str>>();
        let nr = line_split[0].parse::<usize>().unwrap();
        if line_split.len() > 1 {
            let adj_list = line_split.split_off(2);
            for adj in adj_list.iter() {
                let nr_weight = adj.split('w').collect::<Vec<&str>>();
                let node = nr_weight[0].parse::<usize>().unwrap();
                let weight = nr_weight[1].parse::<i32>().unwrap();
                matrix[nr][node] = weight;
            }
        }
    }

    (0..number_of_nodes).for_each(|i| {
        matrix[i][i] = 0;
    });

    for k in 0..number_of_nodes {
        for i in 0..number_of_nodes {
            for j in 0..number_of_nodes {
                unsafe {
                    if *matrix[i].get_unchecked(k) != INF && *matrix[k].get_unchecked(j) != INF {
                        *matrix[i].get_unchecked_mut(j) = (*matrix[i].get_unchecked(j))
                            .min(matrix[i].get_unchecked(k) + matrix[k].get_unchecked(j));
                    }
                }
            }
        }
    }

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    (0..number_of_nodes).for_each(|x| {
        let mut string = String::default();
        for y in 0..number_of_nodes {
            unsafe {
                let weight = *matrix[x].get_unchecked(y);
                if weight != INF {
                    string.push_str(&format!("{y}w{weight} "));
                }
            }
        }
        writeln!(lock, "{} : {}", x, string.trim_end()).unwrap();
        string.clear();
    });

    drop(lock);

    eprintln!("{:?}", now.elapsed());
}
