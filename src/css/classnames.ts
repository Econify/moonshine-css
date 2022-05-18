import type { CxArgs } from "../types";

const unpackObj = (obj) =>
  Object.keys(obj).map((name) => (obj[name] ? name : null));

const processItem = (arg) =>
  typeof arg === "object" && arg !== null
    ? Array.isArray(arg)
      ? arg
      : unpackObj(arg)
    : arg;

function classnames(...args: CxArgs): string {
  return args
    .map(processItem)
    .flat()
    .filter((item) => item)
    .join(" ");
}

export default classnames;
