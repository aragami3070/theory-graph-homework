mod graph;
use std::{io::Write, process};

use graph::core;
fn main() {
    let node_a = core::Node::<u32>::new(1, 20);
    let node_b = core::Node::<u32>::new(2, 23);
    let node_c = core::Node::<u32>::new(3, 32);
    let node_d = core::Node::<u32>::new(4, 400);
    let node_e = core::Node::<u32>::new(5, 500);

    let edge = core::Edge::<u32>::new(&node_a.number, 140, &node_a.value);
    let edge2 = core::Edge::<u32>::new(&node_b.number, 109, &node_b.value);
    let edge3 = core::Edge::<u32>::new(&node_c.number, 10, &node_c.value);
    let edge4 = core::Edge::<u32>::new(&node_d.number, 14, &node_d.value);
    let edge5 = core::Edge::<u32>::new(&node_e.number, 15, &node_e.value);

    let adjacency = core::Adjacency::<u32>::new(edge2);
    let mut adjacency_list = core::AdjacencyList::<u32>::new(1, adjacency, true);

    println!("Добавление вершин");
    adjacency_list.add_node(node_b.number);
    adjacency_list.add_node(node_c.number);
    adjacency_list.add_node(node_d.number);
    println!("{adjacency_list}");

    println!("Добавление ребер");
    adjacency_list.add_edge(&node_b, &edge);
    adjacency_list.add_edge(&node_b, &edge3);

    adjacency_list.add_edge(&node_c, &edge4);
    adjacency_list.add_edge(&node_c, &edge5);

    adjacency_list.add_edge(&node_d, &edge3);
    println!("{adjacency_list}");

    println!("Удаление ребер");
    adjacency_list.delete_edge(&node_b, &edge3.node.number);
    adjacency_list.delete_edge(&node_b, &edge3.node.number);
    println!("{adjacency_list}");

    println!("Удаление вершин и ребер");
    adjacency_list.delete_node(&node_c);
    println!("{adjacency_list}");

	println!("Запись в файл");
    if let Err(err) = adjacency_list.write_in_file("assets/output.json") {
        eprintln!("Error: {err}");
        process::exit(1);
    }
	println!("Успешно");

	println!("Создание из файла");
    adjacency_list = match adjacency_list.new_from_file("assets/output.json") {
        Ok(list) => list,
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    };

	println!("{adjacency_list}")
}
