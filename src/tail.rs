use cursor_common as common;

use std::env;
use std::process;
use std::fs;
use std::io::{self, Read};

const ORIGINAL_TAIL: &str = "/usr/bin/tail";
const MIN_LINES: i32 = 100;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    // 如果不在 Cursor 环境中，直接调用原始命令
    if !common::is_in_cursor() {
        let exit_code = common::execute_original_command(ORIGINAL_TAIL, &args)
            .unwrap_or(1);
        process::exit(exit_code);
    }
    
    // 在 Cursor 环境中，应用强制最佳实践
    enforce_tail_best_practices(args);
}

fn enforce_tail_best_practices(args: Vec<String>) {
    // 检查是否有管道输入
    let has_pipe_input = !atty::is(atty::Stream::Stdin);
    
    if has_pipe_input {
        // 保存管道输入到临时文件（使用统一接口）
        match common::save_stdin_to_file("tail_input") {
            Ok(Some(tmp_file)) => {
                // 解析并调整参数
                let adjusted_args = adjust_line_count(args);
                
                let mut final_args = adjusted_args;
                final_args.push(tmp_file);
                
                let exit_code = common::execute_original_command(ORIGINAL_TAIL, &final_args)
                    .unwrap_or(1);
                process::exit(exit_code);
            }
            Ok(None) => {
                // 没有输入，直接执行
                let adjusted_args = adjust_line_count(args);
                let exit_code = common::execute_original_command(ORIGINAL_TAIL, &adjusted_args)
                    .unwrap_or(1);
                process::exit(exit_code);
            }
            Err(e) => {
                eprintln!("❌ 无法保存管道输入: {}", e);
                process::exit(1);
            }
        }
    } else {
        // 没有管道输入，直接调整参数
        let adjusted_args = adjust_line_count(args);
        
        let exit_code = common::execute_original_command(ORIGINAL_TAIL, &adjusted_args)
            .unwrap_or(1);
        process::exit(exit_code);
    }
}

fn adjust_line_count(args: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut found_n_flag = false;
    
    while i < args.len() {
        let arg = &args[i];
        
        if arg == "-n" {
            // -n 后面跟数字
            result.push(arg.clone());
            found_n_flag = true;
            
            if i + 1 < args.len() {
                i += 1;
                if let Ok(num) = args[i].parse::<i32>() {
                    if num < MIN_LINES {
                        common::print_enforcement_message(
                            &format!("tail -n {}", num), 
                            &format!("tail -n {}", MIN_LINES)
                        );
                        result.push(MIN_LINES.to_string());
                    } else {
                        result.push(args[i].clone());
                    }
                } else {
                    result.push(args[i].clone());
                }
            }
        } else if arg.starts_with("-") && arg.len() > 1 {
            // 处理 -5 这种格式
            if let Ok(num) = arg[1..].parse::<i32>() {
                if num < MIN_LINES {
                    common::print_enforcement_message(
                        &format!("tail {}", arg), 
                        &format!("tail -{}", MIN_LINES)
                    );
                    result.push(format!("-{}", MIN_LINES));
                } else {
                    result.push(arg.clone());
                }
                found_n_flag = true;
            } else {
                result.push(arg.clone());
            }
        } else {
            result.push(arg.clone());
        }
        
        i += 1;
    }
    
    // 如果没有指定行数，添加 -n 100
    if !found_n_flag && !result.iter().any(|a| !a.starts_with("-")) {
        common::print_enforcement_message(
            "tail",
            &format!("tail -n {}", MIN_LINES)
        );
        result.insert(0, MIN_LINES.to_string());
        result.insert(0, "-n".to_string());
    }
    
    result
}


