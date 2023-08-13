use std::cmp::min;
use std::sync::Arc;

use itertools::Itertools;
use mysql::{Conn, Opts, OptsBuilder};
use r2d2::{ManageConnection, Pool};
use r2d2_mysql::MysqlConnectionManager;
use rand::prelude::SliceRandom;

use crate::sample2::ship::{ship_create, ship_drop, ship_ids, ship_insert, ship_update};
use crate::sample2::stock::{stock_create, stock_drop, stock_ids, stock_insert, stock_update};

mod ship;
mod stock;

pub fn init() {
    let mut conn = create_connection();
    stock_drop(&mut conn);
    ship_drop(&mut conn);
    stock_create(&mut conn);
    ship_create(&mut conn);
}

pub fn warehousing(count: usize) {
    stock_insert(create_pool(), count);
}

pub fn count_up(count: usize) {
    let mut conn = create_connection();
    let mut stock_ids = stock_ids(&mut conn);
    if !stock_ids.is_empty() {
        let mut rng = rand::thread_rng();
        stock_ids.shuffle(&mut rng);
        let n = min(count, stock_ids.len());
        stock_update(create_pool(), stock_ids.iter().take(n).collect_vec());
    }
}

pub fn ship(count: usize) {
    ship_insert(create_pool(), count);
}

pub fn arrive(count: usize) {
    let mut conn = create_connection();
    let mut ship_ids = ship_ids(&mut conn);
    if !ship_ids.is_empty() {
        let mut rng = rand::thread_rng();
        ship_ids.shuffle(&mut rng);
        let n = min(count, ship_ids.len());
        ship_update(create_pool(), ship_ids.into_iter().take(n).collect_vec());
    }
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

fn create_connection() -> Conn {
    let opt = Opts::from_url(get_url()).unwrap();
    let builder = OptsBuilder::from_opts(opt);
    let manager = MysqlConnectionManager::new(builder);
    match manager.connect() {
        Ok(x) => x,
        Err(_) => panic!("データベースに接続できませんでした、コンテナが起動しているか確認してください"),
    }
}

fn get_url() -> &'static str {
    "mysql://user:password@localhost:23306/sample2"
}
