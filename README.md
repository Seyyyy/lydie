# lydie

lydie is color sampling library that is simulating human recognition.

## Installation

```sh
$ npm i lydie
```

## Usage

### Web

```ts
import init, { Lydie } from "lydie";

const wasm = init();
// hsvArray is 3-dimensional array in hsv color.
// First argument of Image constructor must be received 1-dimensional array.
const lydie = new Lydie(hsvArray.flat(), width, height, wasm.memory);
// Sapmling of colors.
lydie.image.calc_usage_rate();

// The number of grayscale pixels can be obtained.
lydie.image.get_usage_rate_gray_scale();
```

### Node.js

```ts
import { Lydie } from "lydie";
import * as wasm from "lydie/node/lydie_bg.wasm";

// hsvArray is 3-dimensional array in hsv color.
// First argument of Lydie constructor must be received 1-dimensional array.
const lydie = new Lydie(hsvArray.flat(), width, height, wasm.memory);
// Sapmling of colors.
lydie.image.calc_usage_rate();

// The number of grayscale pixels can be obtained.
lydie.image.get_usage_rate_gray_scale();
```

## Example

https://github.com/Seyyyy/lydie/tree/main/example
