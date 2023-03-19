# lydie

lydie is color sampling library that is simulating human recognition.

## Installation

```sh
$ npm i lydie
```

## Usage

```ts
import { Image } from "lydie";

// hsvArray is 3-dimensional array in hsv color.
// First argument of Image constructor must be received 1-dimensional array.
const image = new Image(new Uint32Array(hsvArray.flat()), width, height);
// Sapmling of colors.
image.calc_usage_rate();

// The number of grayscale pixels can be obtained.
image.get_usage_rate_gray_scale();
```

## Example
