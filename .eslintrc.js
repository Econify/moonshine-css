module.exports = {
  root: true,
  parser: "@typescript-eslint/parser",
  extends: [],
  plugins: ["@typescript-eslint", "react", "prettier", "import"],
  extends: ["prettier"],
  overrides: [
    {
      files: ["src/*.ts"],
      parserOptions: {
        project: "./tsconfig.json",
        tsconfigRootDir: __dirname,
      },
      extends: [
        "eslint:recommended",
        "plugin:@typescript-eslint/eslint-recommended",
        "plugin:@typescript-eslint/recommended",
        "plugin:react/recommended",
        "plugin:import/typescript",
        "prettier",
      ],
    },
  ],
  env: {
    browser: true,
    node: true,
  },
  settings: {
    react: {
      version: "18",
    },
  },
  ignorePatterns: [
    "dist",
    "**/node_modules",
    ".vscode",
    "coverage",
    ".eslintrc.js",
    "vite.config.ts",
  ],
};
