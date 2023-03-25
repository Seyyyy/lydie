import "./App.css";
import { ImageLoader } from "./Loader/Loader";
import init from "lydie";
import url from "lydie/lydie_bg.wasm?url";
import { useEffect } from "react";

function App() {
  useEffect(() => {
    (async () => {
      await init(url);
    })();
  }, []);

  return (
    <div>
      <h1>lydie sample</h1>
      <ImageLoader />
    </div>
  );
}

export default App;
