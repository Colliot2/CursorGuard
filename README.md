# Cursor 强制最佳实践包装器 - Rust 二进制版

## 🎯 目标

强制 Cursor AI 在使用 `grep`、`tail`、`head` 命令时获取足够的上下文信息，避免反复重跑命令。

## 🚀 特性

### 核心功能
- ✅ **二进制替换**：直接替换系统命令（放在 PATH 优先路径中）
- ✅ **智能检测**：自动检测是否在 Cursor 环境中
- ✅ **零影响**：普通终端不受任何影响，正常使用
- ✅ **强制规则**：
  - `grep`: 自动添加 `-C 20`（前后各 20 行上下文）
  - `tail`: 最少 100 行（-n 100）
  - `head`: 最少 100 行（-n 100）
  - 管道输入自动保存到 `/tmp/cursor_outputs/`

### 检测机制
1. **环境变量检测**
   - `TERM_PROGRAM=vscode`
   - `VSCODE_GIT_ASKPASS_MAIN` 包含 "Cursor"
   - `VSCODE_IPC_HOOK_CLI` 包含 "Cursor"

2. **进程树检测**
   - 检查父进程链（最多 10 层）
   - 查找是否包含 "Cursor" 或 "vscode" 进程

## 📦 安装

### 1. 编译（如果还没编译）
```bash
cd /Users/zhenghu/tools/cursor_grep_wrapper
source "$HOME/.cargo/env"
cargo build --release
```

### 2. 安装到系统（需要 sudo）
```bash
./install.sh
# 或手动安装：
sudo cp target/release/{grep,tail,head} /usr/local/bin/
```

### 3. 安装到用户目录（无需 sudo）
```bash
mkdir -p ~/bin
cp target/release/{grep,tail,head} ~/bin/
# 确保 ~/bin 在 PATH 中且优先级高
export PATH="$HOME/bin:$PATH"
# 添加到 ~/.zshrc 或 ~/.bashrc
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.zshrc
```

## 🧪 测试

### 在 Cursor 终端中测试
```bash
# 测试 grep - 应该看到强制提示
echo "error line 1
info line 2
error line 3" | grep "error"

# 输出：
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 📋 [Cursor 强制最佳实践] 管道输入已自动保存
# 📁 文件位置: /tmp/cursor_outputs/pipeline_input_...
# ⚠️  [Cursor 强制最佳实践] grep 未指定上下文，已自动添加 -C 20
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# 测试 tail - 应该强制改为 100 行
seq 1 20 | tail -5
# 输出全部 20 行（因为不足 100 行）

# 测试 head
seq 1 20 | head -3
# 输出全部 20 行
```

### 在普通终端中测试
```bash
# 打开系统 Terminal.app
echo "test" | grep "test"
# 正常输出，无任何提示

seq 1 10 | tail -3
# 正常输出最后 3 行：
# 8
# 9
# 10
```

## 🗑️  卸载

```bash
./uninstall.sh
# 或手动卸载：
sudo rm /usr/local/bin/{grep,tail,head}
# 或：
rm ~/bin/{grep,tail,head}
```

## 📊 工作原理

```
命令调用流程：
┌─────────────┐
│ Cursor AI   │
│ 执行命令    │
└──────┬──────┘
       │
       ↓
┌─────────────────────┐
│ ~/bin/grep          │  ← 我们的包装器（优先级高）
│ /usr/local/bin/grep │
└──────┬──────────────┘
       │
       ├─→ 检测是否在 Cursor 环境？
       │   └─→ 是：应用强制规则
       │   └─→ 否：直接调用原始命令
       │
       ↓
┌─────────────────────┐
│ /usr/bin/grep       │  ← 原始系统命令
└─────────────────────┘
```

## 📂 目录结构

```
cursor_grep_wrapper/
├── Cargo.toml          # 项目配置
├── src/
│   ├── common.rs       # 共享代码（Cursor 检测逻辑）
│   ├── grep.rs         # grep 包装器
│   ├── tail.rs         # tail 包装器
│   └── head.rs         # head 包装器
├── target/release/     # 编译产物
│   ├── grep            # 614KB 二进制
│   ├── tail            # 613KB 二进制
│   └── head            # 613KB 二进制
├── install.sh          # 安装脚本
├── uninstall.sh        # 卸载脚本
└── README.md           # 本文档
```

## 💡 优势

### vs Shell 脚本方案
1. **性能更好**：二进制启动更快，无需解释器
2. **更可靠**：不受 shell 配置影响
3. **更强硬**：直接二进制替换，无法绕过
4. **跨 Shell**：bash、zsh、fish 全部支持

### vs Alias/Function 方案
1. **无法绕过**：Alias 可以用 `\grep` 绕过，二进制不行
2. **不依赖配置**：不需要修改 `.bashrc` 或 `.zshrc`
3. **立即生效**：安装后所有新终端立即生效

## 🎓 技术细节

### 编译选项
- **Release 模式**：`cargo build --release` 优化性能
- **静态链接**：包含所有依赖，无需运行时依赖
- **跨平台**：Rust 代码可跨 macOS/Linux

### 依赖
- `chrono`: 时间戳生成
- `rand`: 随机文件名
- `atty`: 检测是否有管道输入

### 二进制大小
约 600KB/文件（已优化），可以进一步用 `strip` 减小：
```bash
strip target/release/{grep,tail,head}
# 减小到约 400KB
```

## 📝 许可

MIT License

**[DONE]**



