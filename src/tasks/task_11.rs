use std::{collections::HashSet, error::Error, fmt::Debug};

use serde::{Serialize, de::DeserializeOwned};

use crate::graph::core::{Graph, GraphError, GraphKindError};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn task_11<T: Clone + DeserializeOwned + Debug + Serialize + Default>(
    graph: &Graph<T>,
) -> Result<Graph<T>> {
    Ok(graph.clone())
}
