#[derive(Debug, Clone)]
pub struct Table {
    pub headers: Vec<Value>,
    pub rows: Vec<Vec<Value>>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(usize),
    Table(Table),
    Nil,
}
