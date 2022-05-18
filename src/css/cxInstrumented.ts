import type { CxArgs } from "../types";
import classnames from "./classnames";

const __used_css_classes = {};
const classnamesInstrumented = (...classes: CxArgs): string => {
  classes
    .flat()
    .forEach(
      (cls: string) =>
        (__used_css_classes[cls] = __used_css_classes[cls]
          ? __used_css_classes[cls] + 1
          : 1)
    );
  return classnames(...classes);
};

if (typeof window !== "undefined") {
  // @ts-ignore
  window.__used_css_classes = __used_css_classes;
}

export default classnamesInstrumented;
