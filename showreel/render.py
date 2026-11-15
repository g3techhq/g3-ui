"""Regenerate the reel: optional live capture, Blender, original audio, FFmpeg."""
import argparse
import json
import os
from pathlib import Path
import shutil
import socket
import subprocess
import sys
import time
import urllib.request
from urllib.parse import urlparse
from audio import generate

ROOT=Path(__file__).resolve().parent
BUILD=ROOT/'build'
CFG=json.loads((ROOT/'reel.json').read_text())

def command(args, **kwargs):
    print(' '.join(map(str,args)),flush=True)
    subprocess.run(list(map(str,args)),check=True,**kwargs)

def find_blender():
    found=os.environ.get('BLENDER') or shutil.which('blender')
    if found:return found
    candidates=sorted(Path('C:/Program Files/Blender Foundation').glob('Blender */blender.exe'))
    if candidates:return str(candidates[-1])
    raise RuntimeError('Blender not found. Set BLENDER to its executable path.')

def ready(url, marker=None):
    try:
        with urllib.request.urlopen(url,timeout=2) as response:
            html=response.read().decode(errors='replace')
            # Devtools can leave their rebuild notice in otherwise-ready HTML.
            # Capture waits for each app's actual UI before recording.
            return response.status==200 and (marker is None or marker in html)
    except Exception:return False

def occupied(port):
    with socket.socket() as connection:return connection.connect_ex(('127.0.0.1',port))==0

def start_database():
    if ready(CFG['media_mancer']):return False
    if urlparse(CFG['media_mancer']).hostname not in ('127.0.0.1','localhost'):return False
    directory=ROOT.parent.parent/'media-mancer'
    running=subprocess.check_output(['docker','compose','ps','--status','running','-q','db'],cwd=directory,text=True).strip()
    if running:return False
    command(['docker','compose','up','-d','db'],cwd=directory)
    return True

def stop_servers(processes):
    for process,logfile in reversed(processes):
        if os.name=='nt':subprocess.run(['taskkill','/PID',str(process.pid),'/T','/F'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        else:process.terminate()
        logfile.close()

def start_servers(processes):
    for name,directory,key in [
        ('playground',ROOT.parent/'playground','playground'),
        ('media-mancer',ROOT.parent.parent/'media-mancer','media_mancer'),
        ('charts',ROOT.parent.parent/'g3-charts'/'gallery','charts'),
    ]:
        url=CFG[key]
        marker='g3-ui playground' if name=='playground' else None
        if ready(url,marker):continue
        port=urlparse(url).port
        if occupied(port):
            if name!='playground':raise RuntimeError(f'{url} is occupied but not ready. See the existing server log.')
            while occupied(port):port+=1
            parsed=urlparse(url)
            url=parsed._replace(netloc=f'{parsed.hostname}:{port}').geturl()
            CFG[key]=url
            (ROOT/'reel.json').write_text(json.dumps(CFG,indent=2)+'\n',encoding='utf-8')
        logfile=open(BUILD/f'{name}-server.log','w')
        process=subprocess.Popen(['dx','serve','--addr','127.0.0.1','--port',str(port),'--open','false','--interactive','false','--hot-patch','false','--watch','false','--hot-reload','false','--verbose'],cwd=directory,stdout=logfile,stderr=subprocess.STDOUT,creationflags=getattr(subprocess,'CREATE_NO_WINDOW',0))
        processes.append((process,logfile))
        began=time.monotonic()
        while not ready(url,marker):
            if process.poll() is not None or time.monotonic()-began>600:
                raise RuntimeError(f'{name} failed to start. See {logfile.name}')
            time.sleep(1)

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--capture',action='store_true',help='Recapture real app UIs; starts missing dx servers automatically')
    parser.add_argument('--width',type=int,default=1920,choices=[960,1280,1920,2560,3840])
    parser.add_argument('--still',type=int,help='Render one numbered frame for review')
    parser.add_argument('--encode-only',action='store_true')
    parser.add_argument('--capture-only',action='store_true',help='Refresh live recordings without rendering hardware or composing')
    args=parser.parse_args()
    BUILD.mkdir(exist_ok=True)
    servers=[]
    started_database=False
    try:
        if args.capture:
            started_database=start_database()
            start_servers(servers)
            command(['node',ROOT/'capture.cjs'])
            # Rendering does not need a live app or its file watcher.
            stop_servers(servers)
            servers.clear()
            if started_database:
                command(['docker','compose','stop','db'],cwd=ROOT.parent.parent/'media-mancer')
                started_database=False
            if args.capture_only:return
        if not args.encode_only:
            for shot in CFG['shots']:
                if not (BUILD/'screens-v5'/f'{shot["id"]}.mp4').exists():
                    raise RuntimeError(f'Missing {shot["id"]} capture. Run with --capture first.')
            command([find_blender(),'--background','--python',ROOT/'scene.py'])
        if args.still:
            timepoint=(args.still-1)/CFG['fps']
            command(['node',ROOT/'render-stage.cjs','--width',args.width,'--still',timepoint])
            return
        generate()
        command(['node',ROOT/'render-stage.cjs','--width',args.width])
        command(['ffprobe','-v','error','-show_entries','format=duration:stream=codec_name,width,height,sample_aspect_ratio,display_aspect_ratio,r_frame_rate,sample_rate','-of','json',BUILD/'the-g3-stack-v5.mp4'])
    finally:
        stop_servers(servers)
        if started_database:
            subprocess.run(['docker','compose','stop','db'],cwd=ROOT.parent.parent/'media-mancer',check=False)

if __name__=='__main__':main()
