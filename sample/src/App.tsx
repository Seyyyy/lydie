import "./App.css";
import { test_func } from "lydie";
import { useState } from "react";

function App() {
  const [count, setCount] = useState(1);

  return (
    <div>
      <h1>lydie sample</h1>
      <button onClick={() => setCount(test_func(count))}>counter</button>
      <span className="counter">{count}</span>
    </div>
  );
}

export default App;
