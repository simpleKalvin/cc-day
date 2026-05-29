interface NavBarProps {
  title: string;
  onBack: () => void;
}

export function NavBar({ title, onBack }: NavBarProps) {
  return (
    <div className="nav-bar">
      <button className="nav-back" onClick={onBack}>
        ← 返回
      </button>
      <span className="nav-title">{title}</span>
    </div>
  );
}
