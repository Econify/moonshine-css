import React from "react";
import { cx, styled, box, css, untyped } from "@econify/moonshine-css";

const Box1 = styled.div("bg-blue6");
const box4styles = css`
  outline: 2px solid white;
`;

const flex = css`
  display: flex;
  gap: 1rem;
  & > div {
    padding: 1rem;
  }
`;

export default function Example() {
  return (
    <div className={flex}>
      <Box1>Box 1</Box1>
      <box.div cx={["bg-blue6"]}>Box 2</box.div>
      <div className={cx("bg-blue6", untyped(box4styles))}>Box 3</div>
    </div>
  );
}
