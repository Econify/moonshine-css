import React from "react";
import { tokens2variableMap, cssFromVarMap } from "./export";

// Rename to theme provider
export default function CssVariableInjector({ tokens, rootVars }) {
  const brandVarMap = tokens2variableMap(tokens);
  const rootCss = cssFromVarMap({ ...rootVars, ...brandVarMap });

  return React.createElement("style", {
    dangerouslySetInnerHTML: { __html: rootCss },
  });
}
