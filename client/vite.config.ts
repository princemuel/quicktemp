import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { ViteMinifyPlugin } from 'vite-plugin-minify';

export default defineConfig({
  resolve: { tsconfigPaths: true },
  plugins: [tailwindcss(), ViteMinifyPlugin()],
});
