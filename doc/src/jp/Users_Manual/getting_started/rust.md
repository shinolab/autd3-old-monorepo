# Rust版チュートリアル

まずは適当なプロジェクトを作成し, `autd3`ライブラリを依存関係に追加する.
また, デバイスとの通信を行う`autd3-link-soem`ライブラリも依存関係に追加する.

```shell
cargo new --bin autd3-sample
cd autd3-sample
cargo add autd3
cargo add autd3-link-soem
cargo add tokio --features full
```

次に, `src/main.rs`ファイルを編集し, 以下のようにする.
これは単一焦点に$\SI{150}{Hz}$のAM変調をかける場合のソースコードである.

```rust,should_panic,filename=main.rs,edition2021
# extern crate autd3;
# extern crate tokio;
# extern crate autd3_link_soem;
use autd3::prelude::*;
use autd3_link_soem::SOEM;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // AUTDを操作するControllerの作成
    let mut autd = Controller::builder()
        // 接続しているデバイス情報の登録
        // AUTD3::newの引数は位置
        // 位置は自分の設定した座標系におけるこのデバイスの位置を指定する
        // ここでは, デバイスは原点に置かれるする
        .add_device(AUTD3::new(Vector3::zeros()))
        // SOEMリンクを使用してControllerをopenする
        // with_on_lostで指定したコールバックはSOEMがデバイスをロストしたときに呼ばれる 
        .open_with(SOEM::builder().with_on_lost(|msg| {
            eprintln!("Unrecoverable error occurred: {msg}");
            std::process::exit(-1);
        })).await?;

    // ファームウェアバージョンのチェック
    // ここで, v4.0.x以外が表示される場合の動作は保証しない
    autd.firmware_infos().await?.iter().for_each(|firm_info| {
        println!("{}", firm_info);
    });

    // 静音化処理を有効化
    // なお, デフォルトで有効にされているので, 実際には必要ない
    // 無効にしたい場合はSilencer::disable()を送信する
    autd.send(Silencer::default()).await?;

    // デバイスの中心から直上150mmに焦点
    let center = autd.geometry.center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
    let g = Focus::new(center);

    // 150Hzサイン波変調
    let m = Sine::new(150.);

    // データの送信
    autd.send((m, g)).await?;

    println!("press enter to quit...");
    let mut _s = String::new();
    std::io::stdin().read_line(&mut _s)?;

    // コントローラーを閉じる
    autd.close().await?;

    Ok(())
}
```

そして, これを実行する.

```shell
cargo run --release
```

## Linux,macOS使用時の注意

Linux, macOSでは, SOEMを使用するのに管理者権限が必要な場合がある.
その場合は, 
```shell
cargo build --release && sudo ./target/release/autd3_sample
```
とすること.
