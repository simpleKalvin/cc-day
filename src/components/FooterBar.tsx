import type { DayInfo } from "../types";

interface FooterBarProps {
  day: DayInfo;
  onGoToToday: () => void;
}

export function FooterBar({ day, onGoToToday }: FooterBarProps) {
  return (
    <div className="footer">
      <span className="footer-info">
        {day.shengxiao}月 · {day.ganzhiYear}
      </span>
      <button className="today-btn" onClick={onGoToToday}>
        回到今天
      </button>
    </div>
  );
}
