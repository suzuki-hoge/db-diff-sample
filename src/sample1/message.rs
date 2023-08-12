use fake::faker::lorem::en::Words;
use fake::Fake;

use mysql::Conn;

pub fn message_create(conn: &mut Conn) {
    conn.prep_exec(
        "create table message ( id int auto_increment, user_id int, value text, created_at datetime default current_timestamp, primary key (id) )",
        (),
    )
    .unwrap();
}

pub fn message_insert(conn: &mut Conn, user_id: usize) {
    let ws: Vec<String> = Words(5..20).fake();

    let _ = conn.prep_exec("insert into message ( user_id, value ) values ( ?, ? )", (user_id, ws.join(" ")));
}

pub fn message_delete(conn: &mut Conn, user_id: usize) {
    let _ = conn.prep_exec("delete from message where user_id = ?", vec![user_id]);
}

pub fn message_drop(conn: &mut Conn) {
    conn.prep_exec("drop table if exists message", ()).unwrap();
}
