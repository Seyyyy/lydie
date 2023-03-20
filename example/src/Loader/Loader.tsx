import React, { useState } from "react";
import { Image as LImage, ColorModel } from "lydie";
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
            rgbArray.push([r, g, b]);
          }

          let before = performance.now();

          const analyzedImage = new LImage(
            new Uint32Array(rgbArray.flat()),
            img.width,
            img.height,
            ColorModel.RGB
          );
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
