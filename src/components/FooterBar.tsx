import type { DayInfo } from "../types";

interface FooterBarProps {
  day: DayInfo;
  onGoToToday: () => void;
  onOpenSettings: () => void;
}

export function FooterBar({ day, onGoToToday, onOpenSettings }: FooterBarProps) {
  return (
    <div className="footer">
      <div className="footer-left">
        <button className="footer-settings-btn" onClick={onOpenSettings}>
          偏好
        </button>
        <span className="footer-info">
          {day.shengxiao}月 · {day.ganzhiYear}
        </span>
      </div>
      <button className="today-btn" onClick={onGoToToday}>
        回到今天
      </button>
    </div>
  );
}
