/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Unit {
  Celsius = 0,
  Fahrenheit = 1,
  Kelvin = 2,
}
/**
*/
export class Temperature {
  free(): void;
/**
* @returns {Temperature}
*/
  static new(): Temperature;
/**
* @returns {number}
*/
  convert(): number;
/**
* @returns {number}
*/
  get_degree(): number;
/**
* Set the temparature's degree
* @param {number} degree
*/
  set_degree(degree: number): void;
/**
* @returns {Unit}
*/
  get_from_unit(): Unit;
/**
* Set the temparature's from unit
* @param {Unit} unit
*/
  set_from_unit(unit: Unit): void;
/**
* @returns {Unit}
*/
  get_to_unit(): Unit;
/**
* Set the temparature's to unit
* @param {Unit} unit
*/
  set_to_unit(unit: Unit): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_temperature_free: (a: number) => void;
  readonly temperature_new: () => number;
  readonly temperature_convert: (a: number) => number;
  readonly temperature_get_degree: (a: number) => number;
  readonly temperature_set_degree: (a: number, b: number) => void;
  readonly temperature_get_from_unit: (a: number) => number;
  readonly temperature_set_from_unit: (a: number, b: number) => void;
  readonly temperature_get_to_unit: (a: number) => number;
  readonly temperature_set_to_unit: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
