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
