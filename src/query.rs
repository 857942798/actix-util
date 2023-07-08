#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QueryOutput<T> {
    pub items: Vec<T>,

    pub limit: usize,

    pub total: usize,
}

impl<T> QueryOutput<T> {
    pub fn items(mut self, items: Vec<T>) -> Self {
        self.total = items.len();
        self.items = items;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn total(mut self, count: usize) -> Self {
        self.total = count;
        self
    }
}
