const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");
const { chromium } = require("playwright");

async function main() {
    const captureDirectory = path.resolve(__dirname, "target/transition-recording/frames");
    const mediaDirectory = path.resolve(__dirname, "../docs/media");
    const playgroundOrigin = process.env.PLAYGROUND_URL || "http://127.0.0.1:8080";
    const showcaseUrl = new URL("/transitions", playgroundOrigin).toString();
    fs.rmSync(captureDirectory, { recursive: true, force: true });
    fs.mkdirSync(captureDirectory, { recursive: true });
    fs.mkdirSync(mediaDirectory, { recursive: true });

    let browser;
    try {
        browser = await chromium.launch({ channel: "chrome", headless: true });
    } catch {
        browser = await chromium.launch({ headless: true });
    }
    const context = await browser.newContext({ viewport: { width: 1440, height: 1100 } });
    const page = await context.newPage();
    await page.goto(showcaseUrl, { waitUntil: "domcontentloaded" });
    const iosTab = page.getByRole("tab", { name: "iOS" });
    await iosTab.waitFor({ state: "visible", timeout: 30_000 });
    await iosTab.click();
    await page.waitForTimeout(700);
    await page.screenshot({ path: path.join(mediaDirectory, "route-transitions-playground.png") });

    let frame = 0;
    let recording = true;
    const startedAt = Date.now();
    const capture = (async () => {
        while (recording && Date.now() - startedAt < 35_000) {
            await page.screenshot({
                path: path.join(captureDirectory, `frame-${String(frame).padStart(4, "0")}.png`),
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
        "-vf", "scale=960:-2:flags=lanczos", "-loop", "0", "-c:v", "libwebp",
        "-quality", "58", "-compression_level", "5", "-an",
    ]);
    encode(captureDirectory, mediaDirectory, "route-transitions-demo.mp4", [
        "-c:v", "libx264", "-preset", "slow", "-crf", "24", "-pix_fmt", "yuv420p",
        "-movflags", "+faststart",
    ]);
    console.log(`${frame} browser-composited frames captured from ${showcaseUrl}`);
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
