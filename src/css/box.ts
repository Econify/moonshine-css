import React from "react";
import cx from "./classnames";
import type { CxArgs } from "../types";

export interface IStyledTagProps {
  children?: React.ReactNode;
  cx?: CxArgs;
  className?: never;
}

export type StyledTags = {
  [Tag in keyof JSX.IntrinsicElements]: React.FC<
    Omit<JSX.IntrinsicElements[Tag], "className"> & {
      cx?: CxArgs;
    } & IStyledTagProps
  >;
};

function make(tagName: string) {
  const component = function ({
    children,
    className,
    cx: atomicClasses = [],
    ...props
  }: IStyledTagProps) {
    if (
      typeof className !== "undefined" &&
      process.env.NODE_ENV !== "production"
    ) {
      console.warn(
        'Setting className is not allowed when using box component, use "cx" instead'
      );
    }

    return React.createElement(
      tagName,
      {
        ...props,
        className: cx(...atomicClasses),
      },
      children
    );
  };

  component.displayName = `styled-${tagName}`;
  return component;
}

const box = new Proxy(
  {},
  {
    get: (_target, tagName: string) => make(tagName),
  }
) as StyledTags;

export default box;
