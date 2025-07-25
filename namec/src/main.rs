use std::{env, path::{PathBuf, Path}};
use arboard::Clipboard;

fn main() {
        let args: Vec<String> = env::args().collect();

            if args.len() != 2 {
                        eprintln!("用法: namec <路径>");
                                std::process::exit(1);
                                    }

                                        let input_path = PathBuf::from(&args[1]);

                                            // 如果路径不存在，我们也允许 "." 这种合法路径通过 canonicalize
                                                let resolved_path = match input_path.canonicalize() {
                                                            Ok(p) => p,
                                                                    Err(_) => {
                                                                                    eprintln!("错误: 路径无法解析或不存在");
                                                                                                std::process::exit(1);
                                                                                                        }
                                                                                                            };

                                                                                                                let name = match resolved_path.file_name().or_else(|| {
                                                                                                                            // 处理根路径或路径末尾是 ".." 等情况，尝试获取上级路径的名称
                                                                                                                                    resolved_path.parent().and_then(|p| p.file_name())
                                                                                                                                        }) {
                                                                                                                            Some(name) => name.to_string_lossy().to_string(),
                                                                                                                                    None => {
                                                                                                                                                    eprintln!("无法提取目录或文件名");
                                                                                                                                                                std::process::exit(1);
                                                                                                                                                                        }
                                                                                                                                                                            };

                                                                                                                                                                                let mut clipboard = Clipboard::new().expect("无法访问剪贴板");
                                                                                                                                                                                    clipboard.set_text(&name).expect("复制失败");

                                                                                                                                                                                        println!("已复制名称到剪贴板：{}", name);
}

