import typescript from "@rollup/plugin-typescript";

const esmConfig = () => {
  return {
    input: "src/index.ts",
    output: {
      dir: "dist/esm",
      format: "esm",
    },
    plugins: [
      typescript({
        declaration: true,
        emitDeclarationOnly: true,
        outDir: "dist/esm",
        exclude: ["*/**/*.spec.ts"],
      }),
    ],
  };
};

const commonjsConfig = () => {
  return {
    input: "src/index.ts",
    output: {
      dir: "dist/cjs",
      format: "cjs",
    },
    plugins: [
      typescript({
        declaration: true,
        emitDeclarationOnly: true,
        outDir: "dist/cjs",
        exclude: ["*/**/*.spec.ts"],
      }),
    ],
  };
};

export default [esmConfig(), commonjsConfig()];
