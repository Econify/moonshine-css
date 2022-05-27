// Based on: https://codesandbox.io/s/4w0070kpv4?file=/src/index.js:0-5821

/* character codes */
let SEMICOLON = 59; /* ; */
let CLOSEBRACES = 125; /* } */
let OPENBRACES = 123; /* { */
let NEWLINE = 10; /* \n */
let CARRIAGE = 13; /* \r */
let TAB = 9; /* \t */
let AT = 64; /* @ */
let SPACE = 32; /*   */
let AND = 38; /* & */

export default function unnest(SELECTOR_PLACEHOLDER, str) {
  let i = 0,
    out = "",
    char = 0,
    context = "",
    activeSelector = SELECTOR_PLACEHOLDER;

  str = activeSelector + "{" + str; //  + "}"; // leave this off on purpose so we don't have to remove it

  while (i < str.length) {
    char = str.charCodeAt(i);
    switch (char) {
      // replace `&` with the active selector
      case AND:
        context += context + activeSelector;
        break;

      // I can't explain this in writing :-/ Anyone?
      case SEMICOLON:
        out += context + str[i];
        context = "";
        break;

      // whatever is in the context is a selector at this point
      case OPENBRACES:
        out +=
          (out.length ? "}" : "") +
          (activeSelector = context.trim()) + // having trouble explaining why this trim works.
          str[i];
        context = "";
        break;

      // hit wall and write what we have in context
      // we dump our net that we've been dragging behind us
      case CLOSEBRACES:
        out += context + str[i];
        context = "";
        break;

      case AT:
        out += context;
        context = str[i];
        break;

      // minify
      case NEWLINE:
      case CARRIAGE:
      case TAB:
      case SPACE:
        // ensure we have nothing important
        if (context.length === 0) break;
      // eslint-disable-no-fallthrough
      default:
        context += str[i];
        break;
    }
    ++i;
  }
  return out;
}
