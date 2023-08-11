use fake::faker::name::en::Name;
use fake::Fake;

use mysql::{from_row, Conn};

pub fn user_create(conn: &mut Conn) {
    conn.prep_exec("create table user ( id int auto_increment, name char(64), age smallint, created_at datetime default current_timestamp, updated_at datetime default current_timestamp, deleted_at datetime default null, primary key (id) )", ()).unwrap();
}

pub fn user_insert(conn: &mut Conn) {
    let name: String = Name().fake();
    let age: usize = (1..100).fake();

    conn.prep_exec("insert into user ( name, age ) values ( ?, ? )", (name, age)).unwrap();
}

pub fn user_update(conn: &mut Conn, id: usize) {
    let name: String = Name().fake();
    let age: usize = (1..100).fake();

    conn.prep_exec("update user set name = ?, age = ?, updated_at = now() where id = ?", (name, age, id)).unwrap();
}

pub fn user_delete(conn: &mut Conn, id: usize) {
    conn.prep_exec("update user set deleted_at = now() where id = ?", vec![id]).unwrap();
}

pub fn user_drop(conn: &mut Conn) {
    conn.prep_exec("drop table if exists user", ()).unwrap();
}

pub fn user_ids(conn: &mut Conn, deleted: bool) -> Vec<usize> {
    let not = if deleted { "not " } else { "" };
    conn.query(format!("select id from user where deleted_at is {not}null"))
        .map(|result| result.map(|x| x.unwrap()).map(from_row::<usize>).collect())
        .unwrap()
}
