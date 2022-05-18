import type { TAtomClassesOrArray } from "../types";

export function switchProp<TProps>(
  prop: string,
  options: Record<
    string,
    ((props: TProps) => TAtomClassesOrArray[]) | TAtomClassesOrArray
  >
) {
  return function getPropValue(props: TProps) {
    const propValue = options[props[prop]];
    if (propValue instanceof Function) {
      return propValue(props).flat();
    }
    return propValue;
  };
}
