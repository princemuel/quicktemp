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
  value(): number;
/**
* Set the temparature's degree
* @param {number} degree
*/
  set_value(degree: number): void;
/**
* @returns {Unit}
*/
  from(): Unit;
/**
* Set the temparature's from unit
* @param {Unit} unit
*/
  set_from(unit: Unit): void;
/**
* @returns {Unit}
*/
  to(): Unit;
/**
* Set the temparature's to unit
* @param {Unit} unit
*/
  set_to(unit: Unit): void;
}
