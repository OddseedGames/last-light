#version 100
precision highp float;
varying vec2 world;
uniform vec4 Player;
uniform vec4 Beam;

float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}

void main() {
    vec2 p = world - vec2(28.0, 110.0);
    vec2 delta = p - Player.xy;
    vec2 axis = vec2(cos(Player.z), sin(Player.z));
    float x = dot(delta, axis);
    float y = abs(dot(delta, vec2(-axis.y, axis.x)));
    float width = 0.0;
    if (x > 0.0 && x < Beam.x + Beam.y) {
        if (x < Beam.w) {
            width = x * Beam.z / sqrt(Beam.x * Beam.x - Beam.y * Beam.y);
        } else {
            width = Beam.z * sqrt(max(0.0, 1.0 - pow((x - Beam.x) / Beam.y, 2.0)));
        }
    }
    float edge = clamp((width - y) / 12.0, 0.0, 1.0);
    float end = clamp((Beam.x + Beam.y - x) / 22.0, 0.0, 1.0);
    float light = min(edge, end);
    float grit = hash(floor(p * 0.65));
    float broad = hash(floor(p / 37.0));
    vec3 asphalt = vec3(0.18, 0.215, 0.20) * (0.8 + grit * 0.32 + broad * 0.15);
    vec2 tile = abs(mod(p + vec2(25.0, 16.0), 84.0) - 42.0);
    float seam = step(41.2, max(tile.x, tile.y));
    asphalt *= 1.0 - seam * 0.32;
    float crack = step(0.994, hash(floor(p / 3.0))) * step(0.5, grit);
    asphalt *= 1.0 - crack * 0.6;
    float stripe = step(abs(p.x - 170.0), 3.0) * step(mod(p.y, 100.0), 52.0);
    asphalt = mix(asphalt, vec3(0.35, 0.34, 0.23), stripe * 0.5);
    float nearPlayer = (1.0 - smoothstep(8.0, 35.0, length(delta))) * 0.12;
    float brightness = 0.15 + nearPlayer + light * (1.25 - x / 760.0);
    vec3 tint = mix(vec3(0.56, 0.72, 0.75), vec3(1.23, 1.16, 0.82), light);
    vec3 result = asphalt * brightness * tint;
    float dust = step(0.996, hash(floor(p / 2.0))) * light * 0.13;
    result += vec3(0.8, 0.75, 0.5) * dust;
    float vignette = 1.0 - 0.22 * length((p - vec2(612.0, 330.0)) / vec2(612.0, 330.0));
    gl_FragColor = vec4(result * vignette, 1.0);
}
