import { Activity } from "react";
import type { State } from "..";
import InitFailed from "./InitFailed";
import InitPending from "./InitPending";
import InitSuccess from "./InitSuccess";

export default function InitPage({
  onInit,
  state,
}: {
  onInit: () => Promise<void>;
  state: State;
}) {
  return (
    <>
      <Activity mode={state === "pending" ? "visible" : "hidden"}>
        <InitPending onInit={onInit} />
      </Activity>
      <Activity mode={state === "success" ? "visible" : "hidden"}>
        <InitSuccess />
      </Activity>
      <Activity mode={state === "error" ? "visible" : "hidden"}>
        <InitFailed />
      </Activity>
    </>
  );
}
