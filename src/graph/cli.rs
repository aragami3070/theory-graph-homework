use std::error::Error;

use crate::graph::core::{Adjacency, Edge, Graph, Node};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn cli_interface() -> Result<()> {
    println!("Создать ориентированный граф?");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let is_directed: bool = input.trim().parse()?;
    let mut graph: Graph<u32> = Graph::new(None, Adjacency::default(), is_directed);

    loop {
        println!("1. Добавить вершину");
        println!("2. Добавить ребро");
        println!("3. Удалить вершину");
        println!("4. Удалить ребро");
        println!("5. Сохранить в файл");
        println!("6. Создать из файла");
        println!("7 и больше. Выйти");

        input.clear();
        std::io::stdin().read_line(&mut input)?;

        let choice: u8 = input.trim().parse()?;
        match choice {
            1 => {
                println!("Введите номер вершины:");
                input.clear();
                std::io::stdin().read_line(&mut input)?;
                let node_number: u32 = input.trim().parse()?;

                println!("Введите ее значение:");
                input.clear();
                std::io::stdin().read_line(&mut input)?;
                let node_value: u32 = input.trim().parse()?;

                graph.add_node(Node::new(node_number, node_value))?;
                println!("{graph}")
            }

            2 => {
                println!("Введите номер вершины из которой будет выходить ребро:");

                input.clear();
                std::io::stdin().read_line(&mut input)?;
                let node_number: u32 = input.trim().parse()?;

                println!("Введите номер вершины в которую будет идти ребро:");

                input.clear();
                std::io::stdin().read_line(&mut input)?;
                let edge_node_number: u32 = input.trim().parse()?;

                println!("Введите ее значение:");

                input.clear();
                std::io::stdin().read_line(&mut input)?;
                let edge_node_value: u32 = input.trim().parse()?;

                println!("Введите вес ребра:");

                input.clear();
                std::io::stdin().read_line(&mut input)?;
                let edge_node_weight: u32 = input.trim().parse()?;

                let new_edge = Edge::new(&edge_node_number, edge_node_weight, &edge_node_value);

                if graph.get_is_directed() {
                    graph.add_edge(&Node::new(node_number, 0), &new_edge)?;
                } else {
                    println!("Введите значение вершины из которой будет выходить ребро:");
                    input.clear();
                    std::io::stdin().read_line(&mut input)?;
                    let node_value: u32 = input.trim().parse()?;

                    graph.add_edge(&Node::new(node_number, node_value), &new_edge)?;
                }
                println!("{graph}")
            }

            3 => {
                println!("Введите номер вершины:");

                input.clear();
                std::io::stdin().read_line(&mut input)?;
                let node: Node<u32> = Node::new(input.trim().parse::<u32>()?, 0);

                graph.delete_node(&node)?;
                println!("{graph}")
            }

            4 => {
                println!("Введите номер вершины:");

                input.clear();
                std::io::stdin().read_line(&mut input)?;
                let node: Node<u32> = Node::new(input.trim().parse::<u32>()?, 0);

                println!("Введите номер вершины куда идет ребро:");

                input.clear();
                std::io::stdin().read_line(&mut input)?;
                let edge_index: u32 = input.trim().parse()?;

                graph.delete_edge(&node, &edge_index)?;
                println!("{graph}")
            }

            5 => {
                println!("Введите путь до файла:");

                input.clear();
                std::io::stdin().read_line(&mut input)?;

                graph.write_in_file(input.trim_end())?;
                println!("Граф сохранен")
            }

            6 => {
                println!("Введите путь до файла:");

                input.clear();
                std::io::stdin().read_line(&mut input)?;

                graph = graph.new_from_file(input.trim_end())?;
                println!("{graph}")
            }

            _ => {
                break;
            }
        }
    }
    Ok(())
}
