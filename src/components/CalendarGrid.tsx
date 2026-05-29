import { DayCell } from "./DayCell";
import type { MonthGrid } from "../types";

interface CalendarGridProps {
  monthGrid: MonthGrid;
  selectedDate: Date;
  today: Date;
  onSelectDate: (date: Date) => void;
}

const WEEKDAYS = ["日", "一", "二", "三", "四", "五", "六"];

export function CalendarGrid({
  monthGrid,
  selectedDate,
  today,
  onSelectDate,
}: CalendarGridProps) {
  return (
    <div className="calendar">
      <div className="grid">
        {WEEKDAYS.map((name, i) => (
          <div
            key={name}
            className={`weekday${i === 0 || i === 6 ? " is-weekend" : ""}`}
          >
            {name}
          </div>
        ))}

        {monthGrid.days.map((day, i) => {
          const isToday =
            day.solarYear === today.getFullYear() &&
            day.solarMonth === today.getMonth() + 1 &&
            day.solarDay === today.getDate();

          const isSelected =
            day.solarYear === selectedDate.getFullYear() &&
            day.solarMonth === selectedDate.getMonth() + 1 &&
            day.solarDay === selectedDate.getDate();

          return (
            <DayCell
              key={`${day.solarYear}-${day.solarMonth}-${day.solarDay}-${i}`}
              day={day}
              isToday={isToday}
              isSelected={isSelected}
              onSelect={onSelectDate}
            />
          );
        })}
      </div>
    </div>
  );
}
