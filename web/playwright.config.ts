import { defineConfig } from "@playwright/test";
export default defineConfig({
  testDir: "./tests",
  workers: 1,
  retries: 0,
  use: {
    baseURL: "http://127.0.0.1:4173",
    browserName: "chromium",
    launchOptions: process.env.DRAWLAB_CHROMIUM
      ? {
          executablePath: process.env.DRAWLAB_CHROMIUM,
          args: JSON.parse(process.env.DRAWLAB_CHROMIUM_ARGS || "[]"),
        }
      : {},
    trace: "retain-on-failure",
  },
  outputDir: "../test-results/browser",
  webServer: {
    command: "npm run preview -- --port 4173 --strictPort",
    url: "http://127.0.0.1:4173",
    reuseExistingServer: false,
  },
});
