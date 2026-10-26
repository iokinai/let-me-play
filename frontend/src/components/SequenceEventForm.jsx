import { Layers, Plus, Clock, Trash2, Send } from "lucide-react";

export default function SequenceEventForm({
  sequenceList,
  onAddStep,
  onRemoveStep,
  onUpdateStep,
  isConnected,
  onSend,
}) {
  return (
    <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-6 flex flex-col gap-6 max-w-4xl w-full">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-sm font-bold text-slate-200 uppercase font-mono tracking-wider flex items-center gap-2">
            <Layers className="w-4 h-4 text-emerald-400" /> Конструктор
            последовательности
          </h2>
          <p className="text-xs text-slate-400 font-mono mt-0.5">
            Пакетная отправка шагов с задержками delay_ms
          </p>
        </div>
        <button
          onClick={onAddStep}
          className="px-3 py-2 rounded-lg bg-slate-800 hover:bg-slate-700 text-emerald-400 border border-emerald-500/30 text-xs font-mono transition flex items-center gap-1.5 cursor-pointer"
        >
          <Plus className="w-4 h-4" /> Добавить шаг
        </button>
      </div>

      <div className="flex flex-col gap-3">
        {sequenceList.map((item, idx) => (
          <div
            key={item.id}
            className="bg-slate-950/70 border border-slate-800 rounded-lg p-3 flex flex-wrap items-center gap-3 font-mono text-xs"
          >
            <span className="w-6 text-center text-slate-500 font-bold shrink-0">
              #{idx + 1}
            </span>

            <select
              value={item.category}
              onChange={(e) =>
                onUpdateStep(item.id, "category", e.target.value)
              }
              className="bg-slate-900 border border-slate-700 rounded px-2.5 py-1.5 text-slate-200 text-xs focus:outline-none shrink-0"
            >
              <option value="keyboard">Клавиатура</option>
              <option value="mouse_click">Клик мышью</option>
              <option value="mouse_move">Движение мыши</option>
            </select>

            <div className="flex-1 flex items-center gap-3 min-w-50">
              {item.category === "keyboard" && (
                <div className="flex items-center gap-2">
                  <span className="text-slate-500">Символ:</span>
                  <input
                    type="text"
                    maxLength={1}
                    value={item.key}
                    onChange={(e) => {
                      const val = e.target.value.toLowerCase();
                      if (/^[a-z0-9]?$/.test(val))
                        onUpdateStep(item.id, "key", val);
                    }}
                    className="w-12 bg-slate-900 border border-slate-700 rounded px-2 py-1 text-center font-bold text-emerald-400 focus:outline-none"
                  />
                </div>
              )}

              {item.category === "mouse_click" && (
                <div className="flex items-center gap-2">
                  <span className="text-slate-500">Кнопка:</span>
                  <select
                    value={item.button}
                    onChange={(e) =>
                      onUpdateStep(item.id, "button", e.target.value)
                    }
                    className="bg-slate-900 border border-slate-700 rounded px-2.5 py-1.5 text-slate-200 text-xs focus:outline-none"
                  >
                    <option value="Left">Левая</option>
                    <option value="Right">Правая</option>
                  </select>
                </div>
              )}

              {item.category === "mouse_move" && (
                <div className="flex items-center gap-2">
                  <span className="text-slate-500">X:</span>
                  <input
                    type="number"
                    value={item.x}
                    onChange={(e) => onUpdateStep(item.id, "x", e.target.value)}
                    className="w-20 bg-slate-900 border border-slate-700 rounded px-2 py-1 text-slate-200 focus:outline-none"
                  />
                  <span className="text-slate-500">Y:</span>
                  <input
                    type="number"
                    value={item.y}
                    onChange={(e) => onUpdateStep(item.id, "y", e.target.value)}
                    className="w-20 bg-slate-900 border border-slate-700 rounded px-2 py-1 text-slate-200 focus:outline-none"
                  />
                </div>
              )}
            </div>

            <div className="flex items-center gap-2 shrink-0 border-l border-slate-800 pl-3">
              <Clock className="w-3.5 h-3.5 text-amber-400" />
              <span className="text-slate-400">Задержка (мс):</span>
              <input
                type="number"
                value={item.delay_ms}
                onChange={(e) =>
                  onUpdateStep(item.id, "delay_ms", e.target.value)
                }
                className="w-24 bg-slate-900 border border-slate-700 rounded px-2.5 py-1 text-amber-300 font-bold focus:outline-none"
              />
            </div>

            <button
              onClick={() => onRemoveStep(item.id)}
              className="p-1.5 text-slate-500 hover:text-rose-400 hover:bg-slate-900 rounded transition shrink-0 cursor-pointer"
              title="Удалить шаг"
            >
              <Trash2 className="w-4 h-4" />
            </button>
          </div>
        ))}
      </div>

      <button
        disabled={!isConnected || sequenceList.length === 0}
        onClick={onSend}
        className="w-full py-3.5 rounded-lg bg-emerald-600 hover:bg-emerald-500 disabled:opacity-40 disabled:hover:bg-emerald-600 text-slate-950 font-bold text-xs font-mono transition flex items-center justify-center gap-2 shadow-lg shadow-emerald-950/50 cursor-pointer"
      >
        <Send className="w-4 h-4" /> ОТПРАВИТЬ ПОСЛЕДОВАТЕЛЬНОСТЬ
      </button>
    </div>
  );
}
