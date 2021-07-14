// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
import("./index.js")
  .catch(e => console.error("Error importing `index.js`:", e));


window.addEventListener("load", () => {
  const module = window.sc_internal_wrapper().then(module => {
    console.log("in internal wrapper");
    window.sc_internal = module;
    window.wasm.main();
  });
});
