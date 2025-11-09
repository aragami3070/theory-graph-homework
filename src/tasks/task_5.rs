use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Debug,
};

use serde::{Serialize, de::DeserializeOwned};

use crate::graph::core::{ColorNode, Graph, GraphError, GraphKindError, GraphType};

type Result<T> = std::result::Result<T, Box<dyn Error>>;


fn graph_have_cycle<T: Clone + DeserializeOwned + Debug + Serialize>(
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
fn is_a_tree<T: Clone + DeserializeOwned + Debug + Serialize>(graph: &Graph<T>) -> Result<bool> {
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
