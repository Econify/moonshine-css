import { switchProp } from "../src/switchProp/switchProp";

test("switchProp optionally allows function accessor", () => {
  const props = {
    variant: "primary",
    theme: {
      white: "white",
      black: "black",
    },
  };

  const getVariantStyles = switchProp<typeof props>("variant", {
    primary: () => ["c-white"],
    secondary: ["c-black"],
  });

  const primaryStyles = getVariantStyles(props);
  expect(primaryStyles[0]).toBe("c-white");

  props.variant = "secondary";
  const secondaryStyles = getVariantStyles(props);
  expect(secondaryStyles[0]).toBe("c-black");
});
