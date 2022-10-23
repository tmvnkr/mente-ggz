import { qwikCity } from "@builder.io/qwik-city/middleware/node";
import render from "./entry.ssr";

/**
 * The default export is the QwikCity adaptor used by Vite preview.
 */
export default qwikCity(render);
