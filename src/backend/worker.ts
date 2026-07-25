import runCommand, { type CommandRequest, type Message } from "./command";
import SyncFile from "./file";
import { loadData } from "./methods";
import Result from "./result";
import storage from "./storage";
import { Engine, type Files } from "./wrapper";

async function createSyncFile(
  fileHandle: FileSystemFileHandle,
): Promise<SyncFile> {
  const syncHandle = await fileHandle.createSyncAccessHandle();
  return new SyncFile(syncHandle);
}

async function fileInit(): Promise<Files> {
  const root = await navigator.storage.getDirectory();
  const dirHandle = await root.getDirectoryHandle("backend", {
    create: true,
  });

  const dataFileHandle = await dirHandle.getFileHandle("data", {
    create: true,
  });
  const tempFile1Handle = await dirHandle.getFileHandle("sorting_temp1", {
    create: true,
  });
  const tempFile2Handle = await dirHandle.getFileHandle("sorting_temp2", {
    create: true,
  });
  const sortFileHandle = await dirHandle.getFileHandle("sorting_result", {
    create: true,
  });

  return {
    dataFile: await createSyncFile(dataFileHandle),
    tempFile1: await createSyncFile(tempFile1Handle),
    tempFile2: await createSyncFile(tempFile2Handle),
    sortResultFile: await createSyncFile(sortFileHandle),
  };
}

const files = await fileInit();
const engine = new Engine(files);
const result = new Result(engine);

storage.files = files;
storage.engine = engine;
storage.result = result;

if (files.dataFile.handle.getSize() === 0) {
  await loadData(); // Load data if dataFile is empty
}

function sendMessage(message: Message) {
  self.postMessage(message);
}

onmessage = async (event: MessageEvent<CommandRequest>) => {
  const { id, command } = event.data;
  const message: Message = {
    type: "response",
    response: {
      id,
      result: await runCommand(command),
    },
  };
  sendMessage(message);
};

sendMessage({ type: "ready" });
