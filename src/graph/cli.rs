use std::{
    error::Error,
    fmt::{Debug, Display},
};

use serde::{Serialize, de::DeserializeOwned};

use crate::{
    graph::core::{Adjacency, Edge, Graph, GraphError, GraphKindError, GraphType, Node},
    tasks::{
        task_2::task_2_4, task_3::task_3_5, task_4::task_4_6, task_5::task_5_18, task_6::task_6_4,
    },
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn cli_interface() -> Result<()> {
    println!("Создать ориентированный граф?");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let is_directed: bool = input.trim().parse()?;

    let mut graph: Graph<u32> = Graph::new(None, Adjacency::default(), is_directed);

    loop {
        print_choices();
        input.clear();
        std::io::stdin().read_line(&mut input)?;

        let choice: u8 = input.trim().parse()?;
        match choice {
            0 => choice_0(&graph),

            1 => choice_1(&mut graph)?,

            2 => choice_2(&mut graph)?,

            3 => choice_3(&mut graph)?,

            4 => choice_4(&mut graph)?,

            5 => choice_5(&graph)?,

            6 => graph = choice_6()?,

            7 => choice_7(&graph)?,

            8 => choice_8(&graph)?,

            9 => choice_9(&mut graph)?,

            10 => choice_10(&graph)?,

            11 => choice_11(&graph)?,

            12 => choice_12(&graph),

            _ => {
                break;
            }
        }
    }
    Ok(())
}

fn print_choices() {
    println!("===========================================================");
    println!("0. Вывести граф.");
    println!("1. Добавить вершину.");
    println!("2. Добавить ребро.");
    println!("3. Удалить вершину.");
    println!("4. Удалить ребро.");
    println!("5. Сохранить в файл.");
    println!("6. Создать из файла.");
    println!("7. Вывести полустепень захода данной вершины орграфа. (задание 2)");
    println!("8. Для каждой вершины орграфа вывести её степень. (задание 3)");
    println!("9. Построить граф, являющийся пересечением двух заданных. (задание 4)");
    println!(
        "10. Проверить, является ли граф деревом, или лесом, или не \
			является ни тем, ни другим. (задание 5)"
    );
    println!("11. Выяснить, является ли граф связным. (задание 6)");
    println!("12. Найти каркас минимального веса в неориентированном графе");
    println!("13 и больше. Выйти");
    println!("===========================================================");
}

fn choice_0<T: Display + Clone + DeserializeOwned + Debug + Serialize + Default>(graph: &Graph<T>) {
    println!(
        "Граф: {}",
        if graph.get_is_directed() {
            "ориентированный"
        } else {
            "неориентированный"
        }
    );
    println!("{graph}")
}

fn choice_1(graph: &mut Graph<u32>) -> Result<()> {
    println!("Введите номер вершины:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let node_number: u32 = input.trim().parse()?;

    println!("Введите ее значение:");
    input.clear();
    std::io::stdin().read_line(&mut input)?;
    let node_value: u32 = input.trim().parse()?;

    graph.add_node(Node::new(node_number, node_value))?;
    Ok(println!("{graph}"))
}

fn choice_2(graph: &mut Graph<u32>) -> Result<()> {
    println!("Введите номер вершины из которой будет выходить ребро:");

    let mut input = String::new();
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
    Ok(println!("{graph}"))
}

fn choice_3(graph: &mut Graph<u32>) -> Result<()> {
    println!("Введите номер вершины:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let node: Node<u32> = Node::new(input.trim().parse::<u32>()?, 0);

    graph.delete_node(&node)?;
    Ok(println!("{graph}"))
}

fn choice_4(graph: &mut Graph<u32>) -> Result<()> {
    println!("Введите номер вершины:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let node: Node<u32> = Node::new(input.trim().parse::<u32>()?, 0);

    println!("Введите номер вершины куда идет ребро:");

    input.clear();
    std::io::stdin().read_line(&mut input)?;
    let edge_index: u32 = input.trim().parse()?;

    graph.delete_edge(&node, &edge_index)?;
    Ok(println!("{graph}"))
}

fn choice_5<T: Display + Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<()> {
    println!("Введите путь до файла:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    graph.write_in_file(input.trim_end())?;
    Ok(println!("Граф сохранен"))
}

fn choice_6<T: Display + Clone + DeserializeOwned + Debug + Serialize + Default>()
-> Result<Graph<T>> {
    println!("Введите путь до файла:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let graph = Graph::new_from_file(input.trim_end())?;
    println!("{graph}");
    Ok(graph)
}

fn choice_7<T: Display + Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<()> {
    if !graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть орграф",
        )));
    }

    println!("Введите номер вершины:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let node_number: u32 = input.trim().parse()?;

    println!(
        "Полустепень захода вершины {{{node_number}}} = {}",
        task_2_4(&graph, &node_number)?
    );
    Ok(())
}

fn choice_8<T: Display + Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<()> {
    for (index, count) in task_3_5(&graph)? {
        println!("Степень вершины {{{index}}} = {count}")
    }

    Ok(())
}

fn choice_9<T: Display + Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &mut Graph<T>,
) -> Result<()> {
    println!("Введите путь до файла (для второго графа):");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let mut other_graph: Graph<T> = Graph::new_from_file(input.trim_end())?;

    Ok(println!("{}", task_4_6(graph, &mut other_graph)?))
}

fn choice_10<T: Display + Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<()> {
    println!(
        "Данный граф {}",
        match task_5_18(&graph)? {
            GraphType::Tree => "--- дерево",
            GraphType::Forest => "--- лес",
            _ => "не является ни дервом ни лесом",
        }
    );
    Ok(())
}

fn choice_11<T: Display + Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<()> {
    println!(
        "Данный граф: {}",
        match task_6_4(&graph)? {
            true => "связен",
            false => "не связен",
        }
    );
    Ok(())
}

fn choice_12<T: Display + Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) {
}
