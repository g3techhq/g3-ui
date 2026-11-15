"""Original deterministic score. Standard library only; no sampled recordings."""
import array
import json
import math
import random
import wave
from pathlib import Path

ROOT = Path(__file__).resolve().parent
CFG = json.loads((ROOT/'reel.json').read_text())
RATE = 48000
LENGTH = CFG['duration']

def generate():
    left = array.array('f', [0]) * (RATE*LENGTH)
    right = array.array('f', [0]) * (RATE*LENGTH)
    rng = random.Random(73)
    beat = 60 / CFG['bpm']
    def tone(start, duration, freq, gain, pan=0, kind='bell'):
        offset = int(start*RATE)
        for i in range(min(int(duration*RATE),len(left)-offset)):
            t=i/RATE
            if kind=='pad':
                env=min(1,t/.6)*min(1,(duration-t)/.8)
                value=(math.sin(math.tau*freq*t)+.20*math.sin(math.tau*freq*2.003*t))*env*gain
            elif kind=='kick':
                value=math.sin(math.tau*(47*t+4*(1-math.exp(-t*35))))*math.exp(-t*14)*gain
            elif kind=='air':
                env=math.sin(math.pi*t/duration)**2
                value=(rng.random()*2-1)*env*gain
            else:
                env=(1-math.exp(-t*140))*math.exp(-t*5)
                value=(math.sin(math.tau*freq*t)+.18*math.sin(math.tau*freq*3*t))*env*gain
            left[offset+i]+=value*(1-pan*.35)
            right[offset+i]+=value*(1+pan*.35)
    # Dmaj9 -> Bm7 -> Gmaj9 -> Asus: a restrained, warm electronic bed.
    chords=[[50,57,61,64,69],[47,54,57,62,66],[43,50,54,57,62],[45,52,57,59,64]]
    hz=lambda note:440*2**((note-69)/12)
    for bar in range(math.ceil(LENGTH/(beat*8))):
        start=bar*beat*8
        chord=chords[bar%4]
        for j,note in enumerate(chord):
            tone(start,beat*8+.8,hz(note),.026,(j-2)/2,'pad')
        for step in range(16):
            when=start+step*beat/2
            if when<LENGTH-2:
                note=chord[[0,2,3,1,4,2,3,4][step%8]]+12
                tone(when,.85,hz(note),.043,math.sin(step*1.7))
        for step in range(8):
            when=start+step*beat
            if when<LENGTH-2:tone(when,.35,48,.09,0,'kick')
    for shot in CFG['shots'][1:]:
        tone(shot['start']-.50,.7,0,.055,0,'air')
        tone(shot['start'],1.4,hz(86),.08,-.4)
        tone(shot['start']+.125,1.4,hz(93),.04,.4)
    pcm=array.array('h')
    for i,(l,r) in enumerate(zip(left,right)):
        t=i/RATE
        fade=min(1,t/1.5,(LENGTH-t)/2.5)
        pcm.extend((int(math.tanh(l)*fade*28000),int(math.tanh(r)*fade*28000)))
    output=ROOT/'build'/'score.wav'
    output.parent.mkdir(exist_ok=True)
    with wave.open(str(output),'wb') as file:
        file.setnchannels(2);file.setsampwidth(2);file.setframerate(RATE);file.writeframes(pcm.tobytes())
    print(f'Original {CFG["bpm"]} BPM stereo score: {output}')

if __name__=='__main__':generate()
