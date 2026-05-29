# Palace Jieqi Tag 配色 + Lunar Date 分隔符 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 修复 palace 主题下 jieqi tag 红绿配色冲突问题，并为所有主题的 lunar date 添加月份/日期分隔符。

**Architecture:** 纯 UI 层改动：CSS 变量覆盖 + JSX 拼接调整。无数据层变动。

**Tech Stack:** React 19, TypeScript, CSS custom properties

---

### Task 1: Lunar Date 添加中间点分隔符

**Files:**
- Modify: `src/components/DayDetail.tsx:29-31`

- [ ] **Step 1: 修改 lunar date 拼接，加入中间点**

将 `DayDetail.tsx` 第 29-31 行：

```tsx
<div className="lunar-date">
  {day.lunarMonthName}
  {day.lunarDayName}
</div>
```

改为：

```tsx
<div className="lunar-date">
  {day.lunarMonthName}·{day.lunarDayName}
</div>
```

- [ ] **Step 2: 启动 dev server 验证**

Run: `pnpm dev`
Expected: 浏览器 http://localhost:1420 中 detail header 的 lunar date 显示为 "四·十三" 格式

- [ ] **Step 3: 切换所有主题确认无异常**

依次切换 ink-wash、morandi、palace 三个主题，确认 lunar date 分隔符在所有主题下均正常显示。

- [ ] **Step 4: Commit**

```bash
git add src/components/DayDetail.tsx
git commit -m "feat: add middle dot separator to lunar date display"
```

---

### Task 2: Palace 主题 Jieqi Tag 配色改为金色系

**Files:**
- Modify: `src/index.css:82-84`（palace 主题 jieqi 变量）

- [ ] **Step 1: 更新 palace 主题的 jieqi CSS 变量**

将 `src/index.css` 中 palace 主题的 jieqi 变量（第 82-84 行）：

```css
--jieqi: #5a8a5a;
--jieqi-bg: rgba(90, 138, 90, 0.08);
--jieqi-border: rgba(90, 138, 90, 0.15);
```

改为：

```css
--jieqi: #d4b060;
--jieqi-bg: rgba(200, 164, 74, 0.15);
--jieqi-border: rgba(200, 164, 74, 0.25);
```

- [ ] **Step 2: 启动 dev server 验证**

Run: `pnpm dev`
Expected: palace 主题下，detail header 中 jieqi tag 显示为金色文字+金色半透明背景，在红色 header 上清晰可辨

- [ ] **Step 3: 确认日历网格节气颜色未变**

在 palace 主题下查看日历网格，确认 `.day-cell.is-jieqi .day-lunar` 仍然为绿色（未被新变量影响）。

- [ ] **Step 4: 确认其他主题未受影响**

切换 ink-wash 和 morandi 主题，确认 jieqi tag 颜色不变。

- [ ] **Step 5: Commit**

```bash
git add src/index.css
git commit -m "fix: change palace theme jieqi tag color to gold palette for better visibility"
```
