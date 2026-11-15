// Verify the UI features shown in the reel, before capturing or encoding it.
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const {chromium}=require('../playground/node_modules/playwright');
const {fullscreen,pick,hydrated}=require('./capture.cjs');
const config=JSON.parse(fs.readFileSync(path.join(__dirname,'reel.json')));
const output=path.join(__dirname,'build','ui-check-v5');
const sleep=ms=>new Promise(resolve=>setTimeout(resolve,ms));

async function reveal(page,button){
  await button.click();
  await page.waitForFunction(()=>document.getAnimations().some(a=>a.animationName==='g3-theme-reveal'));
  const origin=await page.evaluate(()=>[getComputedStyle(document.documentElement).getPropertyValue('--g3-reveal-x'),getComputedStyle(document.documentElement).getPropertyValue('--g3-reveal-y')]);
  assert.ok(origin.every(value=>Number.isFinite(parseFloat(value))),'bubble reveal must have a real click origin');
  await page.waitForFunction(()=>!document.documentElement.hasAttribute('data-g3-theme-reveal'));
}

async function main(){
  fs.mkdirSync(output,{recursive:true});
  const browser=await chromium.launch({channel:'chrome',headless:true});
  const errors=[];
  try{
    for(const viewport of [{width:390,height:844},{width:1440,height:900}]){
      const page=await browser.newPage({viewport,reducedMotion:'no-preference'});
      page.on('pageerror',error=>errors.push(error.message));
      await page.goto(`${config.charts}/?theme=light&mode=ios`);
      const theme=page.getByRole('radiogroup',{name:'Theme',exact:true});
      const look=page.getByRole('radiogroup',{name:'Look',exact:true});
      await theme.waitFor();
      await hydrated(page);
      await page.addStyleTag({content:'#__dx-toast{display:none!important}'});
      assert.equal(await page.getByRole('link',{name:'Dark',exact:true}).count(),0);
      assert.ok(await theme.evaluate(element=>!!element.closest('header')),'theme segments belong in the header');
      assert.ok(await look.evaluate(element=>!!element.closest('header')),'look segments belong in the header');
      const dimensions=await page.evaluate(()=>({width:innerWidth,document:document.documentElement.scrollWidth}));
      assert.ok(dimensions.document<=dimensions.width+1,'gallery must fit a phone and desktop');
      await page.screenshot({path:path.join(output,`charts-${viewport.width}-light-ios.png`)});
      await reveal(page,theme.getByRole('radio',{name:'Dark',exact:true}));
      assert.equal(await theme.getByRole('radio',{name:'Dark',exact:true}).getAttribute('aria-checked'),'true');
      await page.screenshot({path:path.join(output,`charts-${viewport.width}-dark-ios.png`)});
      await reveal(page,look.getByRole('radio',{name:'Material',exact:true}));
      await page.screenshot({path:path.join(output,`charts-${viewport.width}-dark-material.png`)});
      await reveal(page,theme.getByRole('radio',{name:'Light',exact:true}));
      await page.screenshot({path:path.join(output,`charts-${viewport.width}-light-material.png`)});
      await theme.getByRole('radio',{name:'Light',exact:true}).focus();
      await page.keyboard.press('ArrowRight');
      await page.waitForFunction(()=>document.querySelector('[aria-label="Theme"] [aria-checked="true"]')?.textContent==='Dark');
      await page.close();
    }
    for(const id of ['phone','phone_landscape']){
      const shot=config.shots.find(shot=>shot.id===id);
      const page=await browser.newPage({viewport:{width:1500,height:1200},reducedMotion:'no-preference'});
      page.on('pageerror',error=>errors.push(error.message));
      await page.goto(`${config.playground}/${id==='phone'?'transitions/library/0':'components/demo-app'}`);
      await page.locator('.g3-playground-preview').waitFor();
      await hydrated(page);
      await page.getByRole('radio',{name:'iOS',exact:true}).click();
      await pick(page,'Viewport',id==='phone'?'Mobile':'Landscape phone');
      await fullscreen(page,shot);
      await page.setViewportSize({width:shot.viewport[0]*shot.pixel_ratio,height:shot.viewport[1]*shot.pixel_ratio});
      await page.evaluate(zoom=>document.documentElement.style.zoom=String(zoom),shot.pixel_ratio);
      await sleep(700);
      const app=page.locator('.g3-playground-preview');
      const frame=await app.boundingBox();
      assert.equal(frame.width,shot.viewport[0]*shot.pixel_ratio);
      assert.equal(frame.height,shot.viewport[1]*shot.pixel_ratio);
      if(id==='phone_landscape'){
        const collapse=app.getByRole('button',{name:'Collapse navigation',exact:true,includeHidden:true});
        if(await collapse.count())await collapse.evaluate(element=>element.click());
        await sleep(500);
        await app.getByRole('button',{name:'Expand navigation',exact:true,includeHidden:true}).waitFor({state:'attached'});
        const box=await app.getByRole('navigation',{name:'Primary navigation',exact:true}).boundingBox();
        assert.ok(box.width/shot.pixel_ratio<85,`landscape nav should be an icon rail, got ${box.width/shot.pixel_ratio}px`);
      }else{
        await page.evaluate(()=>{
          window.libraryFrames=[];let count=0;
          function sample(){const group=document.querySelector('.g3-playground-preview [role="radiogroup"][aria-label="Library"]');window.libraryFrames.push({indicator:getComputedStyle(group,'::before').transform,body:getComputedStyle(document.documentElement,'::view-transition-new(segment)').transform});if(count++<100)requestAnimationFrame(sample);}sample();
        });
        await app.getByRole('radiogroup',{name:'Library',exact:true}).getByRole('radio',{name:'Artists',exact:true}).click();
        await sleep(850);
        const frames=await page.evaluate(()=>window.libraryFrames);
        assert.ok(new Set(frames.map(f=>f.indicator)).size>4,'Library header indicator should slide');
        assert.ok(new Set(frames.map(f=>f.body)).size>4,'Library content should slide alongside it');
      }
      await page.screenshot({path:path.join(output,`${id}.png`)});
      await page.close();
    }
    assert.deepEqual(errors,[],'UI must not produce hydration or browser errors');
    console.log('PASS: phone/desktop gallery, iOS/Material, both themes, bubble reveal, keyboard, landscape rail, and Library indicator/body motion');
  }finally{await browser.close();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
