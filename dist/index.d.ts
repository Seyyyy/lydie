import init from "./pkg/lydie";

export default init;

export * from "./pkg/lydie";

export class Lydie {
  public image: Image;
  constructor(
    arr: number[],
    width: number,
    height: number,
    wasmMemory: WebAssembly.Memory
  );
}
