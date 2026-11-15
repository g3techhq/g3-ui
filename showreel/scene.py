"""Generate one rounded Blender device with four matching morph targets."""
import bpy
import json
import math
from pathlib import Path

ROOT=Path(__file__).resolve().parent
CFG=json.loads((ROOT/'reel.json').read_text())
BUILD=ROOT/'build'
BUILD.mkdir(exist_ok=True)
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete(use_global=False)
N=100

def target(shot):
    width=shot['screen_width']
    height=width*shot['viewport'][1]/shot['viewport'][0]
    vertices=[(0,0,.09)]
    normals=[(0,0,1)]
    front=[1]
    # A flat front plus bevel, sidewall, and back. Every size has identical topology.
    profiles=[(0,.09,0,1),(.023,.065,.70710678,.70710678),(.023,-.065,1,0),(0,-.09,.70710678,-.70710678)]
    for expansion,z,radial,nz in profiles:
        w,h=width+.18+expansion*2,height+.18+expansion*2
        radius=.235+expansion
        for cx,cy,start in [(w/2-radius,h/2-radius,0),(-w/2+radius,h/2-radius,90),(-w/2+radius,-h/2+radius,180),(w/2-radius,-h/2+radius,270)]:
            for step in range(25):
                a=math.radians(start+step*90/24)
                vertices.append((cx+radius*math.cos(a),cy+radius*math.sin(a),z))
                normals.append((radial*math.cos(a),radial*math.sin(a),nz))
                front.append(1 if z==.09 else 0)
    return {'id':shot['id'],'positions':[c for v in vertices for c in v],'normals':[c for v in normals for c in v],'front':front,'screen':[width,height]}

targets=[target(shot) for shot in CFG['shots']]
indices=[]
for i in range(N):indices.extend((0,1+i,1+(i+1)%N))
for ring in range(3):
    for i in range(N):
        a=1+ring*N+i;b=1+ring*N+(i+1)%N;c=a+N;d=b+N
        indices.extend((a,c,b,b,c,d))
data={'targets':targets,'indices':indices,'radius':.235,'bezel':.09,'thickness':.18}
(BUILD/'device-v5.json').write_text(json.dumps(data,separators=(',',':')))
positions=targets[0]['positions']
vertices=[positions[i:i+3] for i in range(0,len(positions),3)]
faces=[indices[i:i+3] for i in range(0,len(indices),3)]
mesh=bpy.data.meshes.new('Shared rounded slab topology')
mesh.from_pydata(vertices,[],faces)
device=bpy.data.objects.new('Floating morphing device',mesh)
bpy.context.collection.objects.link(device)
device.shape_key_add(name='Basis')
for shape in targets:
    key=device.shape_key_add(name=shape['id'])
    for i,point in enumerate(key.data):point.co=shape['positions'][i*3:i*3+3]
    # Blender review timeline uses the same smooth size changes as the renderer.
    for frame in range(1,CFG['duration']*CFG['fps']+1):
        t=(frame-1)/CFG['fps']
        weights=[0]*len(targets)
        current=max(i for i,s in enumerate(CFG['shots']) if t>=s['start'])
        weights[current]=1
        for index,shot in enumerate(CFG['shots'][1:],1):
            q=(t-shot['start'])/CFG['morph_seconds']+.5
            if 0<q<1:
                q=q*q*(3-2*q);weights=[0]*len(targets);weights[index-1]=1-q;weights[index]=q
        key.value=weights[targets.index(shape)]
        key.keyframe_insert('value',frame=frame)
for frame in range(1,CFG['duration']*CFG['fps']+1,6):
    t=(frame-1)/CFG['fps']
    device.rotation_euler=(math.radians(3)*math.cos(t*.37),math.radians(6)*math.sin(t*.43),math.radians(.9)*math.sin(t*.31))
    device.location.y=.10*math.sin(t*.9)
    device.keyframe_insert('rotation_euler',frame=frame)
    device.keyframe_insert('location',frame=frame)
scene=bpy.context.scene
scene.render.fps=CFG['fps']
scene.frame_end=CFG['duration']*CFG['fps']
scene.frame_set(1)
bpy.ops.wm.save_as_mainfile(filepath=str(BUILD/'floating-device-v5.blend'))
print('Exported shared device geometry and four Blender morph targets')
