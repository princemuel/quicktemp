import * as wasm from "./quicktemp_bg.wasm";
export * from "./quicktemp_bg.js";
import { __wbg_set_wasm } from "./quicktemp_bg.js";
__wbg_set_wasm(wasm);