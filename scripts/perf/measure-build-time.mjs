import { spawnSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";

const npmExecPath = process.env.npm_execpath;
if (!npmExecPath) {
  console.error("npm_execpath is not set; run this script through pnpm, npm, or yarn.");
  process.exit(1);
}

const runs = Math.max(1, Number(process.env.PERF_BUILD_RUNS || 2));
const durations = [];

for (let i = 0; i < runs; i += 1) {
  const start = Date.now();
  const result = spawnSync(process.execPath, [npmExecPath, "run", "build"], {
    stdio: "inherit",
  });
  const end = Date.now();
  durations.push(end - start);
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

const sorted = [...durations].sort((a, b) => a - b);
const median = sorted[Math.floor(sorted.length / 2)];

mkdirSync(".perf-results", { recursive: true });
writeFileSync(
  ".perf-results/build-time.json",
  JSON.stringify(
    {
      buildMs: median,
      runs,
      samplesMs: durations,
      capturedAt: new Date().toISOString(),
      command: "npm_execpath run build",
    },
    null,
    2,
  ),
);
