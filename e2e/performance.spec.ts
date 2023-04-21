import { test, expect } from "@playwright/test";

test.describe("パフォーマンステスト", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:4173/");
    await page.getByText("image analyzed").innerHTML();
  });

  test(`calc usage rate time`, async ({ page, defaultBrowserType }) => {
    const checkNum = 5;
    let result_avarage = 0;
    for (let i = 0; i < checkNum; i++) {
      await page.goto("http://localhost:4173/");
      await page.getByText("image analyzed").innerHTML();
      const result = await page.locator("div#time").allInnerTexts();
      result_avarage += parseFloat(result[0]);
    }

    const ms = result_avarage / checkNum;
    switch (defaultBrowserType) {
      case "chromium":
        await expect(ms).toBeLessThan(100);
        break;
      case "firefox":
        await expect(ms).toBeLessThan(200);
        break;
      case "webkit":
        await expect(ms).toBeLessThan(200);
        break;
    }
  });
});

// test("パフォーマンスログ出力", async ({ page, context }) => {
//   // start trace
//   await context.tracing.start({ screenshots: true, snapshots: true });

//   await page.goto("http://localhost:4173/");
//   await page.getByRole("button", { name: "Click me" }).click();

//   const testPerf = await page
//     .getByText("image analyzed")
//     .innerHTML({ timeout: 100 }); // 100ms以内に計算結果が表示されることを期待する
//   await expect(testPerf).toBe("image analyzed");

//   // stop trace
//   await context.tracing.stop({ path: "trace.zip" });
// });
