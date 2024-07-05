// A dependency graph that contains any wasm must all be imported
// asynchronously. This `main.js` file does the single async import, so
// that no one else needs to worry about it again.
import "./assets/global.css";
import("./scripts/index.ts").catch((e) =>
  console.error("Error importing `index.ts`:", e)
);
