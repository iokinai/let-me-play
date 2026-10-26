import {
  Gamepad2,
  Keyboard as KeyboardIcon,
  MousePointer,
  Move,
  Send,
} from "lucide-react";

export default function SingleEventForm({
  singleCategory,
  setSingleCategory,
  keyChar,
  setKeyChar,
  mouseBtn,
  setMouseBtn,
  moveX,
  setMoveX,
  moveY,
  setMoveY,
  isConnected,
  onSend,
}) {
  return (
    <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-6 flex flex-col gap-6 max-w-4xl w-full">
      <h2 className="text-sm font-bold text-slate-200 uppercase font-mono tracking-wider flex items-center gap-2">
        <Gamepad2 className="w-4 h-4 text-emerald-400" /> Настройка одиночного
        действия
      </h2>

      <div className="grid grid-cols-3 gap-3">
        <button
          onClick={() => setSingleCategory("keyboard")}
          className={`p-3 rounded-lg border text-xs font-mono flex flex-col items-center gap-2 transition ${
            singleCategory === "keyboard"
              ? "bg-emerald-950/40 border-emerald-500 text-emerald-400"
              : "bg-slate-950/40 border-slate-800 text-slate-400 hover:border-slate-700"
          }`}
        >
          <KeyboardIcon className="w-5 h-5" />
          Клавиатура
        </button>
        <button
          onClick={() => setSingleCategory("mouse_click")}
          className={`p-3 rounded-lg border text-xs font-mono flex flex-col items-center gap-2 transition ${
            singleCategory === "mouse_click"
              ? "bg-emerald-950/40 border-emerald-500 text-emerald-400"
              : "bg-slate-950/40 border-slate-800 text-slate-400 hover:border-slate-700"
          }`}
        >
          <MousePointer className="w-5 h-5" />
          Клик мышью
        </button>
        <button
          onClick={() => setSingleCategory("mouse_move")}
          className={`p-3 rounded-lg border text-xs font-mono flex flex-col items-center gap-2 transition ${
            singleCategory === "mouse_move"
              ? "bg-emerald-950/40 border-emerald-500 text-emerald-400"
              : "bg-slate-950/40 border-slate-800 text-slate-400 hover:border-slate-700"
          }`}
        >
          <Move className="w-5 h-5" />
          Движение мыши
        </button>
      </div>

      <div className="bg-slate-950/60 border border-slate-800/80 rounded-lg p-4">
        {singleCategory === "keyboard" && (
          <div className="flex flex-col gap-2">
            <label className="text-xs font-mono text-slate-400">
              СИМВОЛ КЛАВИШИ (a-z)
            </label>
            <input
              type="text"
              maxLength={1}
              value={keyChar}
              onChange={(e) => {
                const val = e.target.value.toLowerCase();
                if (/^[a-z]?$/.test(val)) setKeyChar(val);
              }}
              className="w-28 bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-center text-lg font-mono text-emerald-400 font-bold focus:outline-none focus:border-emerald-500"
            />
          </div>
        )}

        {singleCategory === "mouse_click" && (
          <div className="flex flex-col gap-2">
            <label className="text-xs font-mono text-slate-400">
              КНОПКА МЫШИ
            </label>
            <div className="flex gap-3">
              {[
                { id: "Left", label: "Левая" },
                { id: "Right", label: "Правая" },
              ].map((btn) => (
                <button
                  key={btn.id}
                  onClick={() => setMouseBtn(btn.id)}
                  className={`px-5 py-2.5 rounded-lg text-xs font-mono border transition ${
                    mouseBtn === btn.id
                      ? "bg-emerald-500 text-slate-950 font-bold border-emerald-400"
                      : "bg-slate-900 border-slate-700 text-slate-300"
                  }`}
                >
                  {btn.label} клик
                </button>
              ))}
            </div>
          </div>
        )}

        {singleCategory === "mouse_move" && (
          <div className="grid grid-cols-2 gap-4">
            <div className="flex flex-col gap-1.5">
              <label className="text-xs font-mono text-slate-400">
                СМЕЩЕНИЕ ПО X (px)
              </label>
              <input
                type="number"
                value={moveX}
                onChange={(e) => setMoveX(e.target.value)}
                className="bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-sm font-mono text-slate-200 focus:outline-none focus:border-emerald-500"
              />
            </div>
            <div className="flex flex-col gap-1.5">
              <label className="text-xs font-mono text-slate-400">
                СМЕЩЕНИЕ ПО Y (px)
              </label>
              <input
                type="number"
                value={moveY}
                onChange={(e) => setMoveY(e.target.value)}
                className="bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-sm font-mono text-slate-200 focus:outline-none focus:border-emerald-500"
              />
            </div>
          </div>
        )}
      </div>

      <button
        disabled={!isConnected}
        onClick={onSend}
        className="w-full py-3.5 rounded-lg bg-emerald-600 hover:bg-emerald-500 disabled:opacity-40 disabled:hover:bg-emerald-600 text-slate-950 font-bold text-xs font-mono transition flex items-center justify-center gap-2 shadow-lg shadow-emerald-950/50 cursor-pointer"
      >
        <Send className="w-4 h-4" /> ОТПРАВИТЬ СОБЫТИЕ
      </button>
    </div>
  );
}
