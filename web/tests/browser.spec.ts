import { test, expect } from "@playwright/test";
import { readFileSync, mkdirSync } from "node:fs";
import { execFileSync } from "node:child_process";

test("browser export is accepted by the native verifier and tampering fails", async ({
  page,
}, info) => {
  const failures: string[] = [];
  page.on("pageerror", (e) => failures.push(e.message));
  await page.goto("/");
  await expect(page.locator("#generate")).toBeEnabled();
  await page.locator("#count").fill("2");
  await page.locator("#generate").click();
  await expect(page.locator(".board")).toHaveCount(2);
  const downloading = page.waitForEvent("download");
  await page.getByRole("button", { name: "Export JSON" }).click();
  const download = await downloading;
  const path = info.outputPath("record.json");
  await download.saveAs(path);
  const record = JSON.parse(readFileSync(path, "utf8"));
  expect(record.profile.id).toBe("lab-5of36");
  expect(record.entropy_backend).toBe("web_crypto");
  expect(
    record.boards.every(
      (b: { main: number[] }) =>
        b.main.length === 5 && new Set(b.main).size === 5,
    ),
  ).toBe(true);
  const binary = process.env.DRAWLAB_BIN || "../target/debug/drawlab";
  expect(
    execFileSync(binary, ["verify", path], { encoding: "utf8" }),
  ).toContain("Valid mathematical record");
  await page.locator("#import").setInputFiles(path);
  await expect(page.locator("#verification")).toContainText(
    "Valid: 2 board(s)",
  );
  record.boards[0].main = [1, 1, 2, 3, 4];
  await page.locator("#import").setInputFiles({
    name: "tampered.json",
    mimeType: "application/json",
    buffer: Buffer.from(JSON.stringify(record)),
  });
  await expect(page.locator("#verification")).toContainText("INVALID_RECORD");
  await page.locator("#count").fill("3");
  await expect(
    page.getByRole("button", { name: "Export JSON" }),
  ).toBeDisabled();
  expect(failures).toEqual([]);
});

test("every model works in both modes, exact odds and disabled rules stay visible", async ({
  page,
}) => {
  await page.goto("/");
  await expect(page.locator("#generate")).toBeEnabled();
  for (const id of [
    "lab-6of58",
    "lab-6of52",
    "lab-5of50-20",
    "lab-5of50-16",
    "lab-5of36",
  ]) {
    await page.locator("#profile").selectOption(id);
    for (const mode of ["selection", "draw"]) {
      await page.locator('input[value="' + mode + '"]').check();
      await page.locator("#generate").click();
      await expect(page.locator(".board")).toHaveCount(1);
      const extra =
        (mode === "draw" && id.startsWith("lab-6")) ||
        id.startsWith("lab-5of50")
          ? 1
          : 0;
      await expect(page.locator(".ball.extra")).toHaveCount(extra);
    }
  }
  await expect(page.locator("#coverage li")).toHaveCount(3);
  await expect(page.locator("#profile option")).toHaveCount(5);
  await page.locator("#profile").selectOption("lab-5of50-20");
  expect((await page.locator("#odds").innerText()).replace(/[^0-9]/g, "")).toBe(
    "142375200",
  );
  await page.locator("#count").fill("101");
  await page.locator("#generate").click();
  await expect(page.locator(".board")).toHaveCount(0);
});

test("entropy failure fails closed, without partial results", async ({
  page,
}) => {
  await page.addInitScript(() => {
    Object.defineProperty(Crypto.prototype, "getRandomValues", {
      value: () => {
        throw new Error("Injected entropy failure");
      },
    });
  });
  await page.goto("/");
  await expect(page.locator("#generate")).toBeEnabled();
  await page.locator("#generate").click();
  await expect(page.locator("#status")).toHaveClass(/error/);
  await expect(page.locator(".board")).toHaveCount(0);
  await expect(
    page.getByRole("button", { name: "Export JSON" }),
  ).toBeDisabled();
});

test("mobile layout and keyboard generation remain usable", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/");
  await expect(page.locator("#generate")).toBeEnabled();
  await page.locator("#generate").focus();
  await page.keyboard.press("Enter");
  await expect(page.locator(".board")).toHaveCount(1);
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  mkdirSync("../test-results/screenshots", { recursive: true });
  await page.screenshot({
    path: "../test-results/screenshots/mobile.png",
    fullPage: true,
  });
  await page.setViewportSize({ width: 1440, height: 1200 });
  await page.screenshot({
    path: "../test-results/screenshots/desktop.png",
    fullPage: true,
  });
});

test("match explorer preserves exact counts and excludes extra-ball events", async ({
  page,
}) => {
  await page.goto("/");
  await expect(page.locator("#generate")).toBeEnabled();
  const rows = page.locator("#match-rows tr");
  await expect(rows).toHaveCount(6);
  // Independent combinatorial reference counts for a fixed 5-of-36 selection.
  const counts = ["169911", "157325", "44950", "4650", "155", "1"];
  for (let r = 0; r < counts.length; r++) {
    await expect(rows.nth(r).locator("th")).toHaveText(String(r));
    await expect(rows.nth(r).locator("td").first()).toHaveText(
      counts[r] + " / 376992",
    );
  }
  await expect(page.locator("#match-total")).toHaveText(
    "Exact total: 376992 / 376992 = 1",
  );
  await page.locator("#profile").selectOption("lab-5of50-20");
  const independent20 = await rows.allTextContents();
  await expect(rows.last()).toContainText("1 / 2118760");
  await expect(rows.last()).toContainText("<0.0001%");
  await page.locator("#profile").selectOption("lab-5of50-16");
  expect(await rows.allTextContents()).toEqual(independent20);
  await page.locator('input[value="draw"]').check();
  await page.locator("#count").fill("100");
  expect(await rows.allTextContents()).toEqual(independent20);
  await page.locator("#profile").selectOption("lab-6of58");
  await expect(rows).toHaveCount(7);
  await expect(rows.last()).toContainText("1 / 40475358");
  await expect(page.locator("#match-caption")).toHaveText(
    "Main matches: 6 of 58",
  );
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  await page.locator("#match-heading").scrollIntoViewIfNeeded();
  await page.screenshot({
    path: "../test-results/screenshots/match-explorer-mobile.png",
    fullPage: true,
  });
});

test("tail calculator and CSV expose exact main-only probabilities", async ({
  page,
}, info) => {
  await page.goto("/");
  await expect(page.locator("#minimum-match")).toBeEnabled();
  await expect(page.locator("#threshold-result")).toHaveText(
    "At least 3 main matches: 4806 / 376992",
  );
  await page.locator("#minimum-match").selectOption("0");
  await expect(page.locator("#threshold-result")).toHaveText(
    "At least 0 main matches: 376992 / 376992",
  );
  await page.locator("#profile").selectOption("lab-6of58");
  await page.locator("#minimum-match").selectOption("6");
  await expect(page.locator("#threshold-result")).toHaveText(
    "At least 6 main matches: 1 / 40475358",
  );
  await page.locator("#profile").selectOption("lab-5of36");
  await expect(page.locator("#minimum-match")).toHaveValue("5");
  const pending = page.waitForEvent("download");
  await page
    .getByRole("button", { name: "Export probability table CSV" })
    .click();
  const download = await pending;
  const path = info.outputPath("probabilities.csv");
  await download.saveAs(path);
  const csv = readFileSync(path, "utf8");
  const lines = csv.split("\r\n");
  expect(lines).toHaveLength(13);
  expect(lines[0]).toBe(
    "classification,software_version,profile,event,main_matches,numerator,denominator",
  );
  expect(
    lines.some((line) => line.endsWith(",lab-5of36,at_least,3,4806,376992")),
  ).toBe(true);
  expect(
    lines.some((line) => line.endsWith(",lab-5of36,exactly,3,4650,376992")),
  ).toBe(true);
  const cli = process.env.DRAWLAB_BIN || "../target/debug/drawlab";
  const result = JSON.parse(
    execFileSync(
      cli,
      ["at-least", "--profile", "lab-5of36", "--matches", "3"],
      { encoding: "utf8" },
    ),
  );
  expect(result.numerator).toBe("4806");
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
  await page.locator("#minimum-match").scrollIntoViewIfNeeded();
  await page.screenshot({
    path: "../test-results/screenshots/probability-analysis-mobile.png",
    fullPage: true,
  });
});

test("exact moments and model explanation remain synchronized with CLI", async ({
  page,
}) => {
  await page.goto("/");
  await expect(page.locator("#match-moments")).toHaveText(
    "E[X] = 25 / 36; Var(X) = 24025 / 45360",
  );
  const bin = process.env.DRAWLAB_BIN || "../target/debug/drawlab";
  const output = JSON.parse(
    execFileSync(bin, ["moments", "--profile", "lab-5of36"], {
      encoding: "utf8",
    }),
  );
  expect(output.expected_numerator).toBe("25");
  expect(output.variance_numerator).toBe("24025");
  await page.locator("#profile").selectOption("lab-6of58");
  await expect(page.locator("#match-moments")).toContainText("E[X] = 36 / 58");
  await page.locator("#sampling-research summary").click();
  await expect(page.locator("#sampling-research")).toContainText(
    "partial Fisher",
  );
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true);
});
