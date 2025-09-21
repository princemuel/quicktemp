/** Helper for throwing errors in expression positions */
export function raise(error: unknown): never {
    throw typeof error === "string" ? new Error(error) : error;
}

export const isString = (value: unknown): value is string => typeof value === "string";

export const throwAsError = (exception: unknown) => {
    throw isString(exception) ? new Error(exception) : exception;
};

export function capitalize(s = "") {
    return s.charAt(0).toUpperCase() + s.slice(1).toLowerCase();
}
