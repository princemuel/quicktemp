/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Scale {
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
* @returns {Temperature}
*/
  build(): Temperature;
/**
* @returns {number}
*/
  convert(): number;
/**
* Set the temparature's value
* @param {number} value
* @returns {Temperature}
*/
  value(value: number): Temperature;
/**
* Set the temparature's source scale
* @param {Scale} value
* @returns {Temperature}
*/
  source_scale(value: Scale): Temperature;
/**
* Set the temparature's target scale
* @param {Scale} value
* @returns {Temperature}
*/
  target_scale(value: Scale): Temperature;
}
