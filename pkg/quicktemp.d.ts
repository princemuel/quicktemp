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
