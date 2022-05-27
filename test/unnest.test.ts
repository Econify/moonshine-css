import unnest from "../src/css/unnest";

describe("unnest", () => {
  it("should not transform regular css", () => {
    const css = `
      color: turquoise;
      @media(min-width: 420px) {
        color: hotpink;
      }
    `;
    expect(unnest(".css-123", css)).toBe(
      [
        ".css-123{color: turquoise;}",
        "@media(min-width: 420px){color: hotpink;}",
      ].join("")
    );
  });

  it("should unnest nested css", () => {
    const css = `
      & .header {
        color: green;
        & .blue {
          color: blue;
          &::placeholder {
            font-size: 12px;
          }
        }
      }
    `;
    expect(unnest(".css-12345", css)).toBe(
      [
        ".css-12345{}",
        ".css-12345 .header{color: green;}",
        ".css-12345 .header .blue{color: blue;}",
        ".css-12345 .header .blue::placeholder{font-size: 12px;}}}",
      ].join("")
    );
  });
});
