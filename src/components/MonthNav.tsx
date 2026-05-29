interface MonthNavProps {
  year: number;
  month: number;
  onPrev: () => void;
  onNext: () => void;
}

export function MonthNav({ year, month, onPrev, onNext }: MonthNavProps) {
  return (
    <div className="month-nav">
      <button className="nav-btn" onClick={onPrev}>
        ◀
      </button>
      <span className="month-title">
        {year}年{month}月
      </span>
      <button className="nav-btn" onClick={onNext}>
        ▶
      </button>
    </div>
  );
}
