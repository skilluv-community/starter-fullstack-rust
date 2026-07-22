import { expect, test } from '@playwright/test';

test('home page renders title and greeting button', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByRole('heading', { level: 1 })).toBeVisible();
  await expect(page.getByRole('button', { name: /bonjour|hello/i })).toBeVisible();
});
