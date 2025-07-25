use std::env;
use arboard::Clipboard;

fn main() {
    match env::current_dir() {
        Ok(path) => {
            let path_str = path.display().to_string();
            let mut clipboard = Clipboard::new().expect("无法访问剪贴板");
            clipboard.set_text(path_str.clone()).expect("复制失败");
            println!("已复制当前目录到剪贴板：{}", path_str);
        }
        Err(e) => eprintln!("获取当前目录失败：{}", e),
    }
}
