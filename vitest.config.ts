import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "node",
    include: ["src/**/*.test.ts"],
    coverage: {
      provider: "v8",
      reporter: ["text", "json-summary"],
      include: ["src/missions/**/*.ts"],
      thresholds: {
        lines: 85,
        statements: 85,
        functions: 90,
        branches: 50,
      },
    },
  },
});
