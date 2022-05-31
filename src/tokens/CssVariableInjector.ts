import React from "react";
import cssFromVarMap from "./cssFromVarMap";
import tokens2variableMap from "./tokens2variableMap";

// Rename to theme provider
export default function CssVariableInjector({ tokens, rootVars }) {
  const brandVarMap = tokens2variableMap(tokens);
  const rootCss = cssFromVarMap({ ...rootVars, ...brandVarMap });

  return React.createElement("style", {
    dangerouslySetInnerHTML: { __html: rootCss },
  });
}
