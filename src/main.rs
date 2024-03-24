use std::io::Write;
use std::process::{Command, Stdio};
use color_eyre::eyre::eyre;
use crate::transpiler::{ChatGPT, Transpiler};

mod transpiler;
mod llm;

#[tokio::main]
async fn main() {
    match run().await {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        _ => {}
    }
}

async fn run() -> color_eyre::Result<()> {
    let gpt = ChatGPT::init()?;
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        return Err(eyre!("usage: {} <source>", args[0]));
    }
    let source_path = &args[1];

    let source_code = std::fs::read_to_string(source_path)?;
    let resp = gpt.transpile(&source_code).await?;
    println!("transpiled Rust code: ====\n {} \n====\n", resp);
    Ok(())
}


fn compile_rust_code(code: &str) -> String {
    // 调用 rustc 编译代码
    let mut rustc = Command::new("rustc")
        .args(&[ "-"]) // 可能需要根据 rustc 版本进行调整
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("无法执行编译");

    // 获取 stdin 的可变引用
    if let Some(mut stdin) = rustc.stdin.as_mut() {
        // 向编译器传递代码
        stdin.write_all(code.as_bytes()).expect("无法写入 stdin");
    } else {
        panic!("无法获取 stdin");
    }

    // 获取编译器的输出（错误信息）
    let error_output = rustc.wait_with_output().expect("无法获取编译器输出");
    String::from_utf8_lossy(&error_output.stderr).to_string()
}

#[test]
fn test_compile_with_error() {
    let code_with_error = r#"
        fn main() {
            let x: u32 = "hello";
        }
    "#;
    let error_message = compile_rust_code(code_with_error);
    println!("error_message: {}", error_message);
    assert!(error_message.contains("expected `u32`, found `&str`"));
}

#[test]
fn test_compile_successfully() {
    let code_without_error = r#"
        fn main() {
            println!("Hello, world!");
        }
    "#;
    let error_message = compile_rust_code(code_without_error);
    println!("error_message: {}", error_message);
    assert!(error_message.is_empty());
}