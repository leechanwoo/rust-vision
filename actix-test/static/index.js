import init from "./wasm-app/pkg/wasm_app.js";

window.addEventListener('load', async() => {
    await init();
});
