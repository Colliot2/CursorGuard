use cursor_common as common;

use std::env;
use std::process;
use std::fs;
use std::io::{self, BufRead};

const ORIGINAL_GREP: &str = "/usr/bin/grep";
const GREP_EXTRA_ARGS: &[&str] = &["--color=auto", "--exclude-dir={.bzr,CVS,.git,.hg,.svn,.idea,.tox,.venv,venv}"];

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    // 如果不在 Cursor 环境中，直接调用原始命令
    if !common::is_in_cursor() {
        let exit_code = common::execute_original_command(ORIGINAL_GREP, &args)
            .unwrap_or(1);
        process::exit(exit_code);
    }
    
    // 在 Cursor 环境中，应用强制最佳实践
    enforce_grep_best_practices(args);
}

fn enforce_grep_best_practices(mut args: Vec<String>) {
    // 检查是否有管道输入
    let has_pipe_input = !atty::is(atty::Stream::Stdin);
    
    if has_pipe_input {
        // 保存管道输入到临时文件（使用统一接口）
        match common::save_stdin_to_file("grep_input") {
            Ok(Some(tmp_file)) => {
                // 检查是否已有上下文参数
                let has_context = has_context_arg(&args);
                
                if !has_context {
                    common::print_enforcement_message("grep", "grep -C 20");
                    args.insert(0, "-C".to_string());
                    args.insert(1, "20".to_string());
                }
                
                // 添加额外参数
                let mut final_args = Vec::new();
                for arg in GREP_EXTRA_ARGS {
                    final_args.push(arg.to_string());
                }
                final_args.extend(args);
                final_args.push(tmp_file);
                
                let exit_code = common::execute_original_command(ORIGINAL_GREP, &final_args)
                    .unwrap_or(1);
                process::exit(exit_code);
            }
            Ok(None) => {
                // 没有输入，直接执行
                let exit_code = common::execute_original_command(ORIGINAL_GREP, &args)
                    .unwrap_or(1);
                process::exit(exit_code);
            }
            Err(e) => {
                eprintln!("❌ 无法保存管道输入: {}", e);
                process::exit(1);
            }
        }
    } else {
        // 没有管道输入，检查是否有上下文参数
        let has_context = has_context_arg(&args);
        
        if !has_context {
            common::print_enforcement_message("grep", "grep -C 20");
            args.insert(0, "-C".to_string());
            args.insert(1, "20".to_string());
        }
        
        // 添加额外参数
        let mut final_args = Vec::new();
        for arg in GREP_EXTRA_ARGS {
            final_args.push(arg.to_string());
        }
        final_args.extend(args);
        
        let exit_code = common::execute_original_command(ORIGINAL_GREP, &final_args)
            .unwrap_or(1);
        process::exit(exit_code);
    }
}

fn has_context_arg(args: &[String]) -> bool {
    for arg in args {
        if arg.starts_with("-A") || arg.starts_with("-B") || arg.starts_with("-C") ||
           arg.starts_with("--context") || arg.starts_with("--after-context") || 
           arg.starts_with("--before-context") {
            return true;
        }
    }
    false
}


