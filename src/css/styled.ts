import React from "react";
import cx from "./classnames";
import type { CxArgs, TAtomClassesOrArray } from "../types";

type CxArgsWithFn<T> = (
  | ((props: T) => TAtomClassesOrArray)
  | TAtomClassesOrArray
)[];

type IStyledProps = {
  cx?: CxArgs;
};

type TStyled = {
  [Tag in keyof JSX.IntrinsicElements]: <T>(
    ...atomicClasses: CxArgsWithFn<T>
  ) => React.FC<
    Omit<JSX.IntrinsicElements[Tag], "className"> & IStyledProps & T
  >;
};

type IComponentProps = React.PropsWithChildren<{
  className: never;
  cx?: CxArgs;
}>;

type TInherit = {
  inherit: (
    base: React.FC<IStyledProps>
  ) => (...additionalClasses: CxArgs) => React.FC<IStyledProps>;
};

function makeCreateStyledComponent(tagName: string) {
  return function createStyledComponent(
    ...atomicClasses: CxArgsWithFn<unknown>
  ) {
    return function Component({
      children,
      className,
      cx: atomicClassesOverride = [],
      ...props
    }: IComponentProps) {
      if (
        typeof className !== "undefined" &&
        process.env.NODE_ENV !== "production"
      ) {
        console.warn('Setting className is not allowed, use "cx" instead');
      }

      // Create a mocked replica of the props object, that keeps track of
      // properties that have been accessed as a side effect. This is to prevent
      // passing invalid props to the DOM element
      const propsWithGetter = {};
      const didAccessProp: Record<string, boolean> = {};
      Object.keys(props).forEach((key) => {
        Object.defineProperty(propsWithGetter, key, {
          get() {
            didAccessProp[key] = true;
            return props[key];
          },
        });
      });

      const atomicClassesUnpacked = atomicClasses.map((classOrFn) =>
        typeof classOrFn === "function" ? classOrFn(propsWithGetter) : classOrFn
      );

      const filteredProps = {};
      Object.keys(props).forEach((key) => {
        // The prop was accessed during the class list generation, so we should
        // not pass it through to the DOM element
        if (!didAccessProp[key]) {
          filteredProps[key] = props[key];
        }
      });

      return React.createElement(
        tagName,
        {
          ...filteredProps,
          className: cx(...atomicClassesUnpacked, ...atomicClassesOverride),
        },
        children
      );
    };
  };
}

function inherit(baseComponent) {
  return function (...additionalClasses: CxArgs) {
    return function Component({ cx: atomicClassesOverride = [], ...props }) {
      return baseComponent({
        cx: [...additionalClasses, ...atomicClassesOverride],
        ...props,
      });
    };
  };
}

const styled = new Proxy(
  {},
  {
    get: (_target, tagName: string) =>
      tagName === "inherit" ? inherit : makeCreateStyledComponent(tagName),
  }
) as TStyled & TInherit;

export default styled;
