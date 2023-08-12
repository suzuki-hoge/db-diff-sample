use std::io;

use dialoguer::Select;

mod sample1;
mod sample2;

fn main() -> io::Result<()> {
    let samples = vec!["サンプル１ ( ユーザ管理 )", "サンプル２ ( バッチ処理 )"];

    match samples[Select::new().items(&samples).interact()?] {
        "サンプル１ ( ユーザ管理 )" => {
            let actions = vec![
                "初期化",
                "ユーザ作成",
                "プロフィール変更",
                "ユーザ退会",
                "メッセージ送信",
                "API トークン生成",
                "API トークン無効化",
                "接続情報確認",
                "プログラム終了",
            ];

            loop {
                match actions[Select::new().items(&actions).interact()?] {
                    "初期化" => sample1::init(),
                    "ユーザ作成" => sample1::signup(),
                    "プロフィール変更" => sample1::edit_profile(),
                    "ユーザ退会" => sample1::withdraw(),
                    "メッセージ送信" => sample1::chat(),
                    "API トークン生成" => sample1::generate_token(),
                    "API トークン無効化" => sample1::delete_token(),
                    "接続情報確認" => sample1::info(),
                    "プログラム終了" => break,
                    _ => unreachable!(),
                }
            }
        }
        "サンプル２ ( バッチ処理 )" => {
            let n = 1000;

            let actions = vec![
                "初期化",
                "入庫 ( 1000 件 )",
                "入庫 ( 10000 件 )",
                "入庫 ( 50000 件 )",
                "在庫数更新 ( 1000 件 )",
                "在庫数更新 ( 10000 件 )",
                "在庫数更新 ( 50000 件 )",
                "発送 ( 1000 件 )",
                "発送 ( 10000 件 )",
                "発送 ( 50000 件 )",
                "着荷 ( 1000 件 )",
                "着荷 ( 10000 件 )",
                "着荷 ( 50000 件 )",
                "接続情報確認",
                "プログラム終了",
            ];

            loop {
                match actions[Select::new().items(&actions).interact()?] {
                    "初期化" => sample2::init(),
                    "入庫 ( 1000 件 )" => sample2::warehousing(n),
                    "入庫 ( 10000 件 )" => sample2::warehousing(n * 10),
                    "入庫 ( 50000 件 )" => {
                        sample2::warehousing(n * 10);
                        sample2::warehousing(n * 10);
                        sample2::warehousing(n * 10);
                        sample2::warehousing(n * 10);
                        sample2::warehousing(n * 10);
                    }
                    "在庫数更新 ( 1000 件 )" => sample2::count_up(n),
                    "在庫数更新 ( 10000 件 )" => sample2::count_up(n * 10),
                    "在庫数更新 ( 50000 件 )" => {
                        sample2::count_up(n * 10);
                        sample2::count_up(n * 10);
                        sample2::count_up(n * 10);
                        sample2::count_up(n * 10);
                        sample2::count_up(n * 10);
                    }
                    "発送 ( 1000 件 )" => sample2::ship(n),
                    "発送 ( 10000 件 )" => {
                        sample2::ship(n * 5);
                        sample2::ship(n * 5);
                    }
                    "発送 ( 50000 件 )" => {
                        sample2::ship(n * 5);
                        sample2::ship(n * 5);
                        sample2::ship(n * 5);
                        sample2::ship(n * 5);
                        sample2::ship(n * 5);
                        sample2::ship(n * 5);
                        sample2::ship(n * 5);
                        sample2::ship(n * 5);
                        sample2::ship(n * 5);
                        sample2::ship(n * 5);
                    }
                    "着荷 ( 1000 件 )" => sample2::arrive(n),
                    "着荷 ( 10000 件 )" => sample2::arrive(n * 10),
                    "着荷 ( 50000 件 )" => {
                        sample2::arrive(n * 10);
                        sample2::arrive(n * 10);
                        sample2::arrive(n * 10);
                        sample2::arrive(n * 10);
                        sample2::arrive(n * 10);
                    }
                    "接続情報確認" => sample2::info(),
                    "プログラム終了" => break,
                    _ => unreachable!(),
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
