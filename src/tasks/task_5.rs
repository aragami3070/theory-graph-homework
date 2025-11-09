use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Debug,
};

use serde::{Serialize, de::DeserializeOwned};

use crate::graph::core::{ColorNode, Graph, GraphError, GraphKindError, GraphType};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn dfs<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    start: &u32,
    visited: &mut HashSet<u32>,
    components: &mut Vec<u32>,
) {
    if !visited.contains(start) {
        // Добавялем вершину в список компонентов связности
        components.push(*start);
    }

    // Отмечаем вершину как посещенную
    visited.insert(*start);

    // Получаем ребра данной вершины
    if let Some(adjacency) = graph.get_adjacency(start) {
        for neighbor in adjacency {
            if visited.get(&neighbor.node.number).is_none() {
                dfs(graph, &neighbor.node.number, visited, components);
            }
        }
    }
}

fn is_a_forest<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<bool> {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut subgraphs: Vec<Graph<T>> = Vec::new();

    // Разбиваем граф на подграфы по связности
    for (ind, _) in graph {
        if !visited.contains(ind) {
            let mut compnents = Vec::new();
            dfs(graph, ind, &mut visited, &mut compnents);
            subgraphs.push(graph.create_subgraph(compnents, true)?);
        }
    }

    // Проверяем, что все из них деревья
    let mut is_forest = true;
    for subgraph in subgraphs {
        println!("{subgraph:?}");
        is_forest = is_a_tree(&subgraph)?;
        println!("{is_forest}");
        if !is_forest {
            break;
        }
    }

    Ok(is_forest)
}

// Проверяет есть ли цикл в графе из вершины start
fn graph_have_cycle<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
    start: &u32,
    visited: &mut HashMap<u32, ColorNode>,
) -> Result<bool> {
    if let Some(color) = visited.get_mut(start) {
        // Если мы заходим в серую вершину, то нашли цикл
        if *color == ColorNode::Gray {
            return Ok(true);
        }

        // Перекрашиваем вершину в серую
        *color = ColorNode::Gray;
    } else {
        return Err(Box::new(GraphError::new(
            GraphKindError::NodeNotFound,
            "Граф пустой",
        )));
    };

    if let Some(adjacency) = graph.get_adjacency(start) {
        // Проходимся по всем смежным вершинам
        for neighbor in adjacency {
            // Если
            if let Some(color) = visited.get(&neighbor.node.number)
                && *color == ColorNode::White
            {
                if graph_have_cycle(graph, &neighbor.node.number, visited)? {
                    return Ok(true);
                }
            }
        }
    }

    if let Some(node_index) = visited.get_mut(start) {
        // Перекрашиваем вершину в черную
        *node_index = ColorNode::Black
    }

    Ok(false)
}

// Проверка является ли граф деревом
fn is_a_tree<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<bool> {
    let mut count = graph.len();
    let mut cycle = false;
    for (index, adjacency) in graph {
        count -= adjacency.len();
        if !cycle {
            let mut visited = graph.get_nodes_with_color();
            cycle = graph_have_cycle(graph, index, &mut visited)?;
        }
    }
    Ok(count == 1 && !cycle)
}

pub fn task_5_18<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<GraphType> {
    if !graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть орграф",
        )));
    }

    if is_a_tree(graph)? {
        return Ok(GraphType::Tree);
    }

    if is_a_forest(graph)? {
        return Ok(GraphType::Forest);
    }

    Ok(GraphType::Default)
}
