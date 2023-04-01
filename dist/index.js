import init, { Image, ColorModel } from "./pkg/lydie.js";
export default init;

export * from "./pkg/lydie.js";

export class Lydie {
  constructor(imageArray, width, height, wasmMemory) {
    this.image = new Image(imageArray.length, width, height, ColorModel.RGB);
    const ar = new Uint32Array(
      wasmMemory.buffer,
      this.image.hsv_pointer,
      imageArray.length
    );
    ar.set(imageArray);
  }
}
