import { useState, useEffect } from "react";
import init, { Lydie, Image as LImage, InitOutput } from "lydie";
import url from "lydie/pkg/lydie_bg.wasm?url";

const TEST_IMAGE_WIDTH = 1000;
const TEST_IMAGE_HEIGHT = 1000;

function createHsvArray(pixelCount: number): number[] {
  const hsvArray = new Array(pixelCount);
  for (let i = 0; i < pixelCount * 3; i += 3) {
    hsvArray[i] = Math.floor(Math.random() * 360);
    hsvArray[i + 1] = Math.floor(Math.random() * 100);
    hsvArray[i + 2] = Math.floor(Math.random() * 100);
  }
  return hsvArray;
}

async function createTestImages(createNumber: number) {
  const images = [];
  for (let i = 0; i < createNumber; i++) {
    images.push(createHsvArray(TEST_IMAGE_WIDTH * TEST_IMAGE_HEIGHT));
  }
  return images;
}

function App() {
  const [wasm, setWasm] = useState<InitOutput>({} as InitOutput);
  const [images, setImages] = useState<number[][] | null>(null);
  const [analyze, setAnalyze] = useState<LImage | null>(null);
  const [time, setTime] = useState<number>(0);

  useEffect(() => {
    (async () => {
      const wasm = await init(url);
      setWasm(wasm);
    })();
  }, []);

  useEffect(() => {
    (async () => {
      const images = await createTestImages(1);
      setImages([...images]);
    })();
  }, []);

  useEffect(() => {
    if (images && wasm.memory) {
      createAnalyze();
    }
  }, [images, wasm]);

  function createAnalyze() {
    if (images && wasm) {
      let before = performance.now();
      const analyzedImage = new Lydie(
        images[0],
        TEST_IMAGE_WIDTH,
        TEST_IMAGE_HEIGHT,
        wasm.memory
      ).image;
      analyzedImage.calc_usage_rate();
      setTime(performance.now() - before);

      setAnalyze(analyzedImage);
    }
  }

  return (
    <>
      {wasm && <p>wasm module loaded</p>}
      {images && <p>image created</p>}
      {analyze && (
        <>
          <p>image analyzed</p>
          <div id="time">
            <p>{time}</p>
          </div>
        </>
      )}
    </>
  );
}

export default App;
