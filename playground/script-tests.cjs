// Execute the production scripts against event/observer doubles. These tests
// exercise behavior and disposal; browser-tests.cjs covers actual browser input.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');
const directory = path.join(__dirname, '../src/components');
const source = (name, values = {}) => Object.entries(values).reduce((text, [key, value]) => text.replaceAll(`__${key}__`, JSON.stringify(value)), fs.readFileSync(path.join(directory, name), 'utf8'));

class Target {
  listeners = new Map();
  addEventListener(type, callback, options = {}) {
    if (options.signal?.aborted) return;
    const callbacks = this.listeners.get(type) || new Set();
    callbacks.add(callback); this.listeners.set(type, callbacks);
    options.signal?.addEventListener('abort', () => callbacks.delete(callback), { once: true });
  }
  removeEventListener(type, callback) { this.listeners.get(type)?.delete(callback); }
  fire(type, event = {}) {
    event.target ||= this;
    event.preventDefault ||= () => { event.defaultPrevented = true; };
    event.stopPropagation ||= () => {};
    for (const callback of [...(this.listeners.get(type) || [])]) callback(event);
    return event;
  }
  count() { return [...this.listeners.values()].reduce((sum, callbacks) => sum + callbacks.size, 0); }
}
class Element extends Target {
  dataset = {}; attributes = new Map(); isConnected = true; parentElement = null;
  style = { setProperty() {}, removeProperty() {}, getPropertyValue() { return ''; } };
  children = []; classes = [];
  matches(selector) { return this.classes.some((name) => selector === `.${name}`); }
  closest() { return null; }
  querySelector() { return null; }
  querySelectorAll() { return []; }
  setAttribute(name, value) { this.attributes.set(name, value); }
  getAttribute(name) { return this.attributes.get(name) ?? null; }
  hasAttribute(name) { return this.attributes.has(name); }
  removeAttribute(name) { this.attributes.delete(name); }
  getClientRects() { return [{}]; }
  getBoundingClientRect() { return { left: 0, right: 300, top: 0, bottom: 48, width: 300, height: 48 }; }
}
function environment(elements = {}) {
  const timers = new Map(); let nextTimer = 0;
  const reports = []; let receive;
  const window = Object.assign(new Target(), { matchMedia: () => ({ matches: false }) });
  const document = Object.assign(new Target(), { getElementById: (id) => elements[id], querySelectorAll: () => [], documentElement: new Element() });
  const observers = [];
  class Observer { constructor(callback) { this.callback = callback; this.targets = new Set(); observers.push(this); } observe(target) { this.targets.add(target); } unobserve(target) { this.targets.delete(target); } disconnect() { this.targets.clear(); } }
  const globals = { window, document, Element, Node: Element, AbortController,
    getComputedStyle: (el) => ({ direction: el.rtl ? 'rtl' : 'ltr', fontSize: '16px' }),
    performance: { now: () => 10 },
    setTimeout(callback) { timers.set(++nextTimer, callback); return nextTimer; },
    clearTimeout(id) { timers.delete(id); }, requestAnimationFrame() { return 999; }, cancelAnimationFrame() {},
    ResizeObserver: Observer, MutationObserver: Observer, IntersectionObserver: Observer,
    dioxus: { send: (report) => reports.push(report), recv: () => new Promise((resolve) => { receive = resolve; }) },
  };
  return { ...globals, globals, timers, reports, observers,
    run(text) { this.running = vm.runInNewContext(`(async () => { ${text}\n})()`, globals); return this; },
    async close() { receive?.('dispose'); await this.running; },
  };
}

test('every production script parses as an eval body', () => {
  const AsyncFunction = Object.getPrototypeOf(async function () {}).constructor;
  for (const name of fs.readdirSync(directory).filter((name) => name.endsWith('.js'))) new AsyncFunction(source(name));
});

test('cancelled refreshes do not commit and disposal clears timers/listeners', async () => {
  const root = new Element(); const label = new Element();
  const env = environment({ root, label }).run(source('refresher.js', { ROOT: 'root', LABEL: 'label', THRESHOLD: 48, ELASTIC: 0.42, PULL: 'pull', RELEASE: 'release', REFRESHING: 'refreshing' }));
  const start = () => root.fire('touchstart', { touches: [{ clientX: 0, clientY: 0 }] });
  const move = () => root.fire('touchmove', { touches: [{ clientX: 0, clientY: 120 }], cancelable: true });
  start(); move(); root.fire('touchcancel');
  assert.equal(env.reports.length, 0);
  start(); move(); root.fire('touchend', { touches: [] });
  assert.equal(env.reports.length, 1);
  assert.ok(env.timers.size > 0);
  await env.close();
  assert.equal(root.count() + env.window.count() + env.document.count(), 0);
  assert.equal(env.timers.size, 0);
});

test('a scrollable segment strip places its indicator under the selected button', async () => {
  const properties = {};
  const selected = Object.assign(new Element(), { offsetLeft: 120, offsetWidth: 80 });
  const strip = Object.assign(new Element(), { classes: ['g3-segment-scrollable'], children: [selected], scrollWidth: 1000, clientWidth: 300, scrollLeft: 0 });
  strip.style = { setProperty(name, value) { properties[name] = value; } };
  strip.querySelector = () => selected;
  strip.scrollTo = () => {};
  environment({ strip }).run(source('hscroll.js', { ID: 'strip' }));
  await new Promise((resolve) => setImmediate(resolve));
  assert.equal(properties['--g3-segment-indicator-left'], '120px');
  assert.equal(properties['--g3-segment-indicator-width'], '80px');
  assert.equal('indicatorPlaced' in strip.dataset, true);
});

for (const rtl of [false, true]) test(`horizontal strip disposal removes all global listeners (${rtl ? 'RTL' : 'LTR'})`, async () => {
  const strip = Object.assign(new Element(), { rtl, scrollWidth: 1000, clientWidth: 300, scrollLeft: 0 });
  const env = environment({ strip }).run(source('hscroll.js', { ID: 'strip' }));
  assert.equal(strip.fire('wheel', { deltaX: rtl ? -30 : 30, deltaY: 0, deltaMode: 0 }).defaultPrevented, true);
  strip.fire('pointerdown', { pointerType: 'mouse', button: 0, clientX: 100 });
  env.window.fire('pointermove', { clientX: rtl ? 150 : 50 });
  assert.equal(strip.scrollLeft, rtl ? -50 : 50);
  assert.ok(env.window.count() > 0);
  strip.isConnected = false;
  await env.close();
  assert.equal(env.window.count() + strip.count(), 0);
  assert.ok(env.observers.every((observer) => observer.targets.size === 0));
});

test('infinite-scroll reconfiguration disposes only its own observers', async () => {
  const sentinel = new Element(); sentinel.parentElement = new Element();
  const env = environment({ sentinel }).run(source('infinite_scroll.js', { ID: 'sentinel', MARGIN: 200 }));
  assert.equal(env.observers.length, 3);
  await env.close();
  assert.ok(env.observers.every((observer) => observer.targets.size === 0));
});

test('tooltip descriptions follow replaced controls and preserve app descriptions', async () => {
  const tip = Object.assign(new Element(), { id: 'tip' });
  tip.parentElement = new Element();
  let control = new Element();
  tip.parentElement.querySelector = () => control;
  const first = control;
  const env = environment({ tip }).run(source('tooltip.js', { ID: 'tip' }));
  assert.equal(first.getAttribute('aria-describedby'), 'tip');
  control = new Element(); env.observers[0].callback();
  assert.equal(first.getAttribute('aria-describedby'), null);
  assert.equal(control.getAttribute('aria-describedby'), 'tip');
  control.setAttribute('aria-describedby', 'app-description');
  await env.close();
  assert.equal(control.getAttribute('aria-describedby'), 'app-description');
  assert.equal(env.observers[0].targets.size, 0);
});

test('old tooltip disposal cannot remove a replacement session description', async () => {
  const tip = Object.assign(new Element(), { id: 'first' });
  const control = new Element(); tip.parentElement = new Element();
  tip.parentElement.querySelector = () => control;
  const first = environment({ first: tip }).run(source('tooltip.js', { ID: 'first' }));
  tip.id = 'second';
  const second = environment({ second: tip }).run(source('tooltip.js', { ID: 'second' }));
  first.observers[0].callback();
  await first.close();
  assert.equal(control.getAttribute('aria-describedby'), 'second');
  await second.close();
  assert.equal(control.getAttribute('aria-describedby'), null);
});

test('tab taps never capture and touch drags survive a child capture loss', async () => {
  const root = new Element(); root.dataset.swipe = 'true'; let captures = 0; let picks = 0;
  root.setPointerCapture = () => captures++;
  const tab = (selected) => ({ disabled: false, closest: () => root, getAttribute: (name) => name === 'aria-selected' ? String(selected) : null, click: () => picks++ });
  root.querySelectorAll = () => [tab(true), tab(false)];
  const button = new Element(); button.closest = (selector) => selector.startsWith('.g3-tabs') ? root : null;
  const env = environment().run(source('tabs.js'));
  const down = () => env.document.fire('pointerdown', { target: button, pointerType: 'touch', pointerId: 1, isPrimary: true, clientX: 200, clientY: 100 });
  down(); env.document.fire('pointerup', { pointerId: 1 });
  assert.equal(captures, 0); assert.equal(picks, 0);
  down(); env.document.fire('pointermove', { pointerId: 1, clientX: 180, clientY: 100 });
  env.document.fire('lostpointercapture', { pointerId: 1, target: button });
  env.document.fire('pointermove', { pointerId: 1, clientX: 130, clientY: 100 });
  env.document.fire('pointerup', { pointerId: 1 });
  assert.equal(captures, 1); assert.equal(picks, 1);
  env.window[Symbol.for('g3-ui.tabs')].dispose(); await env.running;
  assert.equal(env.document.count(), 0);
});

test('top-layer presentation preserves toast priority and advances reopened overlays', () => {
  const order = [];
  const layer = (id) => Object.assign(new Element(), { id,
    matches: () => order.includes(id),
    showPopover() { order.push(id); }, hidePopover() { order.splice(order.indexOf(id), 1); },
  });
  const env = environment({ sheet: layer('sheet'), modal: layer('modal'), toast: layer('toast') });
  vm.runInNewContext(source('layers.js'), env.globals);
  const manager = env.window[Symbol.for('g3-ui.layers')];
  manager.present('sheet', true, 0, 0);
  manager.present('toast', true, 0, 1);
  manager.present('modal', true, 0, 0);
  assert.deepEqual(order, ['sheet', 'modal', 'toast']);
  manager.present('sheet', false, 200, 0);
  manager.present('sheet', true, 200, 0);
  assert.deepEqual(order, ['modal', 'sheet', 'toast']);
  for (const id of ['modal', 'sheet', 'toast']) manager.remove(id);
  assert.equal(order.length, 0);
});

test('a confined layer stays inside the visible part of an offset app', () => {
  const properties = {};
  const app = new Element();
  // The shell starts under a 176px page header and runs past a 900px window.
  app.getBoundingClientRect = () => ({ left: 320, right: 710, top: 176, bottom: 1020, width: 390, height: 844 });
  const el = Object.assign(new Element(), { id: 'sheet', matches: () => false, showPopover() {}, hidePopover() {} });
  el.style = { setProperty(name, value) { properties[name] = value; } };
  el.parentElement = Object.assign(new Element(), { closest: (selector) => selector === '.g3-app' ? app : null });
  const env = environment({ sheet: el });
  env.document.documentElement.clientWidth = 1440;
  env.window.innerHeight = 900;
  env.globals.getComputedStyle = () => ({ overflowX: 'hidden', borderTopLeftRadius: '40px' });
  vm.runInNewContext(source('layers.js'), env.globals);
  const manager = env.window[Symbol.for('g3-ui.layers')];
  manager.present('sheet', true, 0, 0);
  assert.equal(el.hasAttribute('data-g3-confined'), true);
  assert.deepEqual(properties, { '--g3-layer-top': '176px', '--g3-layer-left': '320px', '--g3-layer-width': '390px', '--g3-layer-height': '724px', '--g3-layer-radius': '0px' });
  // A visual viewport (pinch zoom, a docked keyboard) narrows it further.
  env.window.visualViewport = Object.assign(new Target(), { offsetLeft: 0, offsetTop: 100, width: 1440, height: 500 });
  manager.remove('sheet');
  manager.present('sheet', true, 0, 0);
  assert.equal(properties['--g3-layer-top'], '176px');
  assert.equal(properties['--g3-layer-height'], '424px');
  env.window.visualViewport.offsetTop = 300;
  env.window.visualViewport.fire('scroll');
  assert.equal(properties['--g3-layer-top'], '300px');
  assert.equal(properties['--g3-layer-height'], '500px');
  manager.remove('sheet');
  assert.equal(env.window.visualViewport.count() + env.window.count() + env.document.count(), 0);
});

test('horizontal tab lists leave vertical arrows to the page', async () => {
  // Roving source remains inline because it is also used for radio controls.
  const rust = fs.readFileSync(path.join(directory, 'keyboard.rs'), 'utf8');
  let script = rust.match(/const ROVING_SCRIPT: &str = r#"([\s\S]*?)"#;/)[1];
  script = script.replaceAll('__ID__', '"list"').replaceAll('__SELECTOR__', '"[role=tab]"').replaceAll('__VERTICAL__', 'false');
  const root = new Element(); root.attributes.set('role', 'tablist');
  let clicks = 0; const first = { getAttribute: () => null }; const second = { getAttribute: () => null, focus() {}, click() { clicks++; } };
  root.querySelectorAll = () => [first, second];
  const env = environment({ list: root }); env.document.activeElement = first; env.run(script);
  assert.equal(root.fire('keydown', { key: 'ArrowDown' }).defaultPrevented, undefined);
  assert.equal(root.fire('keydown', { key: 'ArrowRight' }).defaultPrevented, true);
  assert.equal(clicks, 1);
  await env.close(); assert.equal(root.count(), 0);
});

test('overlay focus initializes without frames and follows live modality', async () => {
  const root = new Element(); root.dataset.g3TrapFocus = 'false';
  const child = Object.assign(new Element(), { tabIndex: 0 });
  const outside = new Element();
  root.contains = (node) => node === root || node === child;
  root.querySelectorAll = () => [child];
  const env = environment({ root });
  for (const el of [root, child, outside]) el.focus = () => { env.document.activeElement = el; };
  env.document.activeElement = outside;
  env.run(source('focus.js', { ID: 'root', ROVING: null }));
  // requestAnimationFrame never fires in this environment, as in a hidden tab.
  for (const callback of env.timers.values()) callback();
  assert.equal(env.document.activeElement, child);
  env.document.activeElement = outside;
  env.document.fire('focusin', { target: outside });
  assert.equal(env.document.activeElement, outside);
  root.dataset.g3TrapFocus = 'true';
  env.document.fire('focusin', { target: outside });
  assert.equal(env.document.activeElement, child);
  const notice = new Element();
  const action = Object.assign(new Element(), { tabIndex: 0 });
  action.closest = (selector) => selector.startsWith('.g3-toast') ? notice : null;
  action.focus = () => { env.document.activeElement = action; };
  env.document.querySelectorAll = () => [action];
  action.focus(); env.document.fire('focusin', { target: action });
  assert.equal(env.document.activeElement, action, 'toast actions stay reachable above modal focus traps');
  assert.equal(env.document.fire('keydown', { target: action, key: 'Tab' }).defaultPrevented, true);
  assert.equal(env.document.activeElement, child);
  await env.close();
  assert.equal(env.document.activeElement, outside);
  assert.equal(root.count() + env.document.count(), 0);
});

for (const rtl of [false, true]) test(`swipe edges and commands use logical sides (${rtl ? 'RTL' : 'LTR'})`, async () => {
  const root = Object.assign(new Element(), { id: 'row', rtl });
  Object.assign(root.dataset, { hasStart: 'true', hasEnd: 'true', startBehavior: 'reveal', endBehavior: 'reveal' });
  root.closest = (selector) => selector === '.g3-swipe-item' ? root : null;
  root.setPointerCapture = () => {};
  const env = environment({ row: root }).run(source('swipe.js'));
  const api = env.window[Symbol.for('g3-ui.swipe')];
  api.command('row', 'start');
  assert.equal(root.dataset.exposed, 'start');
  assert.ok(env.reports.at(-1)[2][0] > 0);
  api.command('row', 'close');
  env.document.fire('pointerdown', { target: root, pointerId: 1, pointerType: 'touch', clientX: 100, clientY: 10 });
  env.document.fire('pointermove', { pointerId: 1, clientX: rtl ? 20 : 180, clientY: 10 });
  env.document.fire('pointerup', { pointerId: 1 });
  assert.equal(root.dataset.exposed, 'start');
  assert.ok(env.reports.at(-1)[2][0] > 0);
  api.dispose(); await env.running;
});

test('responsive scope reports only crossed width classes and disconnects on disposal', async () => {
  const shell = new Element();
  let width = 390;
  shell.getBoundingClientRect = () => ({ width });
  const env = environment({ scope: shell }).run(source('responsive.js', { ID: 'scope' }));
  assert.deepEqual(env.reports, [0]);
  width = 767; env.observers[0].callback();
  assert.deepEqual(env.reports, [0]);
  width = 768; env.observers[0].callback();
  width = 1024; env.observers[0].callback();
  width = 820; env.observers[0].callback();
  width = 390; env.observers[0].callback();
  assert.deepEqual(env.reports, [0, 1, 2, 1, 0]);
  await env.close();
  assert.equal(env.observers[0].targets.size, 0);
});
