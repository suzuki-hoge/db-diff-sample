use mysql::Conn;
use uuid::Uuid;

pub fn log_create(conn: &mut Conn) {
    conn.prep_exec("create table log ( id char(64), operation char(64), created_at datetime default current_timestamp, primary key (id) )", ())
        .unwrap();
}

pub fn log_insert(conn: &mut Conn, operation: &str) {
    let id = Uuid::new_v4().to_string();
    conn.prep_exec("insert into log ( id, operation ) values ( ?, ? )", (&id, operation)).unwrap();
}

pub fn log_drop(conn: &mut Conn) {
    conn.prep_exec("drop table if exists log", ()).unwrap();
}
