import { Gamepad2, Wifi, WifiOff, Settings } from "lucide-react";

export default function Header({ isConnected, roomId, onOpenConfig }) {
  return (
    <header className="h-14 shrink-0 border-b border-slate-800 bg-slate-900/80 backdrop-blur px-6 flex items-center justify-between z-10">
      <div className="flex items-center gap-3">
        <div className="p-2 bg-emerald-500/10 border border-emerald-500/30 rounded-lg text-emerald-400">
          <Gamepad2 className="w-5 h-5" />
        </div>
        <div>
          <p className="text-xs text-slate-400">
            Конструктор событий и последовательностей
          </p>
        </div>
      </div>

      <div className="flex items-center gap-4">
        <div
          className={`flex items-center gap-2 px-3 py-1 rounded-full border text-xs font-mono transition-all ${
            isConnected
              ? "bg-emerald-950/60 border-emerald-500/40 text-emerald-400"
              : "bg-rose-950/60 border-rose-500/40 text-rose-400"
          }`}
        >
          {isConnected ? (
            <Wifi className="w-3.5 h-3.5 animate-pulse" />
          ) : (
            <WifiOff className="w-3.5 h-3.5" />
          )}
          <span className="truncate max-w-55">
            {isConnected ? `КОМНАТА: ${roomId}` : "НЕ ПОДКЛЮЧЕНО"}
          </span>
        </div>

        <button
          onClick={onOpenConfig}
          className="p-2 text-slate-400 hover:text-white hover:bg-slate-800 rounded-lg transition shrink-0"
          title="Настройки сессии"
        >
          <Settings className="w-4 h-4" />
        </button>
      </div>
    </header>
  );
}
