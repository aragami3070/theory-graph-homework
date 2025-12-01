use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Debug,
};

use serde::{Serialize, de::DeserializeOwned};

use crate::graph::core::{Graph, GraphError, GraphKindError};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn ford_falkerson<T: Clone + DeserializeOwned + Debug + Serialize + Default>(graph: &Graph<T>) {
    let mut capacity: HashMap<(u32, u32), u32> = HashMap::new();

    let mut flow: HashMap<(u32, u32), u32> = HashMap::new();
}

pub fn task_11<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<Graph<T>> {
    if !graph.get_is_directed() {
        return Err(Box::new(GraphError::new(
            GraphKindError::GraphMustBeDirected,
            "по условию должен быть сеть (орграф)",
        )));
    }

    Ok(graph.clone())
}
