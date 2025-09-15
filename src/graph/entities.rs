type Index = u32;
type Weight = u32;

pub struct Edge<T> {
    pub number: Index,
    pub weight: Weight,
    pub value: T,
}

impl<T> Default for Edge<T>
where
    T: Default,
{
    fn default() -> Self {
        Edge::<T> {
            number: 0,
            weight: 0,
            value: T::default(),
        }
    }
}

impl<T> Edge<T> {
    pub fn new(number: Index, weight: Weight, value: T) -> Self {
        Self {
            number,
            weight,
            value,
        }
    }
}
