use std::{error::Error, fmt::Debug};

use serde::{Serialize, de::DeserializeOwned};

use crate::graph::core::{Adjacency, Graph};

pub fn task_4_6<T: Clone + DeserializeOwned + Debug + Serialize>(
    graph_1: &mut Graph<T>,
    graph_2: &mut Graph<T>,
) -> Result<Graph<T>, Box<dyn Error>> {
	// Приводим к графы к ориентированным графам, если у них разная ориентация
    if graph_1.get_is_directed() != graph_2.get_is_directed() {
        graph_1.to_directed();
        graph_2.to_directed();
    }

    let mut res_graph: Graph<T> = Graph::new(None, Adjacency::default(), graph_1.get_is_directed());

    // Находим и добавляем все общие вершины в граф вместе с общими ребрами
    for (index, adjacency_s) in graph_2.iter() {
        if let Some(node) = graph_1.get_node(index) {
            // Добавили общую вершину
            res_graph.add_node(node.clone())?;

            // Добавили общие ребра
            if let Some(adjacency_f) = graph_1.get_adjacency(index) {
                for edge in adjacency_s.into_iter() {
                    if let Some(node_s) = graph_2.get_node(&edge.node.number) {
                        if adjacency_f.contains(edge) {
							// Если второй вершины нет новом графе, то добавляем
                            if res_graph.get_node(index).is_none() {
                                res_graph.add_node(node_s.clone())?;
                            }
                            res_graph.add_edge(node, edge)?
                        }
                    }
                }
            }
        }
    }

    Ok(res_graph)
}
