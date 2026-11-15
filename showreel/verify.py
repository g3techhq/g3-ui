"""Verify source cadence, morph geometry, and actual displayed video dimensions."""
import json
import math
from pathlib import Path
import subprocess

ROOT=Path(__file__).resolve().parent
BUILD=ROOT/'build'
config=json.loads((ROOT/'reel.json').read_text())

def probe(file):
    return json.loads(subprocess.check_output(['ffprobe','-v','error','-show_streams','-show_format','-of','json',str(file)]))

def rate(stream):
    n,d=map(int,stream['r_frame_rate'].split('/'))
    return n/d


def patch_color(file,time,x,y,size):
    """Read the average RGB of a flat patch from the actual encoded video."""
    rgb=subprocess.check_output([
        'ffmpeg','-v','error','-ss',str(time),'-i',str(file),
        '-vf',f'crop={size}:{size}:{round(x-size/2)}:{round(y-size/2)},scale=1:1:flags=area',
        '-frames:v','1','-f','rawvideo','-pix_fmt','rgb24','pipe:1',
    ])
    assert len(rgb)==3
    return list(rgb)


def project_phone(u,v,time,width,height):
    """Locate a source patch on the floating display in delivery pixels."""
    shot=config['shots'][0]
    screen_width,screen_height=geometry['targets'][0]['screen']
    a,b,c=(u-.5)*screen_width,(.5-v)*screen_height,.09
    pitch=.05236*math.cos(time*.37)
    yaw=.10472*math.sin(time*.43)
    roll=.015708*math.sin(time*.31)
    b,c=math.cos(pitch)*b-math.sin(pitch)*c,math.sin(pitch)*b+math.cos(pitch)*c
    a,c=math.cos(yaw)*a+math.sin(yaw)*c,-math.sin(yaw)*a+math.cos(yaw)*c
    a,b=math.cos(roll)*a-math.sin(roll)*b,math.sin(roll)*a+math.cos(roll)*b
    b+=.1*math.sin(time*.9)
    camera=shot['camera_width']
    return (a/camera+.5)*width,(.5-b/(camera*height/width))*height

assert [s['id'] for s in config['shots']]==['phone','phone_landscape','tablet','desktop']
capture=json.loads((BUILD/'capture-v5.json').read_text())
geometry=json.loads((BUILD/'device-v5.json').read_text())
render=json.loads((BUILD/'render-v5.json').read_text())
playback=json.loads((BUILD/'playback-v5.json').read_text())
end=0
vertex_count=len(geometry['targets'][0]['positions'])
for shot,target in zip(config['shots'],geometry['targets'],strict=True):
    assert shot['start']==end and shot['end']>shot['start']
    end=shot['end']
    source=probe(BUILD/'screens-v5'/f'{shot["id"]}.mp4')['streams'][0]
    assert int(source['nb_frames'])==(shot['end']-shot['start'])*config['fps']
    assert rate(source)==config['fps']
    assert source.get('sample_aspect_ratio','1:1')=='1:1'
    expected_aspect=shot['viewport'][0]/shot['viewport'][1]
    assert abs((source['width']/source['height'])/expected_aspect-1)<1e-6
    assert target['id']==shot['id'] and len(target['positions'])==vertex_count
    assert abs((target['screen'][0]/target['screen'][1])/expected_aspect-1)<1e-6
    assert len(target['normals'])==vertex_count
    assert max(geometry['indices'])<vertex_count//3
    cadence=next(x for x in capture['sources'] if x['id']==shot['id'])
    assert cadence['motionFps']>=50
assert end==config['duration']
delivery=probe(BUILD/'the-g3-stack-v5.mp4')
video=next(s for s in delivery['streams'] if s['codec_type']=='video')
audio=next(s for s in delivery['streams'] if s['codec_type']=='audio')
assert abs(float(delivery['format']['duration'])-config['duration'])<.05
assert int(video['nb_frames'])==config['duration']*config['fps']==render['frames']==render['encoded']
assert video['width']*9==video['height']*16
assert video['sample_aspect_ratio']=='1:1', 'Non-square pixels squash the entire video in players'
assert video['display_aspect_ratio']=='16:9'
assert playback['videoWidth']==video['width'] and playback['videoHeight']==video['height']
assert playback['width']==960 and playback['height']==540, 'Actual browser player is not widescreen'
assert rate(video)==config['fps']
assert audio['channels']==2 and audio['sample_rate']=='48000'
assert video['codec_name']=='h264' and audio['codec_name']=='aac'
assert render['morphs']==[s['start'] for s in config['shots'][1:]]
# Compare real UI footage with the finished film. Resolution and successful
# decoding alone missed the full-range AVC upload washing pale fills to white.
source_path=BUILD/'screens-v5'/'phone.mp4'
source_video=probe(source_path)['streams'][0]
colors=[]
for label,u,v in [('page',.1,.65),('segment track',.3,.056)]:
    expected=patch_color(source_path,12,u*source_video['width'],v*source_video['height'],9)
    x,y=project_phone(u,v,12,video['width'],video['height'])
    actual=patch_color(BUILD/'the-g3-stack-v5.mp4',12,x,y,5)
    assert min(expected)>200, f'{label} sample must be a flat pale surface'
    assert max(abs(a-b) for a,b in zip(expected,actual))<=8, f'{label} color was changed: {expected} -> {actual}'
    colors.append({'surface':label,'source':expected,'delivery':actual})
(BUILD/'colors-v5.json').write_text(json.dumps(colors,indent=2)+'\n')
subprocess.run(['ffmpeg','-v','error','-i',str(BUILD/'the-g3-stack-v5.mp4'),'-f','null','-'],check=True)
print(f'PASS: four matching morph targets, complete 60 fps sources, faithful page/segment colors; {video["width"]}x{video["height"]}, square pixels, 16:9 display, browser playback 960x540, {config["duration"]} seconds')
