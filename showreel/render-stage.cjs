const fs=require('node:fs');
const path=require('node:path');
const http=require('node:http');
const {spawnSync}=require('node:child_process');
const {chromium}=require('../playground/node_modules/playwright');
const root=__dirname,build=path.join(root,'build');
const config=JSON.parse(fs.readFileSync(path.join(root,'reel.json')));
const args=process.argv.slice(2);
const width=args.includes('--width')?Number(args[args.indexOf('--width')+1]):1920;
const height=width*9/16;
const run=argv=>{const r=spawnSync('ffmpeg',['-y','-v','error',...argv],{stdio:'inherit'});if(r.status!==0)throw Error('FFmpeg failed');};

async function main(){
  fs.mkdirSync(path.join(build,'browser-v5'),{recursive:true});
  for(const shot of config.shots){
    const source=path.join(build,'screens-v5',`${shot.id}.mp4`),output=path.join(build,'browser-v5',`${shot.id}.mp4`);
    if(!fs.existsSync(output)||fs.statSync(output).mtimeMs<fs.statSync(source).mtimeMs){
      console.log('Preparing frame-addressable video:',shot.id);
      // Chrome's WebGL video upload clips pale colors from full-range AVC.
      // Convert the capture once to explicitly tagged, limited-range BT.709.
      run(['-i',source,'-an','-vf','scale=in_range=auto:out_range=tv:out_color_matrix=bt709,setsar=1','-color_range','tv','-colorspace','bt709','-color_primaries','bt709','-color_trc','bt709','-c:v','libx264','-preset','fast','-crf','12','-g','1','-pix_fmt','yuv420p','-threads','6',output]);
    }
  }
  const server=http.createServer((req,res)=>{
    const relative=decodeURIComponent(new URL(req.url,'http://localhost').pathname).replace(/^\//,'')||'stage.html';
    const file=path.resolve(root,relative);
    if(!file.startsWith(root+path.sep)||!fs.existsSync(file)||!fs.statSync(file).isFile()){res.writeHead(404);res.end();return;}
    const stat=fs.statSync(file),mime={'.html':'text/html','.js':'text/javascript','.json':'application/json','.mp4':'video/mp4'}[path.extname(file)]||'application/octet-stream';
    let start=0,end=stat.size-1,status=200;
    if(req.headers.range){const match=/bytes=(\d+)-(\d*)/.exec(req.headers.range);if(match){start=Number(match[1]);if(match[2])end=Math.min(Number(match[2]),end);status=206;}}
    const headers={'Content-Type':mime,'Accept-Ranges':'bytes','Content-Length':end-start+1};
    if(status===206)headers['Content-Range']=`bytes ${start}-${end}/${stat.size}`;
    res.writeHead(status,headers);fs.createReadStream(file,{start,end}).pipe(res);
  });
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  const origin=`http://127.0.0.1:${server.address().port}`;
  let browser;
  try{
    browser=await chromium.launch({channel:'chrome',headless:true,args:['--disable-background-timer-throttling','--disable-renderer-backgrounding']});
    const page=await browser.newPage({viewport:{width,height}});
    page.on('pageerror',error=>console.error(error));
    await page.goto(origin);
    console.log(await page.evaluate(({width,height})=>initializeStage(width,height),{width,height}));
    if(args.includes('--still')){
      const time=Number(args[args.indexOf('--still')+1]);
      await page.evaluate(t=>drawStageFrame(t),time);
      await page.locator('canvas').screenshot({path:path.join(build,`v5-preview-${time}.png`)});
      return;
    }
    const raw=path.join(build,'floating-v5.h264');fs.writeFileSync(raw,Buffer.alloc(0));
    await page.exposeFunction('writeEncoded',data=>fs.appendFileSync(raw,Buffer.from(data,'base64')));
    await page.exposeFunction('reportProgress',state=>console.log(`Rendering ${state.frame}/${state.total}`,new Date().toISOString()));
    const limit=args.includes('--frames')?Number(args[args.indexOf('--frames')+1]):config.duration*config.fps;
    const report=await page.evaluate(limit=>exportStage(limit),limit);
    fs.writeFileSync(path.join(build,'render-v5.json'),JSON.stringify(report,null,2));
    const output=path.join(build,args.includes('--frames')?'v5-test.mp4':'the-g3-stack-v5.mp4');
    run(['-r',String(config.fps),'-fflags','+genpts','-i',raw,'-i',path.join(build,'score.wav'),'-t',String(limit/config.fps),'-c:v','copy','-bsf:v','h264_metadata=sample_aspect_ratio=1/1','-aspect','16:9','-af','loudnorm=I=-16:TP=-1.5:LRA=9','-ar','48000','-c:a','aac','-b:a','256k','-movflags','+faststart',output]);
    const check=spawnSync('ffprobe',['-v','error','-show_streams','-show_format','-of','json',output],{encoding:'utf8'});
    const metadata=JSON.parse(check.stdout),video=metadata.streams.find(s=>s.codec_type==='video');
    if(video.sample_aspect_ratio!=='1:1'||video.display_aspect_ratio!=='16:9')throw Error('Display aspect ratio validation failed');
    // Check the actual browser player, including pixel-aspect interpretation.
    const playback=await page.evaluate(async url=>{
      const video=document.createElement('video');video.src=url;video.style.cssText='display:block;width:960px;height:auto;position:fixed;top:0;left:0';document.body.append(video);
      await new Promise(resolve=>video.onloadedmetadata=resolve);
      const box=video.getBoundingClientRect();return {videoWidth:video.videoWidth,videoHeight:video.videoHeight,width:box.width,height:box.height};
    },`${origin}/build/${path.basename(output)}`);
    if(playback.width!==960||playback.height!==540)throw Error('Browser playback is not 16:9: '+JSON.stringify(playback));
    fs.writeFileSync(path.join(build,'playback-v5.json'),JSON.stringify(playback,null,2));
    console.log('Verified browser playback:',playback);console.log(output);
  }finally{if(browser)await browser.close();await new Promise(resolve=>server.close(resolve));}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
