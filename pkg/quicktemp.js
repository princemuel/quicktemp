import * as wasm from "./quicktemp_bg.wasm";
import { __wbg_set_wasm } from "./quicktemp_bg.js";
__wbg_set_wasm(wasm);
export * from "./quicktemp_bg.js";
