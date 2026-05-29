import { getNearbyJieqi } from "../lib/lunar";
import type { DayInfo } from "../types";

interface DayDetailProps {
  day: DayInfo;
}

export function DayDetail({ day }: DayDetailProps) {
  const weekdays = ["日", "一", "二", "三", "四", "五", "六"];
  const solarDateStr = `${day.solarYear}年${day.solarMonth}月${day.solarDay}日 星期${weekdays[day.weekday]}`;
  const ganzhiStr = `${day.ganzhiYear}年 ${day.ganzhiMonth}月 ${day.ganzhiDay}日`;

  const nearbyJieqi = getNearbyJieqi(day.date);

  const tags: { label: string; type: "jieqi" | "festival" | "yi" }[] = [];
  if (nearbyJieqi) {
    tags.push({ label: nearbyJieqi, type: "jieqi" });
  }
  if (day.lunarFestival) {
    tags.push({ label: day.lunarFestival, type: "festival" });
  }
  if (day.solarFestival) {
    tags.push({ label: day.solarFestival, type: "festival" });
  }

  return (
    <div className="detail-header">
      <div className="solar-date">{solarDateStr}</div>
      <div className="lunar-date">
        {day.lunarMonthName}
        {day.lunarDayName}
      </div>
      <div className="ganzhi">{ganzhiStr}</div>
      {tags.length > 0 && (
        <div className="tags">
          {tags.map((tag) => (
            <span key={tag.label} className={`tag tag-${tag.type}`}>
              {tag.label}
            </span>
          ))}
        </div>
      )}
    </div>
  );
}
