import { useState } from "react";
import { Gamepad2 } from "lucide-react";

export default function ConfigModal({ serverUrl, onConnectSuccess }) {
  const [inputRoomId, setInputRoomId] = useState("");
  const [loading, setLoading] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");

  const handleVerifyAndConnect = async (e) => {
    e.preventDefault();
    const trimmed = inputRoomId.trim();
    if (!trimmed) return;

    setLoading(true);
    setErrorMsg("");

    try {
      const baseUrl = serverUrl.replace(/\/$/, "");
      const res = await fetch(
        `${baseUrl}/api/room?id=${encodeURIComponent(trimmed)}`,
      );

      if (!res.ok) {
        throw new Error(`Ошибка сервера: ${res.status}`);
      }

      const data = await res.json();

      if (data.exists) {
        onConnectSuccess(trimmed);
      } else {
        setErrorMsg("Комната с таким ID не найдена на хосте!");
      }
    } catch (err) {
      setErrorMsg(err.message || "Не удалось связаться с сервером");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-slate-950/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div className="bg-slate-900 border border-slate-800 rounded-xl w-full max-w-md p-6 shadow-2xl">
        <div className="flex items-center gap-3 mb-6">
          <div className="p-2.5 bg-emerald-500/10 border border-emerald-500/30 rounded-lg text-emerald-400">
            <Gamepad2 className="w-6 h-6" />
          </div>
          <div>
            <h3 className="text-lg font-bold text-white">
              Подключение к хосту
            </h3>
            <p className="text-xs text-slate-400">
              Введите ID активной сессии комнаты
            </p>
          </div>
        </div>

        <form onSubmit={handleVerifyAndConnect} className="space-y-4">
          <div>
            <label className="block text-xs font-mono text-slate-400 mb-1.5">
              ID КОМНАТЫ / КОД СЕССИИ
            </label>
            <input
              type="text"
              value={inputRoomId}
              onChange={(e) => setInputRoomId(e.target.value)}
              placeholder="например: 8fbc1850-25e2-4886-b4d4..."
              className="w-full bg-slate-950 border border-slate-800 rounded-lg px-3 py-2.5 text-sm text-slate-200 focus:outline-none focus:border-emerald-500/60 font-mono"
              required
            />
          </div>

          {errorMsg && (
            <div className="p-2.5 bg-rose-950/60 border border-rose-800/80 text-rose-400 text-xs rounded-lg font-mono">
              {errorMsg}
            </div>
          )}

          <div className="pt-2 flex items-center justify-end">
            <button
              type="submit"
              disabled={loading}
              className="w-full py-3 rounded-lg bg-emerald-600 hover:bg-emerald-500 disabled:opacity-50 text-slate-950 font-bold text-xs font-mono transition shadow-lg shadow-emerald-950 cursor-pointer"
            >
              {loading ? "Проверка комнаты..." : "Подключиться"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
