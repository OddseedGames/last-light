import math
import random
import struct
import wave
from pathlib import Path


assets = Path(__file__).resolve().parent.parent / "assets"
rate = 22050
rng = random.Random(7391)
for name, duration in [("shotgun", 0.38), ("reload", 0.24), ("hurt", 0.30)]:
    samples = []
    for i in range(int(rate * duration)):
        t = i / rate
        noise = rng.uniform(-1.0, 1.0)
        if name == "shotgun":
            sample = (noise * 0.6 + math.sin(t * 2 * math.pi * (85 - t * 100)) * 0.4) * math.exp(-t * 14)
        elif name == "reload":
            envelope = math.exp(-t * 70) + (0.65 * math.exp(-(t - 0.12) * 75) if t >= 0.12 else 0)
            sample = (noise * 0.6 + math.sin(t * 2 * math.pi * 1700) * 0.3) * envelope
        else:
            sample = (math.sin(t * 2 * math.pi * (140 - t * 200)) * 0.7 + noise * 0.2) * math.exp(-t * 10)
        samples.append(struct.pack('<h', int(max(-1, min(1, sample * 0.7)) * 32767)))
    with wave.open(str(assets / (name + '.wav')), 'wb') as audio:
        audio.setnchannels(1)
        audio.setsampwidth(2)
        audio.setframerate(rate)
        audio.writeframes(b''.join(samples))
