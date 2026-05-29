import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { NavBar } from "./NavBar";

interface AboutPageProps {
  onBack: () => void;
}

export function AboutPage({ onBack }: AboutPageProps) {
  const [version, setVersion] = useState("...");

  useEffect(() => {
    invoke<string>("get_app_version").then(setVersion).catch(() => setVersion("unknown"));
  }, []);

  return (
    <div className="about-page">
      <NavBar title="关于" onBack={onBack} />
      <div className="about-content">
        <div className="about-icon">📅</div>
        <div className="about-name">CC-Day</div>
        <div className="about-version">版本 {version}</div>
        <div className="about-divider" />
        <div className="about-desc">
          一款简洁优雅的农历日历<br />托盘常驻，随时查看
        </div>
        <div className="about-copyright">© 2026 CC-Day</div>
      </div>
    </div>
  );
}
