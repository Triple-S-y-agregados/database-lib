use super::schema::records;

#[derive(Queryable,Clone)]
pub struct Record {
    pub id: i32,
    pub timestamp: String,
    pub voltage: f32,
}

#[derive(Insertable)]
#[table_name="records"]
pub struct NewRecord<'a> {
    pub timestamp: &'a str,
    pub voltage: &'a f32,
}
