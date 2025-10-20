use std::env;
use std::fs;
use std::process::{Command, Stdio};
use std::io::{self, Write, Read};
use std::path::Path;

/// 检测当前进程是否在 Cursor AI 执行环境中
/// 只在 Cursor AI 执行命令时返回 true，用户手动执行返回 false
pub fn is_in_cursor() -> bool {
    // 关键检测：CURSOR_AGENT 环境变量
    // 经过验证：只有这个变量能准确区分 AI 执行和用户手动执行
    // - CURSOR_AGENT=1 → 只有 AI 执行时才有
    // - CURSOR_TRACE_ID → 手动终端也有，不能用来区分
    if let Ok(agent) = env::var("CURSOR_AGENT") {
        if agent == "1" {
            return true;
        }
    }
    
    // 用户手动在 Cursor 终端执行时，CURSOR_AGENT 不存在
    false
}


/// 生成唯一的临时文件名（确保不会覆盖现有文件）
fn generate_unique_tmpfile(tmp_dir: &str, prefix: &str) -> String {
    use chrono::Local;
    use rand::Rng;
    
    loop {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let random: u32 = rand::thread_rng().gen();
        let tmp_file = format!("{}/{}_{}_{}_{:08x}.txt", tmp_dir, prefix, std::process::id(), timestamp, random);
        
        // 检查文件是否已存在
        if !Path::new(&tmp_file).exists() {
            return tmp_file;
        }
        // 如果存在，继续循环生成新的文件名
    }
}

/// 保存管道输入到临时文件（统一接口）
/// 返回：Ok(Some(文件路径)) 如果保存成功
///       Ok(None) 如果没有输入
pub fn save_stdin_to_file(prefix: &str) -> io::Result<Option<String>> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = Vec::new();
    
    match handle.read_to_end(&mut buffer) {
        Ok(0) => Ok(None),  // 没有输入
        Ok(_) => {
            let tmp_dir = "/tmp/cursor_outputs";
            fs::create_dir_all(tmp_dir)?;
            
            let tmp_file = generate_unique_tmpfile(tmp_dir, prefix);
            
            // 写入临时文件
            fs::write(&tmp_file, &buffer)?;
            
            // 输出提示信息
            print_file_saved_message(&tmp_file);
            
            Ok(Some(tmp_file))
        }
        Err(e) => Err(e),
    }
}

/// 打印文件保存提示信息（统一格式）
pub fn print_file_saved_message(file_path: &str) {
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("📋 [Cursor 强制最佳实践] 管道输入已自动保存");
    eprintln!("📁 文件位置: {}", file_path);
    eprintln!("💡 用途: 避免重复运行耗时命令，可直接读取此文件");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

/// 打印强制规则提示信息（统一格式）
pub fn print_enforcement_message(original_arg: &str, enforced_arg: &str) {
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("⚠️  [Cursor 强制最佳实践] {} 参数不足，已强制改为 {}", original_arg, enforced_arg);
    eprintln!("💡 这样可以提供足够的信息，避免重复运行命令");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

/// 执行原始命令
pub fn execute_original_command(original_cmd: &str, args: &[String]) -> io::Result<i32> {
    let mut cmd = Command::new(original_cmd);
    cmd.args(args);
    
    let status = cmd.status()?;
    Ok(status.code().unwrap_or(1))
}

/// 执行原始命令并传递管道输入
pub fn execute_with_stdin(original_cmd: &str, args: &[String], stdin_data: &[u8]) -> io::Result<i32> {
    let mut cmd = Command::new(original_cmd);
    cmd.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    
    let mut child = cmd.spawn()?;
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(stdin_data)?;
    }
    
    let status = child.wait()?;
    Ok(status.code().unwrap_or(1))
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_is_in_cursor_with_cursor_agent() {
        // 清理环境
        std::env::remove_var("CURSOR_AGENT");
        
        // 测试 CURSOR_AGENT=1
        std::env::set_var("CURSOR_AGENT", "1");
        assert!(is_in_cursor(), "应该检测到 CURSOR_AGENT=1");
        
        // 清理
        std::env::remove_var("CURSOR_AGENT");
    }

    #[test]
    fn test_is_not_in_cursor_manual_terminal() {
        // 清理 CURSOR_AGENT
        std::env::remove_var("CURSOR_AGENT");
        
        // 即使有 CURSOR_TRACE_ID（手动终端也有），也不应该触发
        std::env::set_var("CURSOR_TRACE_ID", "test-trace-id");
        std::env::set_var("TERM_PROGRAM", "vscode");
        assert!(!is_in_cursor(), "用户手动在 Cursor 终端执行不应该触发");
        
        // 清理
        std::env::remove_var("CURSOR_TRACE_ID");
        std::env::remove_var("TERM_PROGRAM");
    }
    
    #[test]
    fn test_cursor_agent_only_detection() {
        // 验证：只有 CURSOR_AGENT=1 才会触发
        std::env::remove_var("CURSOR_AGENT");
        
        // 有其他 Cursor 变量但没有 CURSOR_AGENT → 不触发
        std::env::set_var("CURSOR_TRACE_ID", "xxx");
        std::env::set_var("VSCODE_IPC_HOOK_CLI", "/tmp/cursor.sock");
        assert!(!is_in_cursor(), "没有 CURSOR_AGENT=1 不应该触发");
        
        // 清理
        std::env::remove_var("CURSOR_TRACE_ID");
        std::env::remove_var("VSCODE_IPC_HOOK_CLI");
    }

    #[test]
    fn test_generate_unique_tmpfile() {
        let tmp_dir = "/tmp/cursor_test";
        fs::create_dir_all(tmp_dir).unwrap();
        
        // 生成第一个文件
        let file1 = generate_unique_tmpfile(tmp_dir, "test");
        assert!(file1.contains("test_"));
        assert!(file1.ends_with(".txt"));
        
        // 生成第二个文件，应该不同
        let file2 = generate_unique_tmpfile(tmp_dir, "test");
        assert_ne!(file1, file2, "两次生成的文件名应该不同");
        
        // 创建文件，然后再次生成，应该跳过已存在的
        fs::write(&file1, "test").unwrap();
        let file3 = generate_unique_tmpfile(tmp_dir, "test");
        assert_ne!(file1, file3, "应该生成不同的文件名以避免覆盖");
        
        // 清理
        let _ = fs::remove_dir_all(tmp_dir);
    }

    #[test]
    fn test_generate_unique_tmpfile_no_overwrite() {
        let tmp_dir = "/tmp/cursor_test_overwrite";
        fs::create_dir_all(tmp_dir).unwrap();
        
        // 生成并创建100个文件
        let mut files = Vec::new();
        for _ in 0..100 {
            let file = generate_unique_tmpfile(tmp_dir, "batch");
            assert!(!Path::new(&file).exists(), "生成的文件不应该已存在");
            fs::write(&file, "test").unwrap();
            files.push(file);
        }
        
        // 验证所有文件名都不同
        let unique_count = files.iter().collect::<std::collections::HashSet<_>>().len();
        assert_eq!(unique_count, 100, "所有文件名应该都是唯一的");
        
        // 清理
        let _ = fs::remove_dir_all(tmp_dir);
    }

    #[test]
    fn test_print_file_saved_message() {
        // 这个测试只是确保函数不会 panic
        print_file_saved_message("/tmp/test.txt");
    }

    #[test]
    fn test_print_enforcement_message() {
        // 这个测试只是确保函数不会 panic
        print_enforcement_message("tail -5", "tail -100");
    }

    #[test]
    fn test_execute_original_command() {
        // 测试执行简单命令
        let args = vec!["test".to_string()];
        let result = execute_original_command("echo", &args);
        assert!(result.is_ok(), "应该能够执行 echo 命令");
        assert_eq!(result.unwrap(), 0, "echo 命令应该返回 0");
    }
}
