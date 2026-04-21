export const throwAsError = (exception: unknown) => {
  throw typeof exception === "string" ? new Error(exception) : exception;
};
