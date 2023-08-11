use fake::Fake;
use mysql::{Conn, Opts, OptsBuilder};
use r2d2::ManageConnection;
use r2d2_mysql::MysqlConnectionManager;

use crate::sample1::log::{log_create, log_drop, log_insert};
use crate::sample1::message::{message_create, message_delete, message_drop, message_insert};
use crate::sample1::token::{token_create, token_delete, token_drop, token_ids, token_insert};
use crate::sample1::user::{user_create, user_delete, user_drop, user_ids, user_insert, user_update};

mod log;
mod message;
mod token;
mod user;

pub fn init() {
    let mut conn = create_connection();
    user_drop(&mut conn);
    message_drop(&mut conn);
    token_drop(&mut conn);
    log_drop(&mut conn);
    user_create(&mut conn);
    message_create(&mut conn);
    token_create(&mut conn);
    log_create(&mut conn);
}

pub fn signup() {
    let mut conn = create_connection();
    user_insert(&mut conn);
    log_insert(&mut conn, "insert: user");
}

pub fn edit_profile() {
    let mut conn = create_connection();
    let user_ids = user_ids(&mut conn, false);
    if !user_ids.is_empty() {
        let n: usize = (0..user_ids.len()).fake();
        user_update(&mut conn, user_ids[n]);
        log_insert(&mut conn, "update: user");
    }
}

pub fn withdraw() {
    let mut conn = create_connection();
    let user_ids = user_ids(&mut conn, false);
    if !user_ids.is_empty() {
        let n: usize = (0..user_ids.len()).fake();
        user_delete(&mut conn, user_ids[n]);
        message_delete(&mut conn, user_ids[n]);
        token_delete(&mut conn, user_ids[n]);
        log_insert(&mut conn, "delete: user");
        log_insert(&mut conn, "delete: message");
        log_insert(&mut conn, "delete: token");
    }
}

pub fn generate_token() {
    let mut conn = create_connection();
    let user_ids = token_ids(&mut conn, false);
    if !user_ids.is_empty() {
        let n: usize = (0..user_ids.len()).fake();
        token_insert(&mut conn, user_ids[n]);
        log_insert(&mut conn, "insert: token");
    }
}

pub fn delete_token() {
    let mut conn = create_connection();
    let user_ids = token_ids(&mut conn, true);
    if !user_ids.is_empty() {
        let n: usize = (0..user_ids.len()).fake();
        token_delete(&mut conn, user_ids[n]);
        log_insert(&mut conn, "delete: token");
    }
}

pub fn chat() {
    let mut conn = create_connection();
    let user_ids = user_ids(&mut conn, false);
    if !user_ids.is_empty() {
        let n: usize = (0..user_ids.len()).fake();
        message_insert(&mut conn, user_ids[n]);
        log_insert(&mut conn, "insert: message");
    }
}

fn create_connection() -> Conn {
    let url = "mysql://user:password@localhost:13306/sample1";
    let opt = Opts::from_url(url).unwrap();
    let builder = OptsBuilder::from_opts(opt);
    let manager = MysqlConnectionManager::new(builder);
    match manager.connect() {
        Ok(x) => x,
        Err(_) => panic!("データベースに接続できませんでした、コンテナが起動しているか確認してください"),
    }
}
