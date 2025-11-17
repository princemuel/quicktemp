import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
    plugins: [tailwindcss(), wasm()],
    build: { target: "esnext" },
});
