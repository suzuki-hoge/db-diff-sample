use std::io;

use dialoguer::Select;

mod sample1;

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
                    "プログラム終了" => break,
                    _ => unreachable!(),
                }
            }
        }
        "サンプル２ ( バッチ処理 )" => {
            //
        }
        _ => unreachable!(),
    }

    Ok(())
}
