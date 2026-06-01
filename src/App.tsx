import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { ThemeProvider } from "./components/ThemeProvider";
import { useCalendar } from "./hooks/useCalendar";
import { DayDetail } from "./components/DayDetail";
import { CalendarGrid } from "./components/CalendarGrid";
import { FooterBar } from "./components/FooterBar";
import { SettingsPage } from "./components/SettingsPage";
import { AboutPage } from "./components/AboutPage";
import type { PageId } from "./types";

function CalendarView({ onOpenSettings }: { onOpenSettings: () => void }) {
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
      {selectedDayInfo && (
        <FooterBar day={selectedDayInfo} onGoToToday={goToToday} onOpenSettings={onOpenSettings} />
      )}
    </div>
  );
}

function AppContent() {
  const [page, setPage] = useState<PageId>("calendar");

  const goToCalendar = () => setPage("calendar");

  useEffect(() => {
    const unlisten = listen<string>("navigate-to", async (event) => {
      const pageId = event.payload as PageId;
      setPage(pageId);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  if (page === "settings") {
    return (
      <div className="app-frame">
        <SettingsPage onBack={goToCalendar} />
      </div>
    );
  }

  if (page === "about") {
    return (
      <div className="app-frame">
        <AboutPage onBack={goToCalendar} />
      </div>
    );
  }

  return <CalendarView onOpenSettings={() => setPage("settings")} />;
}

export default function App() {
  return (
    <ThemeProvider>
      <AppContent />
    </ThemeProvider>
  );
}
