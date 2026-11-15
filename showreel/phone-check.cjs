// Check the real showcase layout before refreshing the phone footage.
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const {chromium}=require('../playground/node_modules/playwright');
const {hydrated,pick,fullscreen}=require('./capture.cjs');
const config=require('./reel.json');
const output=path.join(__dirname,'build','phone-check-v5');

async function settled(page){
  await page.waitForFunction(()=>!document.documentElement.hasAttribute('data-route-transition'));
}

async function main(){
  fs.mkdirSync(output,{recursive:true});
  const browser=await chromium.launch({channel:'chrome',headless:true});
  const errors=[];
  try{
    for(const viewport of [[390,844],[1440,900]])for(const mode of ['iOS','MD'])for(const theme of ['Light','Dark']){
      const page=await browser.newPage({viewport:{width:1600,height:1200},reducedMotion:'no-preference'});
      page.on('pageerror',error=>errors.push(error.message));
      await page.goto(`${config.playground}/transitions/library/0`);
      await hydrated(page);
      await page.getByRole('radio',{name:mode,exact:true}).click();
      await pick(page,'Theme',theme);
      await pick(page,'Viewport',viewport[0]===390?'Mobile':'Desktop');
      await fullscreen(page,{viewport});
      await page.setViewportSize({width:viewport[0],height:viewport[1]});
      const app=page.locator('.g3-playground-preview');
      const group=app.getByRole('radiogroup',{name:'Library',exact:true});
      const colors=await group.evaluate(element=>{
        const shell=element.closest('.g3-app-shell');
        const style=getComputedStyle(shell);
        return {background:getComputedStyle(element).backgroundColor,page:style.getPropertyValue('--g3-color-bg').trim(),card:style.getPropertyValue('--g3-color-card').trim()};
      });
      assert.equal(colors.page,theme==='Light'?'#e9ebef':'#111827');
      assert.notEqual(colors.page,colors.card,'page and cards must have separate surfaces');
      if(mode==='iOS')assert.ok(!['transparent','rgba(0, 0, 0, 0)'].includes(colors.background),'the whole iOS group needs a visible track');
      await group.getByRole('radio',{name:'Artists',exact:true}).click();
      await page.waitForURL('**/transitions/library/1');
      await settled(page);
      assert.equal(await group.getByRole('radio',{name:'Artists',exact:true}).getAttribute('aria-checked'),'true');
      await group.getByRole('radio',{name:'Artists',exact:true}).focus();
      await page.keyboard.press('ArrowRight');
      await page.waitForURL('**/transitions/library/2');
      await settled(page);
      const label=`${viewport[0]}-${mode}-${theme}`;
      await page.screenshot({path:path.join(output,`library-${label}.png`)});
      await app.getByRole('button',{name:'Listen',exact:true}).click();
      await app.getByRole('button',{name:'Open queue',exact:true}).click();
      await page.waitForURL('**/transitions/queue');
      await settled(page);
      const list=await app.locator('.g3-list').last().boundingBox();
      const done=await app.getByRole('button',{name:'Done',exact:true}).boundingBox();
      assert.ok(done.y-(list.y+list.height)>=15.5,'Done needs at least 16px of space below the queue');
      await page.screenshot({path:path.join(output,`queue-${label}.png`)});
      await page.close();
    }
    assert.deepEqual(errors,[]);
    console.log('PASS: Library track, standard light/dark surfaces, segment pointer/keyboard navigation, and queue spacing in both modes at phone and desktop widths');
  }finally{await browser.close();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
