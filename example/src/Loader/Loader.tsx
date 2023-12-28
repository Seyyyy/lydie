import React, { useState, useEffect } from "react";
import init, { Lydie, Image as LImage, InitOutput } from "lydie";
import url from "lydie/web/lydie_bg.wasm?url";
import "./Loader.css";
import { Viewer } from "./Viewer";

export function ImageLoader() {
  const [limage, setLimage] = useState<LImage | null>(null);
  const [imageURL, setImageURL] = useState<string | null>(null);
  const [time, setTime] = useState<number>(0);
  const [imageSize, setImageSize] = useState({
    width: 0,
    height: 0,
  });
  const [wasm, setWasm] = useState<InitOutput>({} as InitOutput);

  useEffect(() => {
    (async () => {
      const wasm = await init(url);
      setWasm(wasm);
    })();
  }, []);

  const handleImageChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      let reader = new FileReader();
      reader.onload = (e) => {
        if (!e.target) throw new Error();
        if (!e.target.result) throw new Error();
        setImageURL(e.target.result as string);
        let img = new Image();
        img.src = e.target.result as string;
        img.onload = () => {
          let canvas = document.createElement("canvas");
          canvas.width = img.width;
          canvas.height = img.height;
          let ctx = canvas.getContext("2d");
          if (!ctx) throw new Error();
          ctx.drawImage(img, 0, 0);
          let imageData = ctx.getImageData(0, 0, img.width, img.height);
          let data = imageData.data;
          let rgbArray: number[][] = [];
          for (let i = 0; i < data.length; i += 4) {
            let r = data[i];
            let g = data[i + 1];
            let b = data[i + 2];
            const ar = rgb2hsv(r, g, b);
            rgbArray.push([ar[0], ar[1], ar[2]]);
          }

          let before = performance.now();

          const flatArray = rgbArray.flat();

          const analyzedImage = new Lydie(
            flatArray,
            img.width,
            img.height,
            wasm.memory
          ).image;
          analyzedImage.calc_usage_rate();
          setLimage(analyzedImage);

          let after = performance.now();
          setTime(after - before);
          setImageSize({
            width: img.width,
            height: img.height,
          });
        };
      };
      reader.readAsDataURL(e.target.files[0]);
    }
  };

  return (
    <div className="root">
      <input type="file" accept="image/*" onChange={handleImageChange} />
      {imageURL && (
        <img
          className="img"
          src={imageURL}
          alt="image"
          width="300"
          height="500"
        />
      )}
      <p>Analyze time is {time}ms</p>
      <p>
        Image size is {imageSize.width} x {imageSize.height}
      </p>
      {limage && <Viewer image={limage} />}
    </div>
  );
}

const rgb2hsv = (r: number, g: number, b: number): number[] => {
  r /= 255;
  g /= 255;
  b /= 255;
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  let h = 0;
  let s = 0;
  let v = max;
  const d = max - min;
  s = max === 0 ? 0 : d / max;
  if (max === min) {
    h = 0;
  } else {
    switch (max) {
      case r:
        h = (g - b) / d + (g < b ? 6 : 0);
        break;
      case g:
        h = (b - r) / d + 2;
        break;
      case b:
        h = (r - g) / d + 4;
        break;
    }
    h /= 6;
  }
  h = Math.round(h * 360);
  s = Math.round(s * 100);
  v = Math.round(v * 100);
  return [h, s, v];
};
