#!/bin/bash

# ==============================================================================
# Cursor 强制最佳实践包装器 - 卸载脚本
# ==============================================================================

set -e

INSTALL_DIR="/usr/local/bin"
BINARIES=("grep" "tail" "head")

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🗑️  卸载 Cursor 强制最佳实践包装器"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 检查是否需要 sudo
if [ ! -w "$INSTALL_DIR" ]; then
    echo "⚠️  需要 sudo 权限"
    SUDO="sudo"
else
    SUDO=""
fi

# 删除包装器
echo "🗑️  删除包装器..."
for bin in "${BINARIES[@]}"; do
    if [ -f "$INSTALL_DIR/$bin" ]; then
        $SUDO rm -f "$INSTALL_DIR/$bin"
        echo "  ✅ 已删除 $INSTALL_DIR/$bin"
    else
        echo "  ℹ️  $INSTALL_DIR/$bin 不存在"
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ 卸载完成！"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "当前命令路径:"
for bin in "${BINARIES[@]}"; do
    echo "  • $bin: $(which $bin)"
done
echo ""


