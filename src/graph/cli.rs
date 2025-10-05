use std::error::Error;

use crate::graph::core::{Adjacency, AdjacencyList, Edge, Node};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn cli_interface() -> Result<()> {
    println!("Создать ориентированный граф?");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let is_directed: bool = input.trim().parse()?;
    let mut adjacency_list: AdjacencyList<u32> =
        AdjacencyList::new(None, Adjacency::default(), is_directed);
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
                adjacency_list.add_node(node_number)?;
                println!("{adjacency_list}")
            }
            2 => {
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

                println!("Введите номер вершины из которой будет выходить ребро:");
                input.clear();
                std::io::stdin().read_line(&mut input)?;
                let node_number: u32 = input.trim().parse()?;

                if adjacency_list.get_is_directed() {
                    adjacency_list.add_edge(&Node::new(node_number, 0), &new_edge)?;
                } else {
                    println!("Введите ее значение:");
                    input.clear();
                    std::io::stdin().read_line(&mut input)?;
                    let node_value: u32 = input.trim().parse()?;

                    adjacency_list.add_edge(&Node::new(node_number, node_value), &new_edge)?;
                }
                println!("{adjacency_list}")
            }
            3 => {
                println!("Введите номер вершины:");
				input.clear();
				std::io::stdin().read_line(&mut input)?;
				let node: Node<u32> = Node::new(input.trim().parse::<u32>()?, 0);
				adjacency_list.delete_node(&node)?;
            }
            4 => {
                todo!()
            }
            5 => {
                todo!()
            }
            6 => {
                todo!()
            }
            _ => {
                break;
            }
        }
    }
    Ok(())
}
