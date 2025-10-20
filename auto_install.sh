#!/bin/bash

# ==============================================================================
# Cursor 强制最佳实践包装器 - 自动化安装脚本
# 自动选择最佳安装方式
# ==============================================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BINARIES=("grep" "tail" "head")

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 Cursor 强制最佳实践包装器 - 自动安装"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 检查编译产物
echo "📦 检查编译产物..."
ALL_EXIST=true
for bin in "${BINARIES[@]}"; do
    if [ ! -f "$SCRIPT_DIR/target/release/$bin" ]; then
        echo "❌ 错误: 找不到 $bin"
        ALL_EXIST=false
    fi
done

if [ "$ALL_EXIST" = false ]; then
    echo ""
    echo "需要先编译项目："
    echo "  cd $SCRIPT_DIR"
    echo "  source \$HOME/.cargo/env"
    echo "  cargo build --release"
    exit 1
fi
echo "✅ 所有二进制文件都存在"
echo ""

# 决定安装位置
if [ -w "/usr/local/bin" ]; then
    # 可以写入 /usr/local/bin，使用系统级安装
    INSTALL_DIR="/usr/local/bin"
    SUDO=""
    echo "📍 安装位置: $INSTALL_DIR（系统级，无需 sudo）"
else
    # 使用用户目录
    INSTALL_DIR="$HOME/bin"
    SUDO=""
    echo "📍 安装位置: $INSTALL_DIR（用户级）"
    mkdir -p "$INSTALL_DIR"
fi
echo ""

# 备份现有文件
echo "💾 备份现有文件（如果有）..."
for bin in "${BINARIES[@]}"; do
    if [ -f "$INSTALL_DIR/$bin" ]; then
        BACKUP_FILE="$INSTALL_DIR/${bin}.backup.$(date +%Y%m%d_%H%M%S)"
        mv "$INSTALL_DIR/$bin" "$BACKUP_FILE"
        echo "  • $bin -> $BACKUP_FILE"
    fi
done
echo ""

# 安装
echo "📥 安装包装器..."
for bin in "${BINARIES[@]}"; do
    cp "$SCRIPT_DIR/target/release/$bin" "$INSTALL_DIR/$bin"
    chmod +x "$INSTALL_DIR/$bin"
    echo "  ✅ $bin"
done
echo ""

# 检查 PATH
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 检查 PATH 配置..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ "$INSTALL_DIR" = "$HOME/bin" ]; then
    # 检查 PATH 中是否包含 ~/bin
    if [[ ":$PATH:" != *":$HOME/bin:"* ]]; then
        echo "⚠️  警告: $HOME/bin 不在 PATH 中"
        echo ""
        echo "需要添加以下内容到 ~/.zshrc 或 ~/.bashrc："
        echo ""
        echo "  export PATH=\"\$HOME/bin:\$PATH\""
        echo ""
        
        # 自动添加到配置文件
        if [ -f "$HOME/.zshrc" ]; then
            if ! grep -q "export PATH=\"\$HOME/bin:\$PATH\"" "$HOME/.zshrc"; then
                echo "🔧 自动添加到 ~/.zshrc..."
                echo "" >> "$HOME/.zshrc"
                echo "# Cursor 强制最佳实践包装器" >> "$HOME/.zshrc"
                echo "export PATH=\"\$HOME/bin:\$PATH\"" >> "$HOME/.zshrc"
                echo "✅ 已添加"
                echo ""
                echo "请运行以下命令使其生效："
                echo "  source ~/.zshrc"
            fi
        fi
        
        if [ -f "$HOME/.bashrc" ]; then
            if ! grep -q "export PATH=\"\$HOME/bin:\$PATH\"" "$HOME/.bashrc"; then
                echo "🔧 自动添加到 ~/.bashrc..."
                echo "" >> "$HOME/.bashrc"
                echo "# Cursor 强制最佳实践包装器" >> "$HOME/.bashrc"
                echo "export PATH=\"\$HOME/bin:\$PATH\"" >> "$HOME/.bashrc"
                echo "✅ 已添加"
                echo ""
                echo "请运行以下命令使其生效："
                echo "  source ~/.bashrc"
            fi
        fi
    fi
fi

# 验证安装
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ 安装完成！"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "当前命令路径："
export PATH="$INSTALL_DIR:$PATH"
for bin in "${BINARIES[@]}"; do
    WHICH_RESULT=$(which $bin 2>/dev/null || echo "未找到")
    echo "  • $bin: $WHICH_RESULT"
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📋 使用说明"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "✅ 在 Cursor 终端中："
echo "   • grep 自动添加 -C 20 上下文"
echo "   • tail/head 最少 100 行"
echo "   • 管道输入自动保存到 /tmp/cursor_outputs/"
echo ""
echo "✅ 在普通终端中："
echo "   • 完全正常使用，无任何影响"
echo ""
echo "🧪 测试命令："
echo "   echo 'test' | grep 'test'"
echo "   seq 1 20 | tail -5"
echo ""



