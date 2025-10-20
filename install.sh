#!/bin/bash

# ==============================================================================
# Cursor 强制最佳实践包装器 - Rust 二进制版安装脚本
# ==============================================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
INSTALL_DIR="/usr/local/bin"
BINARIES=("grep" "tail" "head")

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 Cursor 强制最佳实践包装器 - Rust 二进制版安装"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 检查编译产物是否存在
echo "📦 检查编译产物..."
for bin in "${BINARIES[@]}"; do
    if [ ! -f "$SCRIPT_DIR/target/release/$bin" ]; then
        echo "❌ 错误: 找不到编译产物: $bin"
        echo "请先运行: cargo build --release"
        exit 1
    fi
done
echo "✅ 所有编译产物都存在"
echo ""

# 检查是否需要 sudo
echo "🔐 检查权限..."
if [ ! -w "$INSTALL_DIR" ]; then
    echo "⚠️  需要 sudo 权限来安装到 $INSTALL_DIR"
    SUDO="sudo"
else
    SUDO=""
fi
echo ""

# 备份原有的包装器（如果存在）
echo "💾 备份现有文件（如果有）..."
for bin in "${BINARIES[@]}"; do
    if [ -f "$INSTALL_DIR/$bin" ]; then
        BACKUP_FILE="$INSTALL_DIR/${bin}.backup.$(date +%Y%m%d_%H%M%S)"
        $SUDO mv "$INSTALL_DIR/$bin" "$BACKUP_FILE"
        echo "  • $bin -> $BACKUP_FILE"
    fi
done
echo ""

# 安装新的二进制文件
echo "📥 安装新的包装器..."
for bin in "${BINARIES[@]}"; do
    $SUDO cp "$SCRIPT_DIR/target/release/$bin" "$INSTALL_DIR/$bin"
    $SUDO chmod +x "$INSTALL_DIR/$bin"
    echo "  ✅ $bin -> $INSTALL_DIR/$bin"
done
echo ""

# 验证安装
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 验证安装..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

for bin in "${BINARIES[@]}"; do
    WHICH_RESULT=$(which $bin)
    echo "  • $bin: $WHICH_RESULT"
    if [ "$WHICH_RESULT" != "$INSTALL_DIR/$bin" ]; then
        echo "    ⚠️  警告: 当前 shell 中 $bin 指向 $WHICH_RESULT"
        echo "    💡 请确保 /usr/local/bin 在 PATH 中的优先级高于其他路径"
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 安装完成！"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📌 工作原理："
echo "   • 这些二进制包装器会自动检测是否在 Cursor 环境中"
echo "   • 在 Cursor 中: 强制应用最佳实践（-C 20、至少 100 行）"
echo "   • 在普通终端: 直接调用原始命令，无任何影响"
echo ""
echo "📌 检测方式："
echo "   • 检查环境变量（TERM_PROGRAM、VSCODE_GIT_ASKPASS_MAIN 等）"
echo "   • 检查进程树中是否包含 Cursor 进程"
echo ""
echo "📌 测试方法："
echo "   # 在 Cursor 终端中："
echo "   echo 'test' | grep 'test'    # 会看到强制提示"
echo "   seq 1 10 | tail -3           # 会强制改为 -100"
echo ""
echo "   # 在普通终端中："
echo "   echo 'test' | grep 'test'    # 正常输出，无提示"
echo ""



