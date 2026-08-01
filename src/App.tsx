import { Activity, useRef, useState } from "react";
import Backend, {
  Field,
  type ExprTree,
  type SortingCriterion,
} from "./backend";
import "./index.css";
import InitPage, { type State } from "./init-page";
import OptionPage from "./option-page";
import QueryPage from "./query-page";
import RenderPage from "./render";

function App() {
  const backend = useRef<Backend>(null);
  const [backendInUse, setBackendInUse] = useState(false);

  const [refreshSignal, refresh] = useState<Record<string, never>>({});
  const [initState, setInitState] = useState<State>("pending");
  const [page, setPage] = useState(0);

  const onInit = async () => {
    backend.current = new Backend();
    const state = await backend.current.getState();
    setInitState(state ? "success" : "error");
  };

  const onMatch = async (expr: ExprTree | undefined) => {
    if (expr && backend.current) {
      setBackendInUse(true);
      await backend.current.matchAll(expr);
      setBackendInUse(false);
      refresh({});
    }
  };

  const onSort = async (criteria: SortingCriterion[]) => {
    if (!backend.current) {
      return;
    }
    setBackendInUse(true);
    const success = await backend.current.sort(criteria);
    setBackendInUse(false);
    if (success) {
      refresh({});
    }
  };

  const readResult = async (skip: number, take: number, fields: Field[]) => {
    if (!backend.current) {
      return;
    }
    setBackendInUse(true);
    const success = await backend.current.readResult(skip, take, fields);
    setBackendInUse(false);
    return success;
  };

  const onReload = async () => {
    if (!backend.current) {
      return;
    }
    setBackendInUse(true);
    const success = await backend.current.loadData();
    setBackendInUse(false);
    return success;
  };

  return (
    <div
      style={{
        display: "flex",
        height: "stretch",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          flex: "1",
          margin: 8,
        }}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            gap: 10,
            flex: "1",
            minHeight: 0,
          }}
        >
          <div style={{ display: "flex", padding: 20 }}>
            {["Initialization", "Query", "Result", "Options"].map(
              (name, index) => (
                <button
                  key={index}
                  onClick={() => setPage(index)}
                  disabled={page === index}
                  style={{ flex: "1", fontSize: 20 }}
                >
                  {name}
                </button>
              ),
            )}
          </div>
          <div style={{ minHeight: 0, flex: "1" }}>
            <Activity mode={page === 0 ? "visible" : "hidden"}>
              <InitPage onInit={onInit} state={initState} />
            </Activity>
            <Activity mode={page === 1 ? "visible" : "hidden"}>
              <QueryPage
                actionsDisabled={backendInUse}
                onMatch={onMatch}
                onSort={onSort}
              />
            </Activity>
            <Activity mode={page === 2 ? "visible" : "hidden"}>
              <RenderPage
                readResult={readResult}
                refreshSignal={refreshSignal}
              />
            </Activity>
            <Activity mode={page === 3 ? "visible" : "hidden"}>
              <OptionPage onReload={onReload} reloadDisabled={backendInUse} />
            </Activity>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
