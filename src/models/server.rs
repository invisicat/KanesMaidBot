use diesel::sql_types::Timestamptz;
use chrono::Utc;
use tracing::{error};
use diesel::{PgConnection, RunQueryDsl};
use crate::schema::servers;

#[derive(Queryable)]
pub struct Server {
    pub id: i32,
    pub guild: i64,
    pub joined: Timestamptz,
    pub blacklisted: bool,
}

pub struct NewServer {
    pub guild: u64,
    pub joined: Utc,
    pub blacklisted: bool,
}

impl NewServer {
    pub fn create(&self, connection: &PgConnection) -> Result<Server, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(servers::table)
            .values(self)
            .get_result(connection)
    }
}

impl Server {
    pub fn update(id: &i32, new_server: &NewServer, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::servers::dsl;

        if let Err(why) = diesel::update(dsl::servers.find(id))
            .set(new_server)
            .execute(conn) {
            error!("Could not update server {} with guild id {} due to reason {}", id, new_server.guild, why);
        }
        Ok(())
    }
}