import { useMemo, useState } from "react";
import { getMonthGrid } from "../lib/lunar";
import type { DayInfo, MonthGrid } from "../types";

export function useCalendar() {
  const today = useMemo(() => {
    const now = new Date();
    return new Date(now.getFullYear(), now.getMonth(), now.getDate());
  }, []);

  const [selectedDate, setSelectedDate] = useState<Date>(today);
  const [viewYear, setViewYear] = useState(today.getFullYear());
  const [viewMonth, setViewMonth] = useState(today.getMonth() + 1);

  const monthGrid: MonthGrid = useMemo(
    () => getMonthGrid(viewYear, viewMonth),
    [viewYear, viewMonth],
  );

  const selectedDayInfo: DayInfo | null = useMemo(() => {
    return (
      monthGrid.days.find(
        (d) =>
          d.solarYear === selectedDate.getFullYear() &&
          d.solarMonth === selectedDate.getMonth() + 1 &&
          d.solarDay === selectedDate.getDate(),
      ) ?? null
    );
  }, [monthGrid, selectedDate]);

  function prevMonth() {
    if (viewMonth === 1) {
      setViewMonth(12);
      setViewYear((y) => y - 1);
    } else {
      setViewMonth((m) => m - 1);
    }
  }

  function nextMonth() {
    if (viewMonth === 12) {
      setViewMonth(1);
      setViewYear((y) => y + 1);
    } else {
      setViewMonth((m) => m + 1);
    }
  }

  function goToToday() {
    const now = new Date();
    const todayDate = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    setSelectedDate(todayDate);
    setViewYear(todayDate.getFullYear());
    setViewMonth(todayDate.getMonth() + 1);
  }

  function selectDate(date: Date) {
    setSelectedDate(date);
    if (
      date.getFullYear() !== viewYear ||
      date.getMonth() + 1 !== viewMonth
    ) {
      setViewYear(date.getFullYear());
      setViewMonth(date.getMonth() + 1);
    }
  }

  return {
    today,
    selectedDate,
    selectedDayInfo,
    viewYear,
    viewMonth,
    monthGrid,
    prevMonth,
    nextMonth,
    goToToday,
    selectDate,
  };
}
