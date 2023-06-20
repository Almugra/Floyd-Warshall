use std::env;

const INF: i32 = 3000000;

fn main() {
    let mut args = env::args();
    args.next();

    let path = args.next().unwrap();

    let data = std::fs::read_to_string(path).expect("tried reading file");

    let mut data = data.lines();

    data.next(); // irrelevant comment

    let node_number: usize = data
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    println!("n = {node_number}");

    let mut matrix: Vec<Vec<i32>> = vec![vec![INF; node_number]; node_number];

    for line in data {
        let mut line_split = line.split(' ').collect::<Vec<&str>>();
        let nr = line_split[0].parse::<usize>().unwrap();
        if line_split.len() > 1 {
            let adj_list = line_split.split_off(2);
            for x in adj_list.iter() {
                let nr_weight = x.split('w').collect::<Vec<&str>>();
                let node = nr_weight[0].parse::<usize>().unwrap();
                let weight = nr_weight[1].parse::<i32>().unwrap();
                matrix[nr][node] = weight;
            }
        }
    }

    (0..node_number).for_each(|i| {
        matrix[i][i] = 0_i32;
    });

    for k in 0..node_number {
        for i in 0..node_number {
            for j in 0..node_number {
                if matrix[i][k] != INF && matrix[k][j] != INF {
                    matrix[i][j] = matrix[i][j].min(matrix[i][k] + matrix[k][j]);
                }
            }
        }
    }

    (0..node_number).for_each(|x| {
        let mut string = String::default();
        for y in 0..node_number {
            let weight = matrix[x][y];
            if weight != INF {
                string.push_str(&format!("{y}w{weight} "));
            }
        }
        println!("{x} : {}", string.trim_end());
        string.clear();
    });
}
