# use clap to create a command library
- Rust 编写命令行工具 使用 clap 这个库
- 读取 csv 使用 csv 这个库
- 序列化和反序列化使用 serde 反序列化是将标准的传输格式如 yaml json xml csv 反序列化成特定的 rust 数据结构如   struct 序列化是将 rust 特定的数据结构 转成 json csv ...

# convert csv to json or yaml
- anyhow::bail!("invalid output format") 相当于 return Err("Invalid output format")
- Into::<&str>::into(*self)：
  *self：解引用 self，得到 OutputFormat 的值。
  Into::<&str>::into(*self)：将 OutputFormat 转换为 &str。这是通过之前实现的 From<OutputFormat> for &'static str 完成的。

# generate a random password
- extend_from_slice() 拼接 vec  和 js 的 concat 类似
- rand 中的 choose 方法 从 切片或者数组选个来随机
- String::from_utf8（） 把 Vec<u8> 转成 String
- zxcbvn 验证密码强度的库

# support base64 encode/decode cli
- base64 中的 padding 指的是位数不够进行字节的填充= = STANDARD(位数不够需要填充) URL_SAFE_NO_PAD（位数不够不需要填充  详细见 base64 crate

- cargo run -- base64 encode --format urlsafe -i cargo.toml > fixtures/b64.txt 输出编码文件到 fixtures/b64.txt

- impl From<T> for U {}  U 转化成 T 就可以使用 U.into()
- impl FromStr for T  input: &str的数据类型转换成 T 就可以使用  input::parse()

# blake3  Ed25519-dealk ChaCha20Poly1305
- blake3  hash function
- Ed25519-dealk 非对称加密
- ChaCha20Poly1305  对称加密
