import { ThemeProvider } from "./components/ThemeProvider";
import { useCalendar } from "./hooks/useCalendar";
import { DayDetail } from "./components/DayDetail";
import { CalendarGrid } from "./components/CalendarGrid";
import { FooterBar } from "./components/FooterBar";

function CalendarApp() {
  const {
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
  } = useCalendar();

  return (
    <div className="app-frame">
      {selectedDayInfo && <DayDetail day={selectedDayInfo} />}
      <div className="divider" />
      <CalendarGrid
        monthGrid={monthGrid}
        selectedDate={selectedDate}
        today={today}
        onSelectDate={selectDate}
        viewYear={viewYear}
        viewMonth={viewMonth}
        onPrevMonth={prevMonth}
        onNextMonth={nextMonth}
      />
      <div className="divider" />
      {selectedDayInfo && <FooterBar day={selectedDayInfo} onGoToToday={goToToday} />}
    </div>
  );
}

export default function App() {
  return (
    <ThemeProvider>
      <CalendarApp />
    </ThemeProvider>
  );
}
