use fake::Fake;
use itertools::Itertools;
use mysql::{from_row, Conn};
use uuid::Uuid;

pub fn token_create(conn: &mut Conn) {
    conn.prep_exec("create table token ( user_id int, value char(64), permission json, primary key (user_id) )", ()).unwrap();
}

pub fn token_insert(conn: &mut Conn, user_id: usize) {
    let value = Uuid::new_v4().to_string();
    let json = format!(r#"{{ "read": [{}], "write": [{}] }}"#, get_permission(), get_permission());

    conn.prep_exec("insert into token ( user_id, value, permission ) values ( ?, ?, ? )", (user_id, value, json)).unwrap();
}

pub fn token_delete(conn: &mut Conn, user_id: usize) {
    conn.prep_exec("delete from token where user_id = ?", vec![user_id]).unwrap();
}

pub fn token_drop(conn: &mut Conn) {
    conn.prep_exec("drop table if exists token", ()).unwrap();
}

fn get_permission() -> String {
    let n: usize = (0..8).fake();
    let bits = format!("{:0width$b}", n, width = 3).chars().collect_vec();
    vec!["user", "message", "item"].into_iter().zip(&bits).filter(|&(_, c)| *c == '1').map(|(v, _)| format!(r#""{v}""#)).join(", ")
}

pub fn token_ids(conn: &mut Conn, generated: bool) -> Vec<usize> {
    let not = if generated { "not " } else { "" };
    conn.query(format!("select u.id from user u left outer join token t on u.id = t.user_id where t.user_id is {not}null"))
        .map(|result| result.map(|x| x.unwrap()).map(from_row::<usize>).collect())
        .unwrap()
}
