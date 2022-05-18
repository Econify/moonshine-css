import type { LinariaClassName } from "../types";

export default function untyped(a: string): LinariaClassName {
  return a as LinariaClassName;
}
