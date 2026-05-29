import { NavBar } from "./NavBar";
import { THEME_LIST } from "./ThemeProvider";
import { useTheme } from "../hooks/useTheme";

interface SettingsPageProps {
  onBack: () => void;
}

export function SettingsPage({ onBack }: SettingsPageProps) {
  const { theme, setTheme } = useTheme();

  return (
    <div className="settings-page">
      <NavBar title="偏好设置" onBack={onBack} />
      <div className="settings-content">
        <div className="settings-label">选择主题</div>
        <div className="theme-list">
          {THEME_LIST.map((t) => {
            const isActive = theme === t.id;
            return (
              <div
                key={t.id}
                className={`theme-card${isActive ? " is-active" : ""}`}
                onClick={() => setTheme(t.id)}
              >
                <div
                  className="theme-preview"
                  style={{ background: t.gradient }}
                />
                <div className="theme-info">
                  <div className="theme-name">{t.name}</div>
                  <div className="theme-desc">{t.description}</div>
                </div>
                {isActive && (
                  <div className="theme-check">
                    ✓
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
