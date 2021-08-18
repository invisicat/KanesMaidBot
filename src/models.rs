use diesel::sql_types::Timestamptz;

#[derive(Queryable)]
pub struct Server {
    pub id: i32,
    pub guild: i64,
    pub joined: Timestamptz,
    pub blacklisted: bool,
}
