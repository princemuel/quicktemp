import { throwAsError } from "./utils";

/**
 * Find a DOM element or elements and validate their types.
 * @example
 * // Single element
 * const button = $("button", HTMLButtonElement)
 *
 * // Multiple elements
 * const buttons = $("button", HTMLButtonElement, true, document)
 */
export function $<E extends Element>(
  selector: string,
  Constructor: new (...args: unknown[]) => E,
  nodelist?: false,
  parent?: ParentNode,
): E;
export function $<E extends Element>(
  selector: string,
  Constructor: new (...args: unknown[]) => E,
  nodelist?: true,
  parent?: ParentNode,
): NodeListOf<E>;

export function $<E extends Element>(
  selector: string,
  Constructor: new (...args: unknown[]) => E,
  nodelist = false,
  parent: ParentNode = document,
): E | NodeListOf<E> {
  if (nodelist) {
    const elements = parent.querySelectorAll(selector);
    for (const element of elements) {
      if (!(element instanceof Constructor)) {
        throwAsError(`Element is not of type ${Constructor.name}: ${selector}`);
      }
    }
    return elements as NodeListOf<E>;
  }

  const element = parent.querySelector(selector) ?? throwAsError(`Element not found: ${selector}`);
  if (!(element instanceof Constructor)) {
    throwAsError(`Element is not of type ${Constructor.name}: ${selector}`);
  }
  return element as E;
}
