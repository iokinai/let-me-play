import { useState, useCallback } from "react";
import { Gamepad2, Layers } from "lucide-react";
import Header from "./components/Header";
import SingleEventForm from "./components/SingleEventForm";
import SequenceEventForm from "./components/SequenceEventForm";
import TerminalLogs from "./components/TerminalLogs";
import ConfigModal from "./components/ConfigModal";

export default function App() {
  const serverUrl = import.meta.env.VITE_BACKEND_URL || "http://127.0.0.1:8080";

  const [roomId, setRoomId] = useState("");
  const [isConnected, setIsConnected] = useState(false);
  const [showConfigModal, setShowConfigModal] = useState(true);

  const [eventType, setEventType] = useState("single"); // 'single' | 'sequence'

  // Форма Single Event
  const [singleCategory, setSingleCategory] = useState("keyboard");
  const [keyChar, setKeyChar] = useState("w");
  const [mouseBtn, setMouseBtn] = useState("Left");
  const [moveX, setMoveX] = useState(10);
  const [moveY, setMoveY] = useState(10);

  // Форма Sequence Event
  const [sequenceList, setSequenceList] = useState([
    {
      id: "1",
      category: "keyboard",
      key: "w",
      button: "Left",
      x: 0,
      y: 0,
      delay_ms: 100,
    },
  ]);

  const [logs, setLogs] = useState([]);

  const addLog = useCallback((type, detail, status = "success") => {
    const newEntry = {
      id: Math.random().toString(36).substring(2, 9),
      time: new Date().toLocaleTimeString([], {
        hour12: false,
        minute: "2-digit",
        second: "2-digit",
        fractionalSecondDigits: 3,
      }),
      type,
      detail,
      status,
    };
    setLogs((prev) => [newEntry, ...prev.slice(0, 99)]);
  }, []);

  const buildSingleEvent = (category, key, button, x, y) => {
    if (category === "keyboard") {
      return {
        Input: {
          Keyboard: key.toLowerCase() || "a",
        },
      };
    } else if (category === "mouse_click") {
      return {
        Input: {
          Mouse: {
            Click: button,
          },
        },
      };
    } else {
      return {
        Input: {
          Mouse: {
            Move: {
              x: parseInt(x, 10) || 0,
              y: parseInt(y, 10) || 0,
            },
          },
        },
      };
    }
  };

  const sendPayload = async (eventPayload, desc) => {
    if (!roomId) return;
    try {
      const baseUrl = serverUrl.replace(/\/$/, "");
      const response = await fetch(`${baseUrl}/api/room`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          room_id: roomId,
          event: eventPayload,
        }),
      });

      if (!response.ok) {
        // Пытаемся распарсить JSON ошибку сервера (если вернулась LmpServerError)
        let errorText = `HTTP ${response.status}`;
        try {
          const errData = await response.json();
          if (errData.message)
            errorText = `${errData.message} (${errData.type})`;
        } catch (_) {}
        addLog(desc, errorText, "error");
      } else {
        addLog(desc, JSON.stringify(eventPayload), "success");
      }
    } catch (err) {
      addLog(desc, err.message || "Ошибка сети", "error");
    }
  };

  const handleSendSingle = () => {
    const singleEv = buildSingleEvent(
      singleCategory,
      keyChar,
      mouseBtn,
      moveX,
      moveY,
    );
    sendPayload({ Single: singleEv }, "ОДИНОЧНОЕ СОБЫТИЕ");
  };

  const handleSendSequence = () => {
    if (sequenceList.length === 0) return;

    const formattedSequence = sequenceList.map((item) => ({
      event: buildSingleEvent(
        item.category,
        item.key,
        item.button,
        item.x,
        item.y,
      ),
      delay_ms: parseInt(item.delay_ms, 10) || 0,
    }));

    sendPayload(
      { Sequence: formattedSequence },
      `ПОСЛЕДОВАТЕЛЬНОСТЬ (${sequenceList.length} шаг.)`,
    );
  };

  const addSequenceItem = () => {
    setSequenceList((prev) => [
      ...prev,
      {
        id: Math.random().toString(36).substring(2, 9),
        category: "keyboard",
        key: "a",
        button: "Left",
        x: 0,
        y: 0,
        delay_ms: 100,
      },
    ]);
  };

  const removeSequenceItem = (id) => {
    setSequenceList((prev) => prev.filter((item) => item.id !== id));
  };

  const updateSequenceItem = (id, field, value) => {
    setSequenceList((prev) =>
      prev.map((item) => (item.id === id ? { ...item, [field]: value } : item)),
    );
  };

  const handleConnectSuccess = (validRoomId) => {
    setRoomId(validRoomId);
    setIsConnected(true);
    setShowConfigModal(false);
    addLog("СИСТЕМА", `Успешное подключение к комнате: ${validRoomId}`, "info");
  };

  return (
    <div className="flex flex-col h-dvh w-full bg-slate-950 text-slate-100 font-sans antialiased overflow-hidden select-none">
      <Header
        isConnected={isConnected}
        roomId={roomId}
        onOpenConfig={() => setShowConfigModal(true)}
      />

      <div className="flex-1 flex min-h-0 overflow-hidden">
        <main className="flex-1 min-w-0 p-6 flex flex-col gap-6 bg-slate-950 overflow-y-auto">
          {/* Переключатель вкладок режимов */}
          <div className="flex gap-2 p-1 bg-slate-900 border border-slate-800 rounded-xl shrink-0 max-w-md">
            <button
              onClick={() => setEventType("single")}
              className={`flex-1 py-2 px-4 rounded-lg text-xs font-bold font-mono transition flex items-center justify-center gap-2 cursor-pointer ${
                eventType === "single"
                  ? "bg-emerald-600 text-slate-950 shadow"
                  : "text-slate-400 hover:text-slate-200"
              }`}
            >
              <Gamepad2 className="w-4 h-4" />
              ОДИНОЧНОЕ
            </button>
            <button
              onClick={() => setEventType("sequence")}
              className={`flex-1 py-2 px-4 rounded-lg text-xs font-bold font-mono transition flex items-center justify-center gap-2 cursor-pointer ${
                eventType === "sequence"
                  ? "bg-emerald-600 text-slate-950 shadow"
                  : "text-slate-400 hover:text-slate-200"
              }`}
            >
              <Layers className="w-4 h-4" />
              ПОСЛЕДОВАТЕЛЬНОСТЬ
            </button>
          </div>

          {eventType === "single" ? (
            <SingleEventForm
              singleCategory={singleCategory}
              setSingleCategory={setSingleCategory}
              keyChar={keyChar}
              setKeyChar={setKeyChar}
              mouseBtn={mouseBtn}
              setMouseBtn={setMouseBtn}
              moveX={moveX}
              setMoveX={setMoveX}
              moveY={moveY}
              setMoveY={setMoveY}
              isConnected={isConnected}
              onSend={handleSendSingle}
            />
          ) : (
            <SequenceEventForm
              sequenceList={sequenceList}
              onAddStep={addSequenceItem}
              onRemoveStep={removeSequenceItem}
              onUpdateStep={updateSequenceItem}
              isConnected={isConnected}
              onSend={handleSendSequence}
            />
          )}
        </main>

        <TerminalLogs logs={logs} onClearLogs={() => setLogs([])} />
      </div>

      {showConfigModal && (
        <ConfigModal
          serverUrl={serverUrl}
          onConnectSuccess={handleConnectSuccess}
        />
      )}
    </div>
  );
}
