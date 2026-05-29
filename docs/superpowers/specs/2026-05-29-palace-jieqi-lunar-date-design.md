# Palace 主题 Jieqi Tag 配色 + Lunar Date 分隔符改进

## 背景

两个 UI 问题需要修复：

1. Palace（宫墙红）主题下，detail header 中的 jieqi tag 使用绿色系（`--jieqi: #5a8a5a`），与红色背景（`#c4342e → #d44838`）形成红绿冲突，辨识度极低。
2. Lunar date 直接拼接 `lunarMonthName` + `lunarDayName`（如 "四" + "十三" = "四十三"），看起来像一个数字，无法区分月份与日期。

## 设计决策

### 1. Lunar Date 分隔符

**改动**：在 `DayDetail.tsx` 中，于 `lunarMonthName` 和 `lunarDayName` 之间插入中间点 `·` 分隔符。

**效果**：四十三 → 四·十三

**范围**：所有主题通用。

### 2. Palace 主题 Jieqi Tag 配色

**改动**：为 palace 主题新增专用的 jieqi tag CSS 变量，使用金色系替代绿色系。

**配色方案（金色系）**：

| 变量 | 值 |
|------|------|
| `--jieqi` (tag 文字) | `#d4b060` |
| `--jieqi-bg` (tag 背景) | `rgba(200, 164, 74, 0.15)` |
| `--jieqi-border` (tag 边框) | `rgba(200, 164, 74, 0.25)` |

**范围**：仅影响 palace 主题的 detail header 中 `tag-jieqi` 标签。日历网格中 `.day-cell.is-jieqi .day-lunar` 保持绿色不变。

## 技术方案

### 文件改动

**`src/index.css`**：
- palace 主题的 `--jieqi`、`--jieqi-bg`、`--jieqi-border` 变量更新为金色系值
- 不新增变量，直接覆盖现有变量（仅 palace 作用域内生效）

**`src/components/DayDetail.tsx`**：
- lunar date 区域从 `{day.lunarMonthName}{day.lunarDayName}` 改为 `{day.lunarMonthName}·{day.lunarDayName}`

### 不改动的部分

- 其他两个主题（ink-wash、morandi）的 jieqi 配色不变
- 日历网格中节气日的颜色不变
- types、lunar 等数据层不变
