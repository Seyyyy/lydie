import init, { Image } from "./web/lydie.js";

export * from "./node/lydie";
export * from "./web/lydie";

export default init;

export class Lydie {
  /**
   * @param {number[]} inputArray
   * @param {number} width
   * @param {number} height
   * @param {WebAssembly.Memory} wasmMemory
   */
  constructor(inputArray, width, height, wasmMemory) {
    this.image = new Image(inputArray.length, width, height);
    const imageArray = new Uint16Array(
      wasmMemory.buffer,
      this.image.hsv_pointer,
      inputArray.length
    );
    imageArray.set(inputArray);
  }
}
