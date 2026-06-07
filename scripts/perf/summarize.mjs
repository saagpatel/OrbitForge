import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";

const files = {
  bundle: ".perf-results/bundle.json",
  build: ".perf-results/build-time.json",
  assets: ".perf-results/assets.json",
  memory: ".perf-results/memory.json",
  api: ".perf-results/api-summary.json",
};

const summary = {
  capturedAt: new Date().toISOString(),
  metrics: {},
  status: "pass",
  required: [],
};

for (const [key, file] of Object.entries(files)) {
  if (existsSync(file)) {
    try {
      summary.metrics[key] = JSON.parse(readFileSync(file, "utf8"));
    } catch (error) {
      summary.metrics[key] = {
        status: "fail",
        error: `invalid-json: ${error instanceof Error ? error.message : String(error)}`,
      };
      summary.status = "fail";
    }
  } else {
    summary.metrics[key] = { status: "not-run" };
  }
}

const required = (process.env.PERF_REQUIRED_METRICS || "")
  .split(",")
  .map((item) => item.trim())
  .filter(Boolean);
summary.required = required;

for (const key of required) {
  const metric = summary.metrics[key];
  if (!metric) {
    summary.status = "fail";
    continue;
  }
  if (metric.status === "not-run" || metric.status === "fail") {
    summary.status = "fail";
  }
}

mkdirSync(".perf-results", { recursive: true });
writeFileSync(
  ".perf-results/summary.json",
  `${JSON.stringify(summary, null, 2)}\n`,
);
console.log("wrote .perf-results/summary.json");

if (process.env.PERF_ENFORCE_SUMMARY === "1" && summary.status !== "pass") {
  console.error("Perf summary gate failed.");
  process.exit(1);
}
