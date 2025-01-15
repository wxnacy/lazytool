use lazytool::time;

fn main() {
    let dt = time::from_str("2025-01-15 18:16:13", "%Y-%m-%d %H:%M:%S").unwrap();
    println!("{dt:?}");

    let dt = time::from_str_with_timezone("2025-01-15 18:16:13", "%Y-%m-%d %H:%M:%S", "Asia/Shanghai").unwrap();
    println!("{dt:?}");

    let ts = time::to_timestamp("2025-01-15 18:16:13", "%Y-%m-%d %H:%M:%S").unwrap();
    println!("{ts}");

    let ts = time::current_timestamp();
    println!("{ts}");

}
