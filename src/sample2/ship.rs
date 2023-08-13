use std::sync::Arc;

use fake::faker::address::en::{BuildingNumber, CityName, PostCode, StateName, StreetName};
use fake::faker::company::en::CompanyName;
use fake::faker::phone_number::en::CellNumber;
use fake::Fake;
use itertools::Itertools;
use mysql::{from_row, Conn};
use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;
use rayon::prelude::*;

pub fn ship_create(conn: &mut Conn) {
    conn.prep_exec("create table ship ( id int auto_increment, code char(8), status enum ('shipping', 'arrived'), company char(64), post char(32), building char(64), street char(64), city char(64), state char(64), phone char(32), shipped_at datetime default current_timestamp, arrived_at datetime default null, primary key (id) )", ()).unwrap();
}

pub fn ship_insert(pool: Arc<Pool<MysqlConnectionManager>>, count: usize) {
    let chunks = (0..count).collect_vec().chunks(5000).into_iter().map(|x| x.len()).collect_vec();
    chunks.into_par_iter().for_each(|chunk| {
        let vs = (0..chunk)
            .flat_map(|_| {
                let code: usize = (10000000..100000000).fake();
                let company: String = CompanyName().fake();
                let post: String = PostCode().fake();
                let building: String = BuildingNumber().fake();
                let street: String = StreetName().fake();
                let city: String = CityName().fake();
                let state: String = StateName().fake();
                let phone: String = CellNumber().fake();
                vec![code.to_string(), "shipping".to_string(), company, post, building, street, city, state, phone]
            })
            .collect_vec();
        let ps = (0..chunk).map(|_| "( ?, ?, ?, ?, ?, ?, ?, ?, ? )").join(", ");

        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        let _ = conn.prep_exec(format!("insert into ship ( code, status, company, post, building, street, city, state, phone ) values {ps}"), vs);
    });
}

pub fn ship_update(pool: Arc<Pool<MysqlConnectionManager>>, ids: Vec<usize>) {
    ids.chunks(5000).collect_vec().into_par_iter().for_each(|chunk| {
        let vs = chunk.iter().flat_map(|id| vec![id.to_string(), "arrived".to_string()]).collect_vec();
        let ps = (0..chunk.len()).map(|_| "( ?, ?, now() )").join(", ");

        let pool = pool.clone();
        let mut conn = pool.get().unwrap();
        let _ = conn.prep_exec(format!("insert into ship ( id, status, arrived_at ) values {ps} on duplicate key update id = values ( id ), status = values ( status ), arrived_at = values ( arrived_at )"), vs);
    });
}

pub fn ship_drop(conn: &mut Conn) {
    conn.prep_exec("drop table if exists ship", ()).unwrap();
}

pub fn ship_ids(conn: &mut Conn) -> Vec<usize> {
    match conn.query("select id from ship where status = 'shipping'").map(|result| result.map(|x| x.unwrap()).map(from_row::<usize>).collect()) {
        Ok(x) => x,
        Err(_) => vec![],
    }
}
