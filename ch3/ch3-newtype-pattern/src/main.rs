// Hostname がニュータイプ
struct Hostname(String);

// 型システムによって、不正な使い方から保護される
fn connect(host: Hostname) {
    // 数値インデックスを使って、データにアクセスする
    println!("connected to {}", host.0);
}

fn main() {
    // 普通の文字列をつくる
    let ordinary_string = String::from("localhost");
    // Hostnameを期待している関数に普通の文字列を渡す
    let host = Hostname(ordinary_string.clone());

    connect(host);
}
