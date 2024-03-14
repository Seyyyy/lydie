import { defineConfig } from "vitest/config";

// https://vitejs.dev/config/
export default defineConfig({
  // test src folder by vitest
  test: {
    include: ["src/**/*.spec.ts"],
  },
});
