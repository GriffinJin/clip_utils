# clip_utils

`clip_utils` 是一个用 Rust 开发的轻量级跨平台命令行工具集合，专注于将路径信息复制到剪贴板，极大提升开发者在终端下的工作效率。

## 目前包含的命令

- **pwdc**
   复制当前工作目录的绝对路径到剪贴板。
   使用场景：快速获取当前目录路径，方便粘贴到其他地方。
- **realpathc <路径>**
   复制指定文件或目录的绝对路径到剪贴板。
   使用场景：快速获得任意路径的标准化绝对路径，避免手动转换。

## 特性

- 使用 Rust 编写，运行效率高，跨平台支持（需针对目标平台编译）
- 依赖精简，主要使用 `arboard` 库实现剪贴板操作
- 简洁的命令行接口，零学习成本，命令执行后自动反馈复制成功的信息
- 方便集成到日常开发脚本或工具链中

## 安装与使用

1. 克隆项目并编译：

   ```bash
   git clone https://github.com/yourname/clip_utils.git
   cd clip_utils
   cargo build --release
   ```

2. 运行示例：

   ```bash
   ./target/release/pwdc
   ./target/release/realpathc /path/to/file_or_dir
   ```

3. （可选）将命令加入 PATH：

   ```bash
   ln -s $(pwd)/target/release/pwdc /usr/local/bin/pwdc
   ln -s $(pwd)/target/release/realpathc /usr/local/bin/realpathc
   ```
