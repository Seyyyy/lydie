import init, { ColorModel } from "./pkg/lydie";

export default init;

export * from "./pkg/lydie";

export class Lydie {
  public image: Image;
  constructor(
    imageArray: Uint32Array,
    width: number,
    height: number,
    ColorModel: ColorModel
  );
}
