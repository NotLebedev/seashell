import js from "@eslint/js";
import ts from "typescript-eslint";
import * as tsParser from "@typescript-eslint/parser";
import eslintPluginPrettierRecommended from "eslint-plugin-prettier/recommended";

export default [
  {
    languageOptions: {
      // See https://gjs.guide/guides/gjs/style-guide.html
      globals: {
        ARGV: "readonly",
        Debugger: "readonly",
        GIRepositoryGType: "readonly",
        globalThis: "readonly",
        imports: "readonly",
        Intl: "readonly",
        log: "readonly",
        logError: "readonly",
        pkg: "readonly",
        print: "readonly",
        printerr: "readonly",
        window: "readonly",
        TextEncoder: "readonly",
        TextDecoder: "readonly",
        console: "readonly",
        setTimeout: "readonly",
        setInterval: "readonly",
        clearTimeout: "readonly",
        clearInterval: "readonly",

        // GNOME Shell Only
        global: "readonly",
        _: "readonly",
        C_: "readonly",
        N_: "readonly",
        ngettext: "readonly",
      },
      parserOptions: {
        ecmaVersion: 2022,
        sourceType: "module",
      },
    },
  },
  js.configs.recommended,
  ...ts.configs.recommended,
  {
    files: ["**/*.{ts,tsx}"],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        project: "tsconfig.json",
      },
    },
  },
  eslintPluginPrettierRecommended,
];
