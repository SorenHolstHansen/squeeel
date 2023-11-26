module.exports = {
  extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended"],
  parser: "@typescript-eslint/parser",
  plugins: ["@typescript-eslint"],
  root: true,
  rules: {
    "no-console": "error",
    "no-var": "error",
  },
  overrides: [
    {
      files: ["examples/**/*.ts"],
      rules: {
        "no-console": "off",
      },
    },
  ],
};
