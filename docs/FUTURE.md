# $-Syntax

Applying atomic classes directly as props would simplify styling by reducing the time spend typing. It also simplifies refactoring and improves auto-completion, due to the fact that the class names are prefixed with `$`. Also, since these are just boolean properties, they can easily get applied conditionally. But most importently this solves the problem of having an easy method to distinguish between atomic classes
and regular props WITHOUT shipping a full list of all valid HTML props to the client.

```tsx
export default function Example1() {
  return (
    <>
      <box.button $f $m-0 $p-2 $bg-white $c-primary onClick={() => log("hi")} />
    </>
  );
}
```

There are a couple of challenges with this approach however:

1. While a colon symbold would be possible `$hover:foo`, this relies on the namepsace feature of JSX, which can be enabled with `throwIfNamespace: false`. This however is not supported by all toolchains.

2. When dealing with variants in most cases multiple classes need to be applied.

A possible solution to both problems could look like this:

```tsx
import { vx } from "@econify/moonshine-css";

export default function Example2() {
  const variant = "primary";
  return (
    <div>
      <box.div
        $m0
        //
        $lg-mt5 // Breakpoints - Option 1
        $lg={["mt5", "bg-red"]} // Breakpoints - Option 2
        //
        $hover$bg-red // Pseudo class - Option 1
        $hover={["bg-red", "fs5"]} // Pseudo class -  Option 2
        $bg-red={"hover"} // Pseudo class - Option 3
        //
        {...vx(variant === "primary" ? ["c-primary", "bg-primary"] : [])}
      />
    </div>
  );
}
```

# Mixing Atomic CSS with CSS selectors

One of the problems with most CSS-in-JS frameworks is that they don't support
arbritrary CSS selectors. The following syntax could be a solution for this. It allows developers to define custom selectors, but still locks them into the atomic design system.

```ts
const somethingElse = css`
  c-red mv-5
`;

const newClass = css`
  @media (prefers-reduced-motion: no-preference) {
    ${somethingElse}:before {
      f md-5 c-grey block;
      color: red;
    }
  }
`;
```

# Server side rendering

At build time `css` should get replace with a string containing the name of the scoped css class. The build output would then look like this for each `css` template tag:

```tsx
const simpleCssClass = "sfody3d";
const variants = {
  primary: "p1nnge0e",
  secondary: "s1vfkouw",
};
```

This will require some additional build-time tooling.
