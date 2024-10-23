- Rust 编写命令行工具 使用 clap 这个库
- 读取 csv 使用 csv 这个库
- 序列化和反序列化使用 serde 反序列化是将标准的传输格式如 yaml json xml csv 反序列化成特定的 rust 数据结构如   struct 序列化是将 rust 特定的数据结构 转成 json csv ...

- anyhow::bail!("invalid output format") 相当于 return Err("Invalid output format")
- Into::<&str>::into(*self)：
  *self：解引用 self，得到 OutputFormat 的值。
  Into::<&str>::into(*self)：将 OutputFormat 转换为 &str。这是通过之前实现的 From<OutputFormat> for &'static str 完成的。
