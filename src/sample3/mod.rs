use std::sync::Arc;

use itertools::Itertools;
use mysql::{Opts, OptsBuilder};
use r2d2::ManageConnection;
use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;
use rayon::prelude::*;

pub fn init(tables: usize, cols: usize, lines: usize) {
    let pool = create_pool();

    (0..tables).into_par_iter().for_each(|n| {
        exec(&pool, n, cols, lines);
    });
}

fn exec(pool: &Arc<Pool<MysqlConnectionManager>>, n: usize, cols: usize, lines: usize) {
    let table_name = format!("table_{n:0>2}");
    let col_defs = (0..cols).into_iter().map(|i| format!("col_{i:0>2} char(64)")).join(", ");
    let col_names = (0..cols).into_iter().map(|i| format!("col_{i:0>2}")).join(", ");
    let record = (0..cols).into_iter().join(", ");
    let records = (0..lines).into_iter().map(|_| format!("( {} )", &record)).join(", ");

    let pool = pool.clone();
    let mut conn = pool.get().unwrap();

    conn.prep_exec(format!("drop table if exists {table_name}"), ()).unwrap();
    conn.prep_exec(format!("create table {table_name} ( id int auto_increment, {col_defs}, updated_at datetime default current_timestamp, primary key (id) )"), ()).unwrap();
    conn.prep_exec(format!("insert into {table_name} ( {col_names} ) values {records}"), ()).unwrap();
}

pub fn info() {
    println!("{}", get_url());
}

fn create_pool() -> Arc<Pool<MysqlConnectionManager>> {
    let opt = Opts::from_url(get_url()).unwrap();
    let builder = OptsBuilder::from_opts(opt);
    let manager = MysqlConnectionManager::new(builder);
    let _ = match manager.connect() {
        Ok(x) => x,
        Err(_) => panic!("データベースに接続できませんでした、コンテナが起動しているか確認してください"),
    };
    Arc::new(Pool::builder().build(manager).unwrap())
}

fn get_url() -> &'static str {
    "mysql://user:password@localhost:33306/sample3"
}
