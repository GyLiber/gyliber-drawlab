import { test, expect } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";

test("published build identity matches the engine and checked-out commit", async ({
  page,
  request,
}) => {
  const response = await request.get("/build-info.json");
  expect(response.ok()).toBe(true);
  const info = await response.json();
  const pkg = JSON.parse(readFileSync("package.json", "utf8"));
  expect(info).toEqual({
    version: pkg.version,
    commit: execFileSync("git", ["rev-parse", "HEAD"], {
      encoding: "utf8",
    }).trim(),
    scope: "laboratory-preview",
  });
  await page.goto("/");
  await expect(page.locator("#version")).toHaveText("v" + info.version);
  await expect(
    page.getByRole("link", { name: "Build identity" }),
  ).toHaveAttribute("href", "./build-info.json");
});
