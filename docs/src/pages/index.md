# lydie

lydie is color sampling library that is simulating human recognition.

## Getting Started

```sh
$ npm i lydie
```

```ts
import init, { Lydie } from "lydie";

const wasm = init();
// hsvArray is 3-dimensional array in hsv color.
// First argument of Image constructor must be received 1-dimensional array.
const lydie = new Image(hsvArray.flat(), width, height, wasm.memory);
// Sapmling of colors.
lydie.image.calc_usage_rate();

// The number of grayscale pixels can be obtained.
lydie.image.get_usage_rate_gray_scale();
```

## Document

More discription is [here](/docs/intro).

## Example

https://github.com/Seyyyy/lydie/tree/main/example
