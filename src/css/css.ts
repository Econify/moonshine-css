import type { CSS, LinariaClassName } from "../types";
import unnest from "./unnest";

const clean = (s) => s.replace(/(\n\s*)/g, "");
const cleanWhitespace = (s) => s.replace(/\s+/g, " ");
const hashCode = (str: string): string => {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = (hash << 5) - hash + str.charCodeAt(i);
    hash = hash & hash;
  }
  return hash.toString(36);
};

const css: CSS = function (strings, ...rest) {
  const body = strings.map((s, i) => clean(s) + (rest[i] || "")).join("");
  const id = `css_${hashCode(cleanWhitespace(body))}`;

  // Temporary hack to support css in Next.js
  if (typeof window !== "undefined") {
    const style = document.createElement("style");
    style.innerHTML = unnest(`.${id}`, body);
    document.getElementsByTagName("head")[0].appendChild(style);
  }

  return id as LinariaClassName;
};

export default css;
