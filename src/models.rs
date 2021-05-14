#[derive(Queryable)]
pub struct Record {
    pub id: i32,
    pub timestamp: String,
    pub voltage: i32,
}