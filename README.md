# clip_utils

`clip_utils` 是一组用 Rust 编写的轻量级跨平台命令行工具，专注于将路径信息便捷地复制到剪贴板，极大提升开发者在终端下的效率。

## 工具列表

- **pwdc**  
  复制当前工作目录的绝对路径到剪贴板。  
  _场景：快速获取当前目录路径，便于粘贴到其他应用。_

- **realpathc <路径>**  
  复制指定文件或目录的绝对路径到剪贴板。  
  _场景：获得任意路径的标准化绝对路径，避免手动转换。_

- **namec <路径>**  
  复制指定路径的文件名或目录名到剪贴板。  
  _场景：快速获取文件或目录的名称，便于粘贴到文档、命令行或其他应用。_

## 特性

- 🚀 高效：Rust 编写，启动快、性能优
- 🖥️ 跨平台：支持 macOS、Linux、Windows（需相应编译）
- 📋 剪贴板集成：基于 [`arboard`](https://crates.io/crates/arboard) 库，兼容主流平台
- 🛠️ 易用：命令行接口简洁，零学习成本，自动反馈复制结果
- 🔗 易集成：适合嵌入脚本或开发工具链

## 安装

1. **克隆并编译：**
   ```bash
   git clone https://github.com/yourname/clip_utils.git
   cd clip_utils
   cargo build --release
   ```

2. **运行示例：**
   ```bash
   ./target/release/pwdc
   ./target/release/realpathc /path/to/file_or_dir
   ./target/release/namec /path/to/file_or_dir
   ```

3. **（可选）加入 PATH：**
   ```bash
   ln -s $(pwd)/target/release/pwdc /usr/local/bin/pwdc
   ln -s $(pwd)/target/release/realpathc /usr/local/bin/realpathc
   ln -s $(pwd)/target/release/namec /usr/local/bin/namec
   ```

## 依赖

- [Rust](https://www.rust-lang.org/) 1.60+
- [arboard](https://crates.io/crates/arboard) 剪贴板库

## 平台支持

- macOS
- Linux
- Windows（需测试）

## 贡献

欢迎 issue、PR 或建议！如有贡献指南请先阅读 [CONTRIBUTING](CONTRIBUTING.md)。

## License

MIT
