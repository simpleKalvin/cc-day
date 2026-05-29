export interface DayInfo {
  date: Date;
  solarYear: number;
  solarMonth: number;
  solarDay: number;
  weekday: number;
  lunarDayName: string;
  lunarMonthName: string;
  ganzhiYear: string;
  ganzhiMonth: string;
  ganzhiDay: string;
  shengxiao: string;
  jieqi: string | null;
  lunarFestival: string | null;
  solarFestival: string | null;
  isCurrentMonth: boolean;
  lunarDayText: string;
}

export interface MonthGrid {
  year: number;
  month: number;
  days: DayInfo[];
}

export type ThemeId = "ink-wash" | "morandi" | "palace";

export interface ThemeConfig {
  id: string;
  name: string;
  variables: Record<string, string>;
  isBuiltIn?: boolean;
}

export type PageId = "calendar" | "settings" | "about";

export interface ThemeMeta {
  id: ThemeId;
  name: string;
  description: string;
  gradient: string;
}
