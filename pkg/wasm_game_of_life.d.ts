/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
*/
export enum Cell {
  Dead,
  Alive,
}
/**
*/
export class Universe {
  free(): void;
/**
* @returns {Universe}
*/
  static new(): Universe;
/**
* @returns {string}
*/
  render(): string;
/**
*/
  tick(): void;
/**
* @returns {number}
*/
  width(): number;
/**
* @returns {number}
*/
  height(): number;
/**
* @returns {number}
*/
  cells(): number;
}
