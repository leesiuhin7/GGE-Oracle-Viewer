import type {
  Command,
  CommandRequest,
  CommandResult,
  Message,
} from "./command";
import type { ExprTree } from "./expr";
import { Field } from "./field";
import type { ResultSnapshots } from "./result";
import type { SortingCriterion } from "./sorting";
import BackendWorker from "./worker?worker";

export { Field };
export type { ExprTree, ResultSnapshots, SortingCriterion };

export default class Backend {
  private readonly worker: Worker;
  private readonly listeners: Map<number, (response: CommandResult) => void> =
    new Map();
  private nextId: number = 0;

  private setState: (state: boolean) => void = () => {};
  private readonly state = new Promise<boolean>((resolve) => {
    this.setState = resolve;
  });

  constructor() {
    this.worker = new BackendWorker();
    this.worker.onmessage = (event: MessageEvent<Message>) => {
      if (event.data.type === "response") {
        const { id, result } = event.data.response;
        this.listeners.get(id)?.(result);
      } else if (event.data.type === "ready") {
        this.setState(true);
      }
    };
    this.worker.onerror = () => {
      this.setState(false);
    };
  }

  public getState(): Promise<boolean> {
    return this.state;
  }

  public async matchAll(expr: ExprTree): Promise<void> {
    const { command, result } = await this.runCommand({
      command: "matchAll",
      args: [expr],
    });
    if (command !== "matchAll") {
      throw new Error("Received invalid response.");
    }
    return result;
  }

  public async sort(criteria: SortingCriterion[]): Promise<boolean> {
    const { command, result } = await this.runCommand({
      command: "sort",
      args: [criteria],
    });
    if (command !== "sort") {
      throw new Error("Received invalid response.");
    }
    return result;
  }

  public async readResult(
    skip: number,
    take: number,
    fileds: Field[],
  ): Promise<ResultSnapshots> {
    const { command, result } = await this.runCommand({
      command: "readResult",
      args: [skip, take, fileds],
    });
    if (command !== "readResult") {
      throw new Error("Received invalid response.");
    }
    return result;
  }

  public async loadData(): Promise<boolean> {
    const { command, result } = await this.runCommand({
      command: "loadData",
      args: [],
    });
    if (command !== "loadData") {
      throw new Error("Received invalid response.");
    }
    return result;
  }

  private async runCommand(command: Command): Promise<CommandResult> {
    const id = this.nextId;
    this.nextId += 1;

    const request: CommandRequest = {
      id,
      command,
    };
    this.worker.postMessage(request);

    return await new Promise((resolve) => {
      this.listeners.set(id, resolve);
    });
  }
}
