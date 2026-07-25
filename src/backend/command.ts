import { loadData, matchAll, readResult, sort } from "./methods";

interface Functions {
  matchAll: typeof matchAll;
  sort: typeof sort;
  readResult: typeof readResult;
  loadData: typeof loadData;
}

export type Command = {
  [Name in keyof Functions]: {
    command: Name;
    args: Parameters<Functions[Name]>;
  };
}[keyof Functions];

type RemovePromise<T> = T extends Promise<infer U> ? U : T;

export type CommandResult = {
  [Name in keyof Functions]: {
    command: Name;
    result: RemovePromise<ReturnType<Functions[Name]>>;
  };
}[keyof Functions];

export interface CommandRequest {
  id: number;
  command: Command;
}

export interface CommandResponse {
  id: number;
  result: CommandResult;
}

export type Message =
  | {
      type: "ready";
    }
  | {
      type: "response";
      response: CommandResponse;
    };

export default async function runCommand({
  command,
  args,
}: Command): Promise<CommandResult> {
  switch (command) {
    case "matchAll":
      return {
        command,
        result: matchAll(...args),
      };
    case "sort":
      return {
        command,
        result: sort(...args),
      };
    case "readResult":
      return {
        command,
        result: readResult(...args),
      };
    case "loadData":
      return { command, result: await loadData(...args) };
  }
}
