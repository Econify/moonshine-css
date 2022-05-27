export type TVariableMap = { [key: string]: string };

export default function cssFromVarMap(setRootVariables: TVariableMap): string {
  const rules = Object.keys(setRootVariables)
    .map((name) => `--${name}: ${setRootVariables[name]};`)
    .join("");

  return `:root { ${rules} }`;
}
