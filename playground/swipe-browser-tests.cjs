// Isolated browser input regression: no backend or running playground needed.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { chromium } = require('playwright');

const script = fs.readFileSync(process.env.SWIPE_SCRIPT || path.join(__dirname, '../src/components/swipe.js'), 'utf8');
const stylesheet = fs.readFileSync(path.join(__dirname, '../assets/g3-ui.css'), 'utf8');
const refresher = fs.readFileSync(path.join(__dirname, '../src/components/refresher.js'), 'utf8')
  .replaceAll('__ROOT__', '"refresh"').replaceAll('__LABEL__', '"refresh-label"')
  .replaceAll('__THRESHOLD__', '48').replaceAll('__ELASTIC__', '0.42')
  .replaceAll('__PULL__', '"pull"').replaceAll('__RELEASE__', '"release"').replaceAll('__REFRESHING__', '"refreshing"');
const output = path.join(__dirname, '../target/swipe-browser-tests');

(async () => {
  fs.mkdirSync(output, { recursive: true });
  const browser = await chromium.launch({ headless: true });
  try {
    for (const mode of ['md', 'ios']) {
      for (const width of [390, 1440]) {
        const context = await browser.newContext({ viewport: { width, height: width === 390 ? 844 : 900 }, hasTouch: true, isMobile: true });
        try {
          const page = await context.newPage();
          const errors = [];
          page.on('pageerror', (error) => errors.push(error.message));
          await page.setContent(`<meta name="viewport" content="width=device-width, initial-scale=1"><div class="g3-app g3-app-shell" data-g3-mode="${mode}" style="height:100vh"><div class="g3-content-scroll" style="height:100%;overflow:auto">${Array.from({ length: 16 }, (_, i) => `<div id="row-${i}" class="g3-swipe-item" data-has-start="true" data-has-end="true" data-start-behavior="activate" data-end-behavior="reveal"><div class="g3-swipe-actions g3-swipe-actions-start"><button class="g3-swipe-action">Queue</button></div><div class="g3-swipe-actions g3-swipe-actions-end"><button class="g3-swipe-action">Save</button></div><div class="g3-swipe-content"><button style="display:block;width:100%;height:180px" aria-label="Open card ${i}">Card ${i}</button></div></div>`).join('')}</div></div>`);
          await page.addStyleTag({ content: stylesheet });
          await page.evaluate(() => {
            window.reports = [];
            window.taps = 0;
            window.styleReads = 0;
            window.dioxus = { send: (report) => window.reports.push(report) };
            const original = window.getComputedStyle;
            window.getComputedStyle = (...args) => { window.styleReads++; return original(...args); };
            document.querySelector('#row-0 button[aria-label]').addEventListener('click', () => window.taps++);
          });
          await page.addScriptTag({ content: `(async () => { ${script}\n })()` });
          const session = await context.newCDPSession(page);
          const touch = (type, x = 120, y = 80) => session.send('Input.dispatchTouchEvent', {
            type, touchPoints: type === 'touchEnd' || type === 'touchCancel' ? [] : [{ x, y, id: 1 }],
          });
          await touch('touchStart');
          const downReads = await page.evaluate(() => window.styleReads);
          for (let x = 132; x <= 240; x += 6) await touch('touchMove', x);
          await touch('touchEnd');
          assert.equal(await page.evaluate(() => window.styleReads), downReads, 'drag and release never force computed style');
          assert.equal(await page.evaluate(() => window.reports.filter(([, kind]) => kind === 'activate').length), 1);
          // A distinct touch immediately following the drag must not be eaten
          // by the compatibility-click guard, even while the row is settling.
          await page.touchscreen.tap(240, 80);
          assert.equal(await page.evaluate(() => window.taps), 1, 'rapid tap after a swipe reaches its original button');
          await page.waitForTimeout(450);
          await touch('touchStart');
          await touch('touchMove', 240);
          await touch('touchCancel');
          assert.equal(await page.evaluate(() => window.reports.filter(([, kind]) => kind === 'activate').length), 1);
          await page.waitForTimeout(450);
          await page.screenshot({ path: path.join(output, `${mode}-${width}.png`) });
          await touch('touchStart', 180, 500);
          for (let y = 480; y >= 180; y -= 20) await touch('touchMove', 180, y);
          await touch('touchEnd');
          assert.ok(await page.locator('.g3-content-scroll').evaluate((element) => element.scrollTop) > 0, 'vertical intent stays a native scroll');
          await page.setContent(`<meta name="viewport" content="width=device-width, initial-scale=1"><div class="g3-app g3-app-shell" data-g3-mode="${mode}" style="height:100vh"><div class="g3-content-scroll" style="height:100%;overflow:auto"><div class="g3-refresher" id="refresh" data-refreshing="true"><div class="g3-refresher-indicator"><span class="g3-refresher-spinner"></span><span id="refresh-label">Pull</span></div><div class="g3-refresher-content"><p>Feed</p><div style="height:1500px"></div></div></div></div></div>`);
          await page.addStyleTag({ content: stylesheet });
          await page.evaluate(() => {
            window.refreshes = 0;
            window.dioxus = { send: () => {
              window.refreshes++;
              document.getElementById('refresh').dataset.refreshing = 'true';
            }, recv: () => new Promise(() => {}) };
          });
          await page.addScriptTag({ content: `(async () => { ${refresher}\n })()` });
          const indicator = page.locator('.g3-refresher-indicator');
          await page.waitForFunction(() => getComputedStyle(document.querySelector('.g3-refresher-indicator')).opacity === '0');
          assert.equal(await indicator.evaluate((node) => getComputedStyle(node).opacity), '0');
          assert.equal(await page.locator('.g3-refresher-content').evaluate((node) => getComputedStyle(node).transform), 'none');
          await page.locator('#refresh').evaluate((node) => { node.dataset.refreshing = 'false'; });
          await touch('touchStart', 180, 100);
          for (let y = 105; y <= 220; y += 5) await touch('touchMove', 180, y);
          await touch('touchEnd');
          assert.equal(await page.evaluate(() => window.refreshes), 1);
          assert.equal(await page.locator('#refresh').getAttribute('data-state'), 'refreshing');
          await page.locator('#refresh').evaluate((node) => { node.dataset.refreshing = 'false'; });
          await page.waitForFunction(() => document.getElementById('refresh').dataset.state === 'idle');
          assert.deepEqual(errors, []);
          console.log(`PASS ${mode} ${width}: trusted swipe, rapid tap, cancellation, native scroll, quiet background refresh, explicit pull`);
        } finally { await context.close(); }
      }
    }
  } finally { await browser.close(); }
})().catch((error) => { console.error(error); process.exitCode = 1; });
