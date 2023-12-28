export * from "./node/lydie";
export * from "./web/lydie";

import init from "./web/lydie";

export default init;

export class Lydie {
  public image: Image;
  constructor(
    imageArray: number[],
    width: number,
    height: number,
    wasmMemory: WebAssembly.Memory
  );
}
