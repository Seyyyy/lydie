import init from "./pkg/lydie";

export default init;

export * from "./pkg/lydie";

export class Lydie {
  public image: Image;
  constructor(
    imageArray: number[],
    width: number,
    height: number,
    wasmMemory: WebAssembly.Memory
  );
}
