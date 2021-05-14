#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub timestamp: String,
    pub voltage: i32,
}