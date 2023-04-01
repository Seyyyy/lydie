import init, { Image } from "./pkg/lydie.js";
export default init;

export * from "./pkg/lydie.js";

export class Lydie {
  constructor(imageArray, width, height, ColorModel) {
    this.image = new Image(imageArray, width, height, ColorModel);
  }
}
