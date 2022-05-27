import React from "react";
import Example from "./Example";
import { CssVariableInjector } from "@econify/moonshine-css";

import "./global.css";
import "../build/tokens.css";
import "../build/styles.css";

const tokens = {
  color: { red: "red", blue: "blue" },
  font: { openSans: "Open Sans", times: "Times New Roman" },
  spacing: [0.25, 0.5, 1, 2, 4, 8, 16],
  letterSpacing: [0, 2, 4],
  paragraphSpacing: [0, 2, 4],
  borderRadius: [0, 6, 12],
  opacity: [0, 0.5, 1],
  lineHeight: [1, 1.25, 1.5, 2],
  fontSize: [10, 12, 14, 16, 18, 20, 24, 28, 32, 36, 40, 44, 48],
};

export default function App() {
  return (
    <main>
      <CssVariableInjector tokens={tokens} />
      <Example />
    </main>
  );
}
