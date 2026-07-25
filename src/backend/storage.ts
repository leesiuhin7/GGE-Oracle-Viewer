import type Result from "./result";
import { Engine, type Files } from "./wrapper";

function onError(): never {
  throw new Error("Storage is not initialized.");
}

const storage: {
  _files: Files | undefined;
  get files(): Files;
  set files(files: Files);

  _engine: Engine | undefined;
  get engine(): Engine;
  set engine(engine: Engine);

  _result: Result | undefined;
  get result(): Result;
  set result(result: Result);
} = {
  _files: undefined,
  get files() {
    return this._files ?? onError();
  },
  set files(files: Files) {
    this._files = files;
  },

  _engine: undefined,
  get engine() {
    return this._engine ?? onError();
  },
  set engine(engine: Engine) {
    this._engine = engine;
  },

  _result: undefined,
  get result() {
    return this._result ?? onError();
  },
  set result(result: Result) {
    this._result = result;
  },
};

export default storage;
