// Offline, frame-addressed GPU rendering. No real-time screen recording of the film.
window.initializeStage=async function(width=1920,height=1080) {
  const config=await fetch('reel.json').then(r=>r.json());
  const mesh=await fetch('build/device-v5.json').then(r=>r.json());
  if(mesh.targets.some((target,i)=>target.id!==config.shots[i]?.id||Math.abs(target.screen[0]-config.shots[i].screen_width)>1e-6))throw Error('Model targets do not match the storyboard; regenerate with render.py');
  const canvas=document.querySelector('canvas');canvas.width=width;canvas.height=height;
  const gl=canvas.getContext('webgl2',{alpha:false,antialias:true,preserveDrawingBuffer:true});
  if(!gl)throw Error('WebGL2 is required');
  function shader(type,source){const s=gl.createShader(type);gl.shaderSource(s,source);gl.compileShader(s);if(!gl.getShaderParameter(s,gl.COMPILE_STATUS))throw Error(gl.getShaderInfoLog(s));return s;}
  const program=gl.createProgram();
  gl.attachShader(program,shader(gl.VERTEX_SHADER,`#version 300 es
    precision highp float;
    layout(location=0) in vec3 p0;layout(location=1) in vec3 p1;layout(location=2) in vec3 p2;layout(location=3) in vec3 p3;
    layout(location=4) in vec3 normal;layout(location=5) in float face;
    uniform vec4 weights;uniform mat3 rotation;uniform vec2 camera;uniform float bob;
    out vec3 local;out vec3 worldNormal;out float frontFace;
    void main(){local=p0*weights.x+p1*weights.y+p2*weights.z+p3*weights.w;
      vec3 p=rotation*local;p.y+=bob;
      gl_Position=vec4(p.x*2.0/camera.x,p.y*2.0/camera.y,-p.z/8.0,1.0);
      worldNormal=rotation*normal;frontFace=face;}
  `));
  gl.attachShader(program,shader(gl.FRAGMENT_SHADER,`#version 300 es
    precision highp float;
    in vec3 local;in vec3 worldNormal;in float frontFace;
    uniform vec2 screen;uniform float blend;uniform float aspect0;uniform float aspect1;
    uniform sampler2D video0;uniform sampler2D video1;
    out vec4 color;
    float rounded(vec2 p,vec2 b,float r){vec2 q=abs(p)-b+r;return length(max(q,0.0))+min(max(q.x,q.y),0.0)-r;}
    vec2 cover(vec2 uv,float sourceAspect){float targetAspect=screen.x/screen.y;
      vec2 scale=sourceAspect>targetAspect?vec2(targetAspect/sourceAspect,1.0):vec2(1.0,sourceAspect/targetAspect);
      return (uv-.5)*scale+.5;}
    void main(){vec3 n=normalize(worldNormal);
      float key=max(dot(n,normalize(vec3(-3.,5.,8.))),0.);
      float fill=max(dot(n,normalize(vec3(5.,-2.,6.))),0.);
      float spec=pow(max(dot(n,normalize(vec3(-1.5,2.5,12.))),0.),55.);
      vec3 metal=vec3(.43,.45,.48)*(.45+.48*key+.22*fill)+vec3(.28)*spec;
      vec3 rgb=metal;
      if(frontFace>.999){
        float inner=rounded(local.xy,(screen+vec2(.13))*.5,.21);
        rgb=inner<0.?vec3(.018,.020,.024):metal;
        float d=rounded(local.xy,screen*.5,.145);
        float aa=max(fwidth(d),.0001);
        if(d<aa){vec2 uv=local.xy/screen+.5;
          vec3 a=texture(video0,cover(uv,aspect0)).rgb;
          vec3 b=texture(video1,cover(uv,aspect1)).rgb;
          rgb=mix(rgb,mix(a,b,blend),1.-smoothstep(-aa,aa,d));}
        vec2 lens=local.xy-vec2(0.,screen.y*.5+.045);
        if(length(lens)<.014)rgb=vec3(.10,.12,.15);
      }
      color=vec4(rgb,1.);
    }
  `));
  gl.linkProgram(program);if(!gl.getProgramParameter(program,gl.LINK_STATUS))throw Error(gl.getProgramInfoLog(program));
  gl.useProgram(program);
  function attribute(location,values,size){const b=gl.createBuffer();gl.bindBuffer(gl.ARRAY_BUFFER,b);gl.bufferData(gl.ARRAY_BUFFER,new Float32Array(values),gl.STATIC_DRAW);gl.enableVertexAttribArray(location);gl.vertexAttribPointer(location,size,gl.FLOAT,false,0,0);}
  mesh.targets.forEach((target,i)=>attribute(i,target.positions,3));
  attribute(4,mesh.targets[0].normals,3);attribute(5,mesh.targets[0].front,1);
  const index=gl.createBuffer();gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER,index);gl.bufferData(gl.ELEMENT_ARRAY_BUFFER,new Uint16Array(mesh.indices),gl.STATIC_DRAW);
  const uniforms=Object.fromEntries(['weights','rotation','camera','bob','screen','blend','aspect0','aspect1','video0','video1'].map(name=>[name,gl.getUniformLocation(program,name)]));
  const textures=[0,1].map(unit=>{const tex=gl.createTexture();gl.activeTexture(gl.TEXTURE0+unit);gl.bindTexture(gl.TEXTURE_2D,tex);gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_MIN_FILTER,gl.LINEAR_MIPMAP_LINEAR);gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_MAG_FILTER,gl.LINEAR);gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_WRAP_S,gl.CLAMP_TO_EDGE);gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_WRAP_T,gl.CLAMP_TO_EDGE);return tex;});
  gl.uniform1i(uniforms.video0,0);gl.uniform1i(uniforms.video1,1);
  gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL,true);
  gl.enable(gl.DEPTH_TEST);gl.clearColor(1,1,1,1);
  const videos=[];
  for(const shot of config.shots){const video=document.createElement('video');video.muted=true;video.preload='auto';video.src=`build/browser-v5/${shot.id}.mp4`;document.body.append(video);
    await new Promise((resolve,reject)=>{video.onloadeddata=resolve;video.onerror=()=>reject(Error('Cannot decode '+video.src));});videos.push(video);}
  const smooth=x=>{x=Math.max(0,Math.min(1,x));return x*x*(3-2*x);};
  const mix=(a,b,s)=>a+(b-a)*s;
  function state(time){
    let current=0;config.shots.forEach((shot,i)=>{if(time>=shot.start)current=i;});
    let a=current,b=current,s=0;
    for(let i=1;i<config.shots.length;i++){const p=(time-config.shots[i].start)/config.morph_seconds+.5;if(p>0&&p<1){a=i-1;b=i;s=smooth(p);}}
    const weights=[0,0,0,0];weights[a]+=1-s;weights[b]+=s;
    const screen=mesh.targets[a].screen.map((n,i)=>mix(n,mesh.targets[b].screen[i],s));
    return {a,b,s,weights,screen,camera:mix(config.shots[a].camera_width,config.shots[b].camera_width,s)};
  }
  async function seek(video,time){time=Math.max(0,Math.min(video.duration-1/config.fps,time))+.0001;
    if(Math.abs(video.currentTime-time)<.00005)return;
    await new Promise((resolve,reject)=>{video.onseeked=()=>{video.onseeked=null;resolve();};video.onerror=reject;video.currentTime=time;});}
  function multiply(a,b){const out=Array(9).fill(0);for(let c=0;c<3;c++)for(let r=0;r<3;r++)for(let k=0;k<3;k++)out[c*3+r]+=a[k*3+r]*b[c*3+k];return out;}
  function rotation(t){const x=.05236*Math.cos(t*.37),y=.10472*Math.sin(t*.43),z=.015708*Math.sin(t*.31);const sx=Math.sin(x),cx=Math.cos(x),sy=Math.sin(y),cy=Math.cos(y),sz=Math.sin(z),cz=Math.cos(z);
    return multiply([cz,sz,0,-sz,cz,0,0,0,1],multiply([cy,0,-sy,0,1,0,sy,0,cy],[1,0,0,0,cx,sx,0,-sx,cx]));}
  async function frame(time){const st=state(time);
    await seek(videos[st.a],time-config.shots[st.a].start);
    if(st.b!==st.a)await seek(videos[st.b],time-config.shots[st.b].start);
    [st.a,st.b].forEach((index,unit)=>{gl.activeTexture(gl.TEXTURE0+unit);gl.bindTexture(gl.TEXTURE_2D,textures[unit]);gl.texImage2D(gl.TEXTURE_2D,0,gl.RGBA,gl.RGBA,gl.UNSIGNED_BYTE,videos[index]);gl.generateMipmap(gl.TEXTURE_2D);});
    gl.uniform4fv(uniforms.weights,st.weights);gl.uniformMatrix3fv(uniforms.rotation,false,rotation(time));
    gl.uniform2f(uniforms.camera,st.camera,st.camera*height/width);gl.uniform1f(uniforms.bob,.10*Math.sin(time*.9));
    gl.uniform2fv(uniforms.screen,st.screen);gl.uniform1f(uniforms.blend,smooth((st.s-.42)/.16));
    gl.uniform1f(uniforms.aspect0,config.shots[st.a].viewport[0]/config.shots[st.a].viewport[1]);
    gl.uniform1f(uniforms.aspect1,config.shots[st.b].viewport[0]/config.shots[st.b].viewport[1]);
    gl.viewport(0,0,width,height);gl.clear(gl.COLOR_BUFFER_BIT|gl.DEPTH_BUFFER_BIT);gl.drawElements(gl.TRIANGLES,mesh.indices.length,gl.UNSIGNED_SHORT,0);gl.finish();
    return st;
  }
  window.drawStageFrame=frame;
  window.exportStage=async function(limit=config.duration*config.fps){
    let encodeError;const writes=[];let encoded=0;
    const encoder=new VideoEncoder({output(chunk){const bytes=new Uint8Array(chunk.byteLength);chunk.copyTo(bytes);let binary='';for(let i=0;i<bytes.length;i+=8192)binary+=String.fromCharCode(...bytes.subarray(i,i+8192));writes.push(window.writeEncoded(btoa(binary)));encoded++;},error(error){encodeError=error.message;}});
    encoder.configure({codec:width>1920?'avc1.640034':'avc1.64002a',width,height,framerate:config.fps,bitrate:Math.round(16000000*width*height/(1920*1080)),latencyMode:'realtime',avc:{format:'annexb'}});
    for(let i=0;i<limit;i++){
      await frame(i/config.fps);
      const vf=new VideoFrame(canvas,{timestamp:Math.round(i*1000000/config.fps),duration:Math.round(1000000/config.fps)});
      encoder.encode(vf,{keyFrame:i%120===0});vf.close();
      if(encoder.encodeQueueSize>5)await new Promise(resolve=>encoder.addEventListener('dequeue',resolve,{once:true}));
      if(encodeError)throw Error(encodeError);
      if(i%120===0)await window.reportProgress({frame:i,total:limit});
    }
    await encoder.flush();await Promise.all(writes);encoder.close();
    return {width,height,frames:limit,encoded,fps:config.fps,pixelAspect:'1:1',morphs:config.shots.slice(1).map(s=>s.start)};
  };
  await frame(0);
  return {width,height,targets:mesh.targets.length,encoder:typeof VideoEncoder};
};
