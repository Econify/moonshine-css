import cx from "../src/css/classnames";

test("classnames ", () => {
  expect(cx("f")).toBe("f");
  expect(cx("f", "b")).toBe("f b");
  expect(cx("m1", "p1")).toBe("m1 p1");
  expect(cx(["m1", "p1"])).toBe("m1 p1");
  expect(cx(null, false, "b", undefined, "")).toBe("b");
  expect(cx("f", "b", { c: true, d: false })).toBe("f b c");
  expect(cx("f", ["mx1"], { fgb: true, d: false })).toBe("f mx1 fgb");
  expect(cx("m1", { p1: true })).toBe("m1 p1");
  expect(cx({ m1: true }, { p1: true })).toBe("m1 p1");
  expect(cx({ m1: true, p1: true })).toBe("m1 p1");
  expect(cx("m1", { p1: 1, duck: 0 }, "baz", { q: 1 })).toBe("m1 p1 baz q");
  expect(cx(null, false, "b", undefined, 0, 1, { baz: null }, "")).toBe("b 1");
});
