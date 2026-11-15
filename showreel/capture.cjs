// Stream Chrome's compositor frames; never serialize screenshots in a timer loop.
const fs = require('node:fs');
const path = require('node:path');
const {spawnSync} = require('node:child_process');
const {chromium} = require('../playground/node_modules/playwright');
const root = __dirname;
const config = JSON.parse(fs.readFileSync(path.join(root,'reel.json')));
const build = path.join(root,'build');
const sleep = ms => new Promise(resolve=>setTimeout(resolve,ms));
const provenance = process.argv.includes('--device') && fs.existsSync(path.join(build,'capture-v5.json')) ? JSON.parse(fs.readFileSync(path.join(build,'capture-v5.json'))) : {method:'Chrome compositor screencast',capturedAt:new Date().toISOString(),sources:[]};

async function hydrated(page) {
  // Server HTML is visible before WASM has attached its event listeners.
  await page.waitForFunction(()=>!!document.querySelector('button[data-dioxus-id]'));
}

async function pick(page,name,value,behindPreview=false) {
  const control=page.getByRole('combobox',{name,exact:true,includeHidden:behindPreview});
  if(behindPreview) await control.evaluate(element=>element.click());
  else await control.click();
  const option=page.getByRole('option',{name:value,exact:true,includeHidden:behindPreview});
  if(behindPreview) await option.evaluate(element=>element.click());
  else await option.click();
}

async function fullscreen(page,shot) {
  await page.addStyleTag({content:`
    #__dx-toast,dioxus-devtools,.transition-showcase-device-frame{display:none!important}
    [role="tooltip"],.g3-nav-item-tooltip{display:none!important}
    #main > .g3-app-shell{visibility:hidden!important}
    .g3-playground-preview,.g3-playground-preview *{visibility:visible!important}
    .playground-header,.playground-intro,.source-panel,.playground-selector-sheet{visibility:hidden!important}
    .playground-viewport-card{position:fixed!important;inset:0!important;width:${shot.viewport[0]}px!important;height:${shot.viewport[1]}px!important;z-index:9999!important}
    .playground-viewport-demo,.g3-playground-demo-stack{width:100%!important;height:100%!important}
    .g3-playground-preview{position:fixed!important;inset:0!important;width:${shot.viewport[0]}px!important;height:${shot.viewport[1]}px!important;max-width:none!important;max-height:none!important;aspect-ratio:auto!important;border:0!important;border-radius:0!important;box-shadow:none!important;transform:none!important}
    html,body{overflow:hidden!important}
  `});
}

async function artwork(page) {
  await page.evaluate(async()=>{
    await document.fonts.ready;
    await Promise.race([
      Promise.all(Array.from(document.images).filter(img=>img.getBoundingClientRect().top<innerHeight).map(img=>img.decode().catch(()=>{}))),
      new Promise(resolve=>setTimeout(resolve,2500)),
    ]);
  });
}

async function record(page,shot,actions) {
  const seconds=shot.end-shot.start;
  const dir=path.join(build,'capture-v5',shot.id,String(Date.now()));
  fs.mkdirSync(dir,{recursive:true});
  const client=await page.context().newCDPSession(page);
  const timestamps=[];
  let firstTimestamp;
  let firstFrame;
  const ready=new Promise(resolve=>{firstFrame=resolve;});
  let accepting=true;
  client.on('Page.screencastFrame',event=>{
    void client.send('Page.screencastFrameAck',{sessionId:event.sessionId}).catch(()=>{});
    if(!accepting)return;
    firstTimestamp ??= event.metadata.timestamp;
    const time=event.metadata.timestamp-firstTimestamp;
    if(time>seconds)return;
    const index=timestamps.length;
    fs.writeFileSync(path.join(dir,`${String(index).padStart(5,'0')}.jpg`),Buffer.from(event.data,'base64'));
    timestamps.push(time);
    if(index===0)firstFrame();
  });
  await client.send('Page.startScreencast',{format:'jpeg',quality:100,maxWidth:4096,maxHeight:4096,everyNthFrame:1});
  await Promise.race([ready,sleep(15000).then(()=>{throw Error('Compositor did not produce a frame');})]);
  let actionError;
  const actionRun=actions().catch(error=>{actionError=error;});
  await sleep(seconds*1000);
  accepting=false;
  await client.send('Page.stopScreencast');
  await actionRun;
  await client.detach();
  if(actionError)throw actionError;
  if(timestamps.length<2)throw Error(`No changing compositor frames for ${shot.id}`);
  // image2 otherwise defaults to a 1/25 time base and quantizes smooth motion.
  const manifest=timestamps.map((t,i)=>`file '${String(i).padStart(5,'0')}.jpg'\noption framerate 1000000\nduration ${Math.max(.0001,(timestamps[i+1]??seconds)-t)}`).join('\n')+`\nfile '${String(timestamps.length-1).padStart(5,'0')}.jpg'\noption framerate 1000000\n`;
  fs.writeFileSync(path.join(dir,'frames.txt'),manifest);
  const output=path.join(build,'screens-v5');
  fs.mkdirSync(output,{recursive:true});
  const result=spawnSync('ffmpeg',['-y','-v','error','-f','concat','-safe','0','-i',path.join(dir,'frames.txt'),'-vf',`fps=${config.fps},tpad=stop_mode=clone:stop_duration=1,setsar=1`,'-frames:v',String(seconds*config.fps),'-c:v','libx264','-preset','fast','-crf','10','-pix_fmt','yuv444p',path.join(output,`${shot.id}.mp4`)],{stdio:'inherit'});
  if(result.status!==0)throw Error(`Could not encode ${shot.id}`);
  const deltas=timestamps.slice(1).map((t,i)=>t-timestamps[i]).filter(dt=>dt>0&&dt<.1).sort((a,b)=>a-b);
  const motionFps=1/deltas[Math.floor(deltas.length/2)];
  const info={id:shot.id,url:page.url(),viewport:shot.viewport,pixelRatio:shot.pixel_ratio,seconds,frames:timestamps.length,motionFps:Math.round(motionFps),capturedAt:new Date().toISOString()};
  provenance.sources=provenance.sources.filter(source=>source.id!==shot.id);
  provenance.sources.push(info);
  fs.writeFileSync(path.join(build,'capture-v5.json'),JSON.stringify(provenance,null,2));
  console.log(JSON.stringify(info));
}

async function main() {
  let browser;
  const options={headless:true,args:['--disable-background-timer-throttling','--disable-renderer-backgrounding','--run-all-compositor-stages-before-draw']};
  try{browser=await chromium.launch({...options,channel:'chrome'});}catch{browser=await chromium.launch(options);}
  try {
    for(const shot of config.shots) {
      const [width,height]=shot.viewport;
      if(process.argv.includes('--device') && shot.id!==process.argv[process.argv.indexOf('--device')+1])continue;
      const physical={width:Math.round(width*shot.pixel_ratio),height:Math.round(height*shot.pixel_ratio)};
      const page=await browser.newPage({viewport:physical,deviceScaleFactor:1,reducedMotion:'no-preference'});
      if(shot.source==='transitions'||shot.source==='playground') {
        await page.setViewportSize({width:1500,height:1200});
        await page.goto(`${config.playground}/${shot.source==='transitions'?'transitions':'components/demo-app'}`);
        await page.locator('.g3-playground-preview').waitFor();
        await hydrated(page);
        await page.getByRole('radio',{name:'iOS',exact:true}).click();
        if(shot.theme)await pick(page,'Theme',shot.theme);
        await pick(page,'Viewport',shot.id==='phone'?'Mobile':'Landscape phone');
        await fullscreen(page,shot);
        await page.setViewportSize(physical);
        await page.evaluate(zoom=>document.documentElement.style.zoom=String(zoom),shot.pixel_ratio);
        await artwork(page);
        await sleep(700);
        const app=page.locator('.g3-playground-preview');
        if(shot.source==='playground') {
          const collapse=app.getByRole('button',{name:'Collapse navigation',exact:true,includeHidden:true});
          if(await collapse.count())await collapse.evaluate(element=>element.click());
          await sleep(500);
          await app.getByRole('button',{name:'Expand navigation',exact:true,includeHidden:true}).waitFor({state:'attached'});
        }
        await page.mouse.move(physical.width+20,physical.height+20);
        await record(page,shot,async()=>{
          if(shot.source==='transitions') {
            await sleep(900);await app.getByRole('button',{name:'Open album',exact:true}).click();
            await sleep(1800);await app.getByRole('button',{name:'Listen',exact:true}).first().click();
            await sleep(1600);await app.getByRole('button',{name:'Open queue',exact:true}).click();
            await sleep(1800);await app.getByRole('button',{name:'Listen',exact:true}).first().click();
            await sleep(1700);await app.getByRole('button',{name:'Library',exact:true}).click();
            const library=app.getByRole('radiogroup',{name:'Library',exact:true});
            await sleep(1800);await library.getByRole('radio',{name:'Artists',exact:true}).click();
            await sleep(2000);await library.getByRole('radio',{name:'Playlists',exact:true}).click();
            await sleep(2200);await library.getByRole('radio',{name:'Albums',exact:true}).click();
            await sleep(2000);await library.getByRole('radio',{name:'Artists',exact:true}).click();
          } else {
            await sleep(900);await app.getByRole('button',{name:'Book',exact:true}).click();
            await page.mouse.move(physical.width+20,physical.height+20);
            await sleep(1800);await pick(page,'Theme','Dark',true);
            await sleep(2200);await pick(page,'Theme','Glass',true);
            await sleep(2000);await app.getByRole('button',{name:'Profile',exact:true}).evaluate(el=>el.click());
            await sleep(1200);await page.getByRole('radio',{name:'MD',exact:true,includeHidden:true}).evaluate(el=>el.click());
          }
        });
      } else if(shot.source==='media_mancer') {
        await page.goto(`${config.media_mancer}/signin/options`,{waitUntil:'domcontentloaded'});
        await page.addStyleTag({content:'#__dx-toast,dioxus-devtools{display:none!important}'});
        await page.evaluate(zoom=>document.documentElement.style.zoom=String(zoom),shot.pixel_ratio);
        const guest=page.getByRole('button',{name:'Continue as guest',exact:true});
        await guest.waitFor({timeout:60000});
        for(let i=0;i<15&&!/\/home/.test(page.url());i++){if(await guest.isVisible())await guest.click();await sleep(800);}
        if(!/\/home/.test(page.url()))throw Error('Guest sign-in failed');
        const later=page.getByRole('button',{name:'Later',exact:true});
        if(await later.isVisible())await later.click();
        const nav=page.getByRole('navigation',{name:'Primary navigation'});
        await nav.getByRole('button',{name:'Search',exact:true}).click();
        await page.getByRole('searchbox').first().fill('Dune');
        const dune=page.getByRole('button',{name:'Dune',exact:true}).first();
        await dune.waitFor();await sleep(1000);await artwork(page);await dune.click();await sleep(1000);await artwork(page);
        await nav.getByRole('button',{name:'Home',exact:true}).click();await sleep(1500);await artwork(page);
        await record(page,shot,async()=>{
          await sleep(1800);await nav.getByRole('button',{name:'Search',exact:true}).click();
          await page.getByRole('searchbox').first().fill('Dune');
          await sleep(3300);await page.getByRole('button',{name:'Dune',exact:true}).first().click();
          await sleep(2500);await nav.getByRole('button',{name:'Home',exact:true}).click();
        });
      } else {
        await page.goto(`${config.charts}/?theme=light&mode=ios`);
        await page.getByRole('heading',{name:'g3-charts',exact:true}).waitFor();
        await hydrated(page);
        await page.addStyleTag({content:'#__dx-toast,dioxus-devtools{display:none!important}'});
        // Keep the root View Transition layer in viewport pixels so the
        // circular reveal starts exactly at the clicked toolbar segment.
        await page.addStyleTag({content:`body{zoom:${shot.pixel_ratio};width:${width}px;height:${height}px}#main>.g3-app-shell{height:${height}px!important;min-height:0!important}`});
        await artwork(page);
        await record(page,shot,async()=>{
          const theme=page.getByRole('radiogroup',{name:'Theme',exact:true});
          const look=page.getByRole('radiogroup',{name:'Look',exact:true});
          await sleep(1500);
          const plot=page.getByRole('slider',{name:'Inspect Portfolio value and contributions',exact:true});
          const box=await plot.boundingBox();
          if(box) {
            for(let i=0;i<=32;i++) {
              await page.mouse.move(box.x+box.width*(.1+i*.025),box.y+box.height*.45);
              await sleep(22);
            }
          }
          await sleep(900);await theme.getByRole('radio',{name:'Dark',exact:true}).click();
          await page.waitForFunction(()=>document.documentElement.hasAttribute('data-g3-theme-reveal'));
          await page.waitForFunction(()=>!document.documentElement.hasAttribute('data-g3-theme-reveal'));
          await sleep(1700);await look.getByRole('radio',{name:'Material',exact:true}).click();
          await page.waitForFunction(()=>!document.documentElement.hasAttribute('data-g3-theme-reveal'));
          await sleep(1800);await theme.getByRole('radio',{name:'Light',exact:true}).click();
          await page.waitForFunction(()=>!document.documentElement.hasAttribute('data-g3-theme-reveal'));
          await sleep(1800);await look.getByRole('radio',{name:'iOS',exact:true}).click();
          await page.waitForFunction(()=>!document.documentElement.hasAttribute('data-g3-theme-reveal'));
          await sleep(1700);
          await page.evaluate(()=>document.querySelector('.g3-content-scroll')?.scrollTo({top:520,behavior:'smooth'}));
          await sleep(2800);
          await page.evaluate(()=>document.querySelector('.g3-content-scroll')?.scrollTo({top:0,behavior:'smooth'}));
        });
      }
      await page.close();
    }
  } finally {await browser.close();}
}
module.exports={fullscreen,pick,artwork,hydrated};
if(require.main===module)main().catch(error=>{console.error(error);process.exitCode=1;});
