import type { DayInfo } from "../types";

interface DayCellProps {
  day: DayInfo;
  isToday: boolean;
  isSelected: boolean;
  onSelect: (date: Date) => void;
}

export function DayCell({ day, isToday, isSelected, onSelect }: DayCellProps) {
  const isWeekend = day.weekday === 0 || day.weekday === 6;
  const hasFestival = day.lunarFestival || day.solarFestival;
  const hasJieqi = day.jieqi;

  const cls = [
    "day-cell",
    isToday && "is-today",
    isSelected && "is-selected",
    isWeekend && "is-weekend",
    !day.isCurrentMonth && "is-other-month",
    hasFestival && "is-festival",
    hasJieqi && "is-jieqi",
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <div className={cls} onClick={() => onSelect(day.date)}>
      <div className="day-num">{day.solarDay}</div>
      <div className="day-lunar">{day.lunarDayText}</div>
    </div>
  );
}
