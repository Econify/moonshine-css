# Moonshine CSS Syntax

Moonshine CSS supports 3 ways of writing CSS-in-JS. `cx` is the basic building block, whereas `styled.*` and `box.*` provide some syntactic sugar on top of `cx`.

## `cx`

`cx` is heavily inspired by [classnames](https://github.com/JedWatson/classnames) and supports the following syntax:

```ts
cx("foo", "bar"); // => 'foo bar'
cx("foo", ["bar", "baz"], []); // => 'foo bar baz'
cx("foo", { bar: true }); // => 'foo bar'
cx({ "foo-bar": true }); // => 'foo-bar'
cx({ "foo-bar": false }); // => ''
cx({ foo: true }, { bar: true }); // => 'foo bar'
cx({ foo: true, bar: true }); // => 'foo bar'

// lots of arguments of various types
cx("foo", { bar: true, duck: false }, "baz", { quux: true }); // => 'foo bar baz quux'

// other falsy values are just ignored
cx(null, false, "bar", undefined, 0, 1, { baz: null }, ""); // => 'bar 1'
```

Note: Arrays are not supported by the original `classnames` package.

A typical component using atomic CSS classes looks like this:

```tsx
import { cx } from "@utils";

export function Example1a({ children }) {
  return (
    <div className={cx("m4", "f", "bg-gray300", "wsn", "pt2")}>{children}</div>
  );
}
```

A typical component with variants looks like this:

```tsx
import { cx } from "@utils";

export function Example2({ children, variant }) {
  return (
    <div
      className={cx(
        "m1",
        variant === "primary" && ["m4", "f"],
        variant === "secondary" && ["abs", "bg-advertising", "pt2", "hover:o80"]
      )}
    >
      {children}
    </div>
  );
}
```

## `box`

The box syntax is inspired by [xstyled](https://xstyled.dev/) and [styled systems](https://styled-system.com/). It's a more convenient way to write CSS-in-JS, because it doesn't not require using `cx` within `className`. Instead `cx` is a prop that is available on every box component.

```tsx
import { box } from "@utils";

export function Button({ children, cx }) {
  return (
    <box.button
      cx={["m1", "f", "bg-feature", "ttu"]}
      onClick={() => console.log("click")}
    >
      {children}
    </box.button>
  );
}
```

## `styled`

`styled` is inspired by [styled-components](https://www.styled-components.com/).
It's an easier way of creating component that only apply styling.

`styled.<tag>(classes)` creates a React component with tag `<tag>` where `classes` is a list of utiliy classes. `styled` uses `cx` internally, so anything that `cx` supports it also supported by `styled`.

```tsx
const StyledButton = styled.button("m1", "f", "bg-feature", "ttu");
```

`styled` also supports conditional styles:

```tsx
const StyledLink = styled.a<{ isPrimary: boolean }>(
  "m1",
  ({ isPrimary }) => isPrimary && ["m4", "dbr"]
);
```

And inheritance (combining classes of the base class and child class):

```tsx
const PrimaryStyledDiv = styled.inherit(StyledLink)("m2", "bg-primary");
```

## `css`

`css` is Moonshine's escape hatch. It allows you to write CSS-in-JS without using atomic classes. Similar to CSS-modules it generates a unique scoped classname.
It is inspired by [linaria](https://linaria.dev/).

```tsx
import { css } from "@utils";

const simpleCssClass = css`
  color: green;
`;

const variants = {
  primary: css`
    border: 10px solid red;
    font-weight: bold;
  `,
  secondary: css`
    border: 10px solid blue;
    padding: 2rem;
  `,
};

function Button() {
  return <div className={`${simpleCssClass} ${variants.primary}`>;
}
```

Currently `css` does not support server-side redering (SSR).

### Mixing atomic CSS classes with other classes

To bypass Moonshine's typechecking and allow mixing of atomic CSS classes with other classes, you can use `untyped` to use any string as a classname.

```tsx
import React from "react";
import { untyped } from "@econify/moonshine-css";

import styles from "styles.module.css"; // CSS modules

const exampleClass = css`color: green;`;

export default function Example() {
  return (
    <div
      className={cx(
        "bg-blue6",
        untyped("my-global-class")
        untyped(exampleClass).
        untyped(styles.box),
      )}
    >
      Box
    </div>
  );
}
```

Mixing CSS classes should be done with care, because the order in which styles get loaded is not guaranteed. Please make sure the custom class always has higher specificity than the atomic utily classes used.
