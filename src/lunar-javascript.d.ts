declare module "lunar-javascript" {
  export class Solar {
    static fromDate(date: Date): Solar;
    static fromYmd(year: number, month: number, day: number): Solar;
    getYear(): number;
    getMonth(): number;
    getDay(): number;
    getWeek(): number;
    getFestivals(): string[];
    getLunar(): Lunar;
  }

  export class Lunar {
    getDayInChinese(): string;
    getMonthInChinese(): string;
    getYearInGanZhi(): string;
    getMonthInGanZhi(): string;
    getDayInGanZhi(): string;
    getYearShengXiao(): string;
    getJieQi(): string;
    getFestivals(): string[];
  }
}
