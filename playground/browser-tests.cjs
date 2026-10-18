// Run against an already running playground: just browser-test.
// UI automation is separate from the dependency-free production-script tests.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { chromium } = require('playwright');

const origin = process.env.PLAYGROUND_URL || 'http://127.0.0.1:8080';
const output = path.join(__dirname, 'target/browser-tests');
const poll = async (read, expected) => {
  for (let i = 0; i < 100; i++) {
    if (await read() === expected) return;
    await new Promise((resolve) => setTimeout(resolve, 30));
  }
  assert.equal(await read(), expected);
};
// The Mobile preview fits under the header, so a sheet in it is never cut off.
const frameFits = async (page) => {
  const frame = await page.locator('.g3-playground-preview').boundingBox();
  const height = page.viewportSize().height;
  assert.ok(frame.y >= 0 && frame.y + frame.height <= height, `the device frame (${Math.round(frame.y)}-${Math.round(frame.y + frame.height)}) must fit the ${height}px window`);
};
// A stale dx build once served its loader bootstrap twice, mounting two app
// trees with duplicate ids. Every page must hold exactly one.
const singleRoot = async (page) => {
  await page.locator('#main > *').first().waitFor();
  assert.equal(await page.locator('#main > *').count(), 1, '#main must hold exactly one app tree');
};

async function pickViewport(page, name) {
  await page.getByRole('combobox', { name: 'Viewport', exact: true }).click();
  await page.getByRole('option', { name, exact: true }).click();
}

async function configure(page, mode, wide, dark) {
  await singleRoot(page);
  const platform = page.getByRole('radio', { name: mode, exact: true });
  await platform.click();
  await poll(() => platform.getAttribute('aria-checked'), 'true');
  await pickViewport(page, wide ? 'Desktop' : 'Mobile');
  if (dark) {
    await page.getByRole('combobox', { name: 'Theme', exact: true }).click();
    await page.getByRole('option', { name: 'Ember', exact: true }).click();
  }
}

async function main() {
  fs.mkdirSync(output, { recursive: true });
  const browser = await chromium.launch({ headless: true });
  try {
    const responsive = await browser.newContext({ viewport: { width: 1600, height: 1000 }, reducedMotion: 'reduce' });
    try {
      const page = await responsive.newPage();
      const errors = [];
      page.on('pageerror', error => errors.push(String(error)));
      await page.goto(`${origin}/components/responsive`);
      await singleRoot(page);
      // Whether the header is one row or two, the title never sits under the
      // toolbar: a wide toolbar once pushed it below the preview controls.
      const headerTitle = await page.locator('.playground-header .g3-header-title').boundingBox();
      const headerToolbar = await page.locator('.playground-header .g3-header-toolbar').boundingBox();
      const besideToolbar = headerTitle.x + headerTitle.width <= headerToolbar.x + 1;
      const aboveToolbar = headerTitle.y + headerTitle.height <= headerToolbar.y + 1;
      assert.ok(besideToolbar || aboveToolbar, 'the playground title stays clear of its toolbar');
      const draft = page.getByLabel('Draft survives resizing', { exact: true });
      await draft.fill('Keep this draft');
      for (const [preview, size, columns] of [['Mobile', 'compact', 1], ['Tablet', 'medium', 2], ['Landscape phone', 'medium', 2], ['Desktop', 'expanded', 3]]) {
        await pickViewport(page, preview);
        await poll(() => page.locator('.g3-adaptive-layout').getAttribute('data-size'), size);
        assert.equal(await draft.inputValue(), 'Keep this draft', 'preview width must not reset page state');
        // The third column needs an 80rem shell, which the Desktop preset
        // reaches only in a window wide enough to give it that.
        const shellWidth = await page.locator('.g3-playground-preview').evaluate(el => el.getBoundingClientRect().width);
        const expected = columns === 3 && shellWidth < 1280 ? 2 : columns;
        assert.equal(await page.locator('.g3-page-layout').evaluate(el => getComputedStyle(el).gridTemplateColumns.split(' ').length), expected);
        const frame = await page.locator('.g3-playground-preview').boundingBox();
        const ratio = preview === 'Tablet' ? 4 / 3 : preview === 'Landscape phone' ? 844 / 390 : preview === 'Desktop' ? 10 / 7 : 390 / 844;
        assert.ok(Math.abs(frame.width / frame.height - ratio) < 0.01, 'device presets must keep their distinct proportions');
        await page.screenshot({ path: path.join(output, `responsive-${preview}.png`) });
      }
      await pickViewport(page, 'Resize');
      const slider = page.getByRole('slider', { name: 'Preview width (pixels)', exact: true });
      await slider.focus();
      await page.keyboard.press('Home');
      await poll(() => page.locator('.g3-adaptive-layout').getAttribute('data-size'), 'compact');
      await page.keyboard.press('End');
      await poll(() => page.locator('.g3-adaptive-layout').getAttribute('data-size'), 'expanded');
      assert.equal(await draft.inputValue(), 'Keep this draft');
      await page.keyboard.press('Control+k');
      const search = page.getByRole('dialog', { name: 'Search documentation', exact: true });
      await search.waitFor();
      await search.getByRole('searchbox').fill('use_toast');
      await search.getByRole('button', { name: /Overlay hooks/ }).first().click();
      await page.waitForURL('**/docs/hooks-overlays');
      await page.getByRole('banner').getByRole('heading', { name: 'Overlay hooks', exact: true }).waitFor();
      assert.equal(await page.locator('.playground-header .g3-header-toolbar').count(), 1, 'live guides expose preview controls');
      await page.getByRole('button', { name: 'Show toast', exact: true }).click();
      await page.getByText('Saved', { exact: true }).waitFor();
      await page.getByRole('button', { name: 'Theme generator', exact: true }).click();
      await page.getByRole('radio', { name: 'MD', exact: true }).focus();
      await page.keyboard.press('ArrowRight');
      const focus = await page.getByRole('radio', { name: 'iOS', exact: true }).evaluate(el => {
        const style = getComputedStyle(el);
        return { visible: el.matches(':focus-visible'), extent: parseFloat(style.outlineWidth) + parseFloat(style.outlineOffset) };
      });
      assert.ok(focus.visible && focus.extent <= 0, 'keyboard focus must fit inside the clipped segment');
      await page.getByRole('radio', { name: 'iOS', exact: true }).click();
      const iosGap = await page.locator('.playground-header .g3-header-toolbar').evaluate(toolbar => toolbar.getBoundingClientRect().bottom - toolbar.querySelector('[role="radiogroup"]').getBoundingClientRect().bottom);
      assert.ok(iosGap >= 8, 'iOS toolbar pills must have bottom spacing');
      await page.getByRole('radio', { name: 'MD', exact: true }).click();
      const mdGap = await page.locator('.playground-header .g3-header-toolbar').evaluate(toolbar => toolbar.getBoundingClientRect().bottom - toolbar.querySelector('[role="radiogroup"]').getBoundingClientRect().bottom);
      assert.ok(Math.abs(mdGap) < 1, 'MD toolbar underline must remain flush');
      await page.getByLabel('Accent CSS color', { exact: true }).fill('#7c3aed');
      await poll(() => page.locator('[aria-label="Custom theme preview"] .g3-app').evaluate(el => getComputedStyle(el).getPropertyValue('--g3-color-accent').trim()), '#7c3aed');
      assert.ok((await page.locator('.theme-generator-export pre').textContent()).includes('accent: "#7c3aed".into()'));
      await page.getByRole('button', { name: 'Use throughout playground', exact: true }).click();
      await page.getByRole('button', { name: 'Card', exact: true }).click();
      await poll(() => page.locator('.g3-app-shell').first().evaluate(el => getComputedStyle(el).getPropertyValue('--g3-color-accent').trim()), '#7c3aed');
      // Nested grids must keep their own policy inside expanded containers.
      const nestedPage = await responsive.newPage();
      await nestedPage.setContent('<div class="g3-app" style="width:1280px"><div class="g3-grid" style="--g3-grid-columns:repeat(1,minmax(0,1fr));--g3-grid-columns-wide:repeat(2,minmax(0,1fr));--g3-grid-columns-medium:repeat(3,minmax(0,1fr));--g3-grid-columns-expanded:repeat(4,minmax(0,1fr))"><div id="nested-grid" class="g3-grid" style="--g3-grid-columns:repeat(1,minmax(0,1fr))"><div>One readable cell</div></div></div></div>');
      await nestedPage.addStyleTag({ content: fs.readFileSync(path.join(__dirname, '../assets/g3-ui.css'), 'utf8') });
      assert.equal(await nestedPage.locator('.g3-app > .g3-grid').evaluate(el => getComputedStyle(el).gridTemplateColumns.split(' ').length), 4);
      assert.equal(await nestedPage.locator('#nested-grid').evaluate(el => getComputedStyle(el).gridTemplateColumns.split(' ').length), 1);
      await nestedPage.close();
      assert.deepEqual(errors, []);
    } finally { await responsive.close(); }
    // An app that starts below other content and runs past the window keeps
    // its bottom sheet on screen: the confined layer is clipped to the viewport.
    const offset = await browser.newContext({ viewport: { width: 1440, height: 900 }, reducedMotion: 'reduce' });
    try {
      const page = await offset.newPage();
      const errors = [];
      page.on('pageerror', error => errors.push(String(error)));
      await page.goto(`${origin}/components/alert`);
      await configure(page, 'MD', false, false);
      await frameFits(page);
      await page.addStyleTag({ content: '.g3-playground-preview.g3-playground-preview { width: 390px; height: 1400px; max-height: none; aspect-ratio: auto; }' });
      await page.getByRole('button', { name: 'Stacked overlays', exact: true }).click();
      const shell = await page.locator('.g3-playground-preview').boundingBox();
      assert.ok(shell.y > 0 && shell.y + shell.height > 900, 'the shell must start below the top and end past the window');
      const confirm = page.getByRole('button', { name: 'Confirm above sheet', exact: true });
      await confirm.waitFor();
      // The whole sheet rests on the window's bottom edge, not the shell's.
      const sheet = page.locator('.g3-sheet-bottom[data-state="open"]');
      await poll(async () => {
        const box = await sheet.boundingBox();
        return box.y >= 0 && Math.abs(box.y + box.height - 900) < 1;
      }, true);
      assert.equal(await confirm.evaluate(el => {
        const r = el.getBoundingClientRect();
        return el.contains(document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2));
      }), true, 'the sheet button must be reachable on screen');
      await page.screenshot({ path: path.join(output, 'offset-shell.png') });
      assert.deepEqual(errors, []);
      console.log('PASS offset shell: sheet stays inside the viewport');
    } finally { await offset.close(); }
    for (const mode of ['MD', 'iOS']) for (const wide of [false, true]) for (const dark of [false, true]) {
      const label = `${mode}-${wide ? 'wide' : 'phone'}-${dark ? 'dark' : 'light'}`;
      const context = await browser.newContext({ viewport: wide ? { width: 1440, height: 900 } : { width: 390, height: 844 }, hasTouch: true, reducedMotion: 'reduce' });
      const page = await context.newPage();
      const errors = [];
      page.on('pageerror', (error) => errors.push(String(error)));
      try {
        await page.goto(`${origin}/components/alert`);
        await configure(page, mode, wide, dark);
        if (!wide) await frameFits(page);
        await page.getByRole('button', { name: 'Stacked overlays', exact: true }).click();
        await page.getByRole('button', { name: 'Confirm above sheet', exact: true }).click();
        const alert = page.getByRole('alertdialog', { name: 'Leave round?' });
        await alert.waitFor();
        await poll(() => alert.evaluate((el) => el.contains(document.activeElement)), true);
        const aboveSheet = await alert.evaluate((el) => {
          const r = el.getBoundingClientRect();
          return el.contains(document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2));
        });
        assert.equal(aboveSheet, true, 'alert must receive hits above the sheet');
        await page.screenshot({ path: path.join(output, `${label}-nested.png`) });
        await page.keyboard.press('Escape');
        await poll(() => alert.isVisible(), false);
        assert.equal(await page.locator('.g3-modal-overlay[data-state="closed"]').evaluate((el) => getComputedStyle(el).pointerEvents), 'none');
        const sheetButton = page.getByRole('button', { name: 'Confirm above sheet', exact: true });
        assert.equal(await sheetButton.isVisible(), true);
        await poll(() => sheetButton.evaluate((el) => el === document.activeElement), true);
        await page.getByRole('button', { name: 'Show toast', exact: true }).click();
        await page.getByText('Toast above sheet', { exact: true }).waitFor();
        await page.keyboard.press('Escape');

        await page.goto(`${origin}/components/tabs`);
        await configure(page, mode, wide, dark);
        const action = page.getByRole('button', { name: 'Panel action: 0', exact: true });
        const box = await action.boundingBox();
        assert.ok(box);
        await page.touchscreen.tap(box.x + box.width / 2, box.y + box.height / 2);
        await page.getByRole('button', { name: 'Panel action: 1', exact: true }).waitFor();
        const scores = page.getByRole('tab', { name: 'Scores', exact: true });
        await scores.focus();
        await page.keyboard.press('ArrowRight');
        await poll(() => page.getByRole('tab', { name: 'Players', exact: true }).getAttribute('aria-selected'), 'true');
        await page.keyboard.press('ArrowLeft');
        await poll(() => scores.getAttribute('aria-selected'), 'true');
        await page.screenshot({ path: path.join(output, `${label}-tabs.png`) });
        assert.deepEqual(errors, []);
        console.log(`PASS ${label}: nested overlay, focus return, toast, touch tap, keyboard tabs`);
      } catch (error) {
        await page.screenshot({ path: path.join(output, `${label}-failure.png`) }).catch(() => {});
        throw error;
      } finally { await context.close(); }
    }
  } finally { await browser.close(); }
}
main().catch((error) => { console.error(error); process.exitCode = 1; });
