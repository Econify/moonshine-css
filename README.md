# Moonshine CSS

![](./docs/logo.svg)

## Features

- 🥃 Minimal and simple atomic CSS framework
- ⚗️ 100% customizable - build your own CSS framework
- 🦀 Blazing fast generator written in Rust
- 🐜 Tiny `1.7 kB` runtime
- ⚛️ CSS-in-JS library for React

## Install

```bash
npm install --save @econify/moonshine-css
yarn add @econify/moonshine-css
```

## Generate Styles

Create a `.moonshinerc` file in your project root.

by running

```
npx distill --init
```

or by creating a file manually:

```json
{
  "options": {
    "breakpoints": {
      "sm": "min-width: 576px",
      "md": "min-width: 768px",
      "lg": "min-width: 992px"
    },
    "designTokens": [
        "./atomic-design-tokens.yml"
    ],
    "templates": [
        "./tachyons-colors.yml",
        "./tachyons-flex.yml",
        "./tachyons-spacing.yml"
    ],
    "output": {
        "css": "./dist/styles.css",
        "json": "./dist/styles.json"
    }
  }
}
```

then run

```bash
npx distill --watch
```

## Usage

```js
import "atomic-styles.css";
```

```js
import { styled } from "@econify/moonshine-css";

const Button = styled.button(
  "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
  ({ isPrimary }) => isPrimary && "bg-primary text-white"
);

export default Demo() {
  return (
    <div>
      <Button isPrimary={true}>Click me</Button>
    </div>
  );
};
```

## Acknowledgements

TBD
