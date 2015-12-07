
use r2d2_postgres::PostgresConnectionManager;
use r2d2::PooledConnection;

pub fn db_init(db_conn: &PooledConnection<PostgresConnectionManager>) {
    db_conn.execute(
        "create table if not exists users (
            id serial primary key,
            first_name varchar not null,
            last_name varchar not null
        )",
        &[]
    ).unwrap();
}
