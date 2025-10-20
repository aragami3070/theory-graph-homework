use std::{error::Error, fmt::Debug};

use serde::{Serialize, de::DeserializeOwned};

use crate::graph::core::{Graph, GraphError, GraphKindError, GraphType};


pub fn task_5_18<T: Clone + DeserializeOwned + Debug + Serialize>(
    graph: &Graph<T>,
) -> Result<GraphType, Box<dyn Error>> {
	if !graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть орграф",
        )));
	}

	let mut count = graph.len();
	for (ind, adjacency) in graph {
	    count -= adjacency.len();

	}

    Ok(GraphType::Default)
}
