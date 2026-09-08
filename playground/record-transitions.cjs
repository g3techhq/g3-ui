const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");
const { chromium } = require("playwright");

async function main() {
    const socialPreviewOnly = process.argv.includes("--social-preview");
    const captureDirectory = path.resolve(
        __dirname,
        `target/transition-recording/frames-${process.pid}`,
    );
    const mediaDirectory = path.resolve(__dirname, "../docs/media");
    const playgroundOrigin = process.env.PLAYGROUND_URL || "http://127.0.0.1:8080";
    const demoAppUrl = new URL("/components/demo-app", playgroundOrigin).toString();
    const showcaseUrl = new URL("/transitions", playgroundOrigin).toString();
    fs.mkdirSync(mediaDirectory, { recursive: true });

    let browser;
    try {
        browser = await chromium.launch({ channel: "chrome", headless: true });
    } catch {
        browser = await chromium.launch({ headless: true });
    }
    const context = await browser.newContext({
        viewport: socialPreviewOnly
            ? { width: 1440, height: 1200 }
            : { width: 1440, height: 1100 },
    });
    const page = await context.newPage();

    await page.goto(demoAppUrl, { waitUntil: "domcontentloaded" });
    if (socialPreviewOnly) {
        await captureSocialPreview(page, context, mediaDirectory);
        await context.close();
        await browser.close();
        console.log(`1280x640 four-screen social preview captured from ${demoAppUrl}`);
        return;
    }

    const demoDevice = page.locator(".playground-viewport-mobile .g3-playground-preview").first();
    await demoDevice.waitFor({ state: "visible", timeout: 30_000 });
    await page.waitForTimeout(700);
    await demoDevice.screenshot({ path: path.join(mediaDirectory, "playground.png") });
    if (process.argv.includes("--demo-only")) {
        await context.close();
        await browser.close();
        console.log(`Demo App device captured from ${demoAppUrl}`);
        return;
    }

    await page.goto(showcaseUrl, { waitUntil: "domcontentloaded" });
    const iosTab = page.getByRole("tab", { name: "iOS" });
    await iosTab.waitFor({ state: "visible", timeout: 30_000 });
    await iosTab.click();
    await page.waitForTimeout(700);
    const transitionDevice = page.locator(".transition-showcase-preview").first();
    await transitionDevice.waitFor({ state: "visible", timeout: 30_000 });
    const transitionClip = await transitionDevice.boundingBox();
    if (!transitionClip) {
        throw new Error("Could not measure the route-transition device preview");
    }
    await page.screenshot({
        path: path.join(mediaDirectory, "route-transitions-playground.png"),
        clip: transitionClip,
    });

    fs.mkdirSync(captureDirectory, { recursive: true });
    let frame = 0;
    let recording = true;
    const startedAt = Date.now();
    const capture = (async () => {
        while (recording && Date.now() - startedAt < 35_000) {
            await page.screenshot({
                path: path.join(captureDirectory, `frame-${String(frame).padStart(4, "0")}.png`),
                clip: transitionClip,
            });
            frame += 1;
            const wait = Math.max(0, startedAt + frame * 80 - Date.now());
            await page.waitForTimeout(wait);
        }
    })();

    await page.waitForTimeout(900);
    const playButton = page.getByRole("button", { name: "Play every route transition" });
    await playButton.click();
    await page.waitForFunction(
        () => document.querySelector('[aria-label="Play every route transition"]')?.disabled,
    );
    await page.waitForFunction(
        () => !document.querySelector('[aria-label="Play every route transition"]')?.disabled,
        undefined,
        { timeout: 32_000 },
    );
    await page.waitForTimeout(1_500);
    recording = false;
    await capture;
    await context.close();
    await browser.close();

    encode(captureDirectory, mediaDirectory, "route-transitions-demo.webp", [
        "-loop", "0", "-c:v", "libwebp",
        "-quality", "58", "-compression_level", "5", "-an",
    ]);
    encode(captureDirectory, mediaDirectory, "route-transitions-demo.mp4", [
        "-c:v", "libx264", "-preset", "slow", "-crf", "24", "-pix_fmt", "yuv420p",
        "-movflags", "+faststart",
    ]);
    try {
        fs.rmSync(captureDirectory, {
            recursive: true,
            force: true,
            maxRetries: 5,
            retryDelay: 100,
        });
    } catch (error) {
        console.warn(`Could not remove temporary frames: ${error.message}`);
    }
    console.log(`${frame} browser-composited frames captured from ${showcaseUrl}`);
}

async function captureSocialPreview(page, context, mediaDirectory) {
    const device = page.locator(".playground-viewport-mobile .g3-playground-preview").first();
    await device.waitFor({ state: "visible", timeout: 30_000 });
    await page.waitForTimeout(700);

    const captures = [await device.screenshot()];
    await page.getByRole("tab", { name: "Discover", exact: true }).click();
    await page.waitForTimeout(500);
    captures.push(await device.screenshot());

    await page.getByRole("button", { name: "More filters", exact: true }).click();
    await page.waitForTimeout(500);
    captures.push(await device.screenshot());

    await page.getByRole("button", { name: /^Apply/ }).click();
    await page.waitForTimeout(350);
    await page.getByRole("tab", { name: "Profile", exact: true }).click();
    await page.waitForTimeout(500);
    captures.push(await device.screenshot());

    const composite = await context.newPage();
    await composite.setViewportSize({ width: 1280, height: 640 });
    const images = captures
        .map(
            (capture, index) =>
                `<img src="data:image/png;base64,${capture.toString("base64")}" alt="Demo App screen ${index + 1}">`,
        )
        .join("");
    await composite.setContent(`
        <!doctype html>
        <html>
            <head>
                <style>
                    * { box-sizing: border-box; }
                    html, body {
                        width: 1280px;
                        height: 640px;
                        margin: 0;
                        overflow: hidden;
                    }
                    body {
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        gap: 18px;
                        background:
                            radial-gradient(circle at 50% 0%, #f8fbff 0%, transparent 48%),
                            linear-gradient(135deg, #dbeafe 0%, #eef5ff 55%, #dbeafe 100%);
                    }
                    img {
                        display: block;
                        width: auto;
                        height: 608px;
                        filter: drop-shadow(0 12px 14px rgba(30, 64, 175, 0.18));
                    }
                </style>
            </head>
            <body>${images}</body>
        </html>
    `);
    await composite.waitForFunction(() => Array.from(document.images).every((image) => image.complete));
    await composite.screenshot({ path: path.join(mediaDirectory, "social-preview.png") });
    await composite.close();
}

function encode(captureDirectory, mediaDirectory, fileName, codecArguments) {
    const result = spawnSync("ffmpeg", [
        "-y", "-loglevel", "error", "-framerate", "12.5", "-i",
        path.join(captureDirectory, "frame-%04d.png"),
        ...codecArguments,
        path.join(mediaDirectory, fileName),
    ], { stdio: "inherit" });
    if (result.status !== 0) {
        throw new Error(`FFmpeg could not create ${fileName}. Is it available on PATH?`);
    }
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
