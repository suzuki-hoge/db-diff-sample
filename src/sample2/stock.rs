use std::sync::Arc;

use fake::faker::lorem::en::Words;
use fake::Fake;
use itertools::Itertools;
use mysql::{from_row, Conn};
use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;
use rayon::prelude::*;
use uuid::Uuid;

pub fn stock_create(conn: &mut Conn) {
    conn.prep_exec("create table stock ( id char(64), name char(64), code char(8), price int, count int, primary key (id) )", ()).unwrap();
}

pub fn stock_insert(pool: Arc<Pool<MysqlConnectionManager>>, count: usize) {
    let chunks = (0..count).collect_vec().chunks(1000).into_iter().map(|x| x.len()).collect_vec();
    chunks.into_par_iter().for_each(|chunk| {
        let vs = (0..chunk)
            .flat_map(|_| {
                let id = Uuid::new_v4().to_string();
                let name: Vec<String> = Words(1..2).fake();
                let code: usize = (10000000..100000000).fake();
                let price: usize = (1000..100000).fake();
                let count: usize = (0..5000).fake();
                vec![id, name[0].to_string(), code.to_string(), price.to_string(), count.to_string()]
            })
            .collect_vec();
        let ps = (0..chunk).map(|_| "( ?, ?, ?, ?, ? )").join(", ");

        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        let _ = conn.prep_exec(format!("insert into stock ( id, name, code, price, count ) values {ps}"), vs);
    });
}

pub fn stock_update(pool: Arc<Pool<MysqlConnectionManager>>, ids: Vec<&String>) {
    ids.chunks(1000).collect_vec().into_par_iter().for_each(|chunk| {
        let vs = chunk
            .iter()
            .flat_map(|id| {
                let count: usize = (0..5000).fake();
                vec![id.to_string(), count.to_string()]
            })
            .collect_vec();
        let ps = (0..chunk.len()).map(|_| "( ?, ? )").join(", ");

        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        let _ = conn.prep_exec(
            format!("insert into stock ( id, count ) values {ps} on duplicate key update id = values ( id ), count = values ( count )"),
            vs,
        );
    });
}

pub fn stock_drop(conn: &mut Conn) {
    conn.prep_exec("drop table if exists stock", ()).unwrap();
}

pub fn stock_ids(conn: &mut Conn) -> Vec<String> {
    match conn.query("select id from stock").map(|result| result.map(|x| x.unwrap()).map(from_row::<String>).collect()) {
        Ok(x) => x,
        Err(_) => vec![],
    }
}
