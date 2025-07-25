use std::{env, fs, path::PathBuf};
use arboard::Clipboard;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("用法: realpathc <路径>");
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);

    if !input_path.exists() {
        eprintln!("错误: 路径不存在");
        std::process::exit(1);
    }

    match fs::canonicalize(&input_path) {
        Ok(absolute_path) => {
            let path_str = absolute_path.display().to_string();
            let mut clipboard = Clipboard::new().expect("无法访问剪贴板");
            clipboard.set_text(&path_str).expect("复制失败");
            println!("已复制绝对路径到剪贴板：{}", path_str);
        }
        Err(e) => {
            eprintln!("无法获取绝对路径：{}", e);
        }
    }
}
