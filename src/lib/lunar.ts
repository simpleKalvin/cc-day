import { Solar } from "lunar-javascript";
import type { DayInfo, MonthGrid } from "../types";

export function getDayInfo(date: Date, isCurrentMonth = true): DayInfo {
  const solar = Solar.fromDate(date);
  const lunar = solar.getLunar();

  const lunarFestivals = lunar.getFestivals();
  const solarFestivals = solar.getFestivals();
  const jieqi = lunar.getJieQi();
  const lunarDayName = lunar.getDayInChinese();

  let lunarDayText = lunarDayName;
  if (lunarFestivals.length > 0) {
    lunarDayText = lunarFestivals[0];
  } else if (jieqi) {
    lunarDayText = jieqi;
  }

  return {
    date,
    solarYear: solar.getYear(),
    solarMonth: solar.getMonth(),
    solarDay: solar.getDay(),
    weekday: solar.getWeek(),
    lunarDayName,
    lunarMonthName: lunar.getMonthInChinese(),
    ganzhiYear: lunar.getYearInGanZhi(),
    ganzhiMonth: lunar.getMonthInGanZhi(),
    ganzhiDay: lunar.getDayInGanZhi(),
    shengxiao: lunar.getYearShengXiao(),
    jieqi: jieqi || null,
    lunarFestival: lunarFestivals.length > 0 ? lunarFestivals[0] : null,
    solarFestival: solarFestivals.length > 0 ? solarFestivals[0] : null,
    isCurrentMonth,
    lunarDayText,
  };
}

export function getMonthGrid(year: number, month: number): MonthGrid {
  const firstDay = new Date(year, month - 1, 1);
  const startWeekday = firstDay.getDay();

  const prevMonth = month === 1 ? 12 : month - 1;
  const prevYear = month === 1 ? year - 1 : year;
  const daysInPrevMonth = new Date(prevYear, prevMonth, 0).getDate();

  const daysInMonth = new Date(year, month, 0).getDate();

  const days: DayInfo[] = [];

  for (let i = startWeekday - 1; i >= 0; i--) {
    const day = daysInPrevMonth - i;
    const d = new Date(prevYear, prevMonth - 1, day);
    days.push(getDayInfo(d, false));
  }

  for (let day = 1; day <= daysInMonth; day++) {
    const d = new Date(year, month - 1, day);
    days.push(getDayInfo(d, true));
  }

  const remaining = 42 - days.length;
  const nextMonth = month === 12 ? 1 : month + 1;
  const nextYear = month === 12 ? year + 1 : year;

  for (let day = 1; day <= remaining; day++) {
    const d = new Date(nextYear, nextMonth - 1, day);
    days.push(getDayInfo(d, false));
  }

  return { year, month, days };
}

export function getNearbyJieqi(date: Date): string | null {
  const solar = Solar.fromDate(date);
  const lunar = solar.getLunar();
  const current = lunar.getJieQi();
  if (current) return current;

  for (let i = 1; i <= 15; i++) {
    const next = new Date(date);
    next.setDate(next.getDate() + i);
    const nextLunar = Solar.fromDate(next).getLunar();
    const jq = nextLunar.getJieQi();
    if (jq) return `${jq}将至`;
  }

  return null;
}
