import { Terminal, Trash2, AlertCircle } from "lucide-react";

export default function TerminalLogs({ logs, onClearLogs }) {
  return (
    <aside className="w-96 shrink-0 border-l border-slate-800 bg-slate-900/40 flex flex-col font-mono text-xs min-h-0">
      <div className="p-3 border-b border-slate-800 flex items-center justify-between bg-slate-900/60 shrink-0">
        <div className="flex items-center gap-2 text-slate-300 font-bold">
          <Terminal className="w-4 h-4 text-emerald-400" />
          <span>ЖУРНАЛ СОБЫТИЙ</span>
        </div>
        <button
          onClick={onClearLogs}
          className="p-1.5 text-slate-400 hover:text-rose-400 hover:bg-slate-800 rounded transition cursor-pointer"
          title="Очистить лог"
        >
          <Trash2 className="w-3.5 h-3.5" />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto p-2.5 space-y-1.5 min-h-0">
        {logs.length === 0 ? (
          <div className="h-full flex flex-col items-center justify-center text-slate-600 gap-2">
            <AlertCircle className="w-6 h-6 opacity-40" />
            <span>События еще не отправлялись</span>
          </div>
        ) : (
          logs.map((log) => (
            <div
              key={log.id}
              className="p-2.5 rounded bg-slate-900/90 border border-slate-800/80 flex flex-col gap-1 transition-all"
            >
              <div className="flex items-center justify-between text-[10px]">
                <span className="text-slate-500">{log.time}</span>
                <span
                  className={`px-1.5 py-0.5 rounded text-[9px] uppercase border ${
                    log.status === "error"
                      ? "bg-rose-950 text-rose-400 border-rose-800"
                      : log.status === "info"
                        ? "bg-indigo-950 text-indigo-400 border-indigo-800"
                        : "bg-emerald-950 text-emerald-400 border-emerald-800"
                  }`}
                >
                  {log.type}
                </span>
              </div>
              <div className="text-slate-300 break-all font-mono text-[11px] bg-slate-950 p-1.5 rounded border border-slate-800/60 mt-1">
                {log.detail}
              </div>
            </div>
          ))
        )}
      </div>
    </aside>
  );
}
