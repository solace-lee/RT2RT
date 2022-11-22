use serde::{Deserialize, Serialize};
use std::{fs::File, io::ErrorKind, process};

/// 格式化成json并保存到本地
pub fn output<T>(data: &T, filename: &str)
where
    T: for<'a> Deserialize<'a> + Serialize,
{
    let writer = File::create(filename);
    let file_result = match writer {
        Ok(v) => v,
        Err(e) => match e.kind() {
            ErrorKind::OutOfMemory => process::exit(1),
            _ => process::exit(2),
        },
    };
    serde_json::to_writer_pretty(file_result, &data).unwrap_or_else(|err| {
        eprintln!("序列化及写入时出现错误：{}", err);
        process::exit(3);
    });

    println!("生成的闭合轮廓已经写入了:{}", filename)
}
