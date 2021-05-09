#ifdef VERT

in vec2 vPos;
in vec2 vTexCoord;

out vec2 fTexCoord;

void main() {
	gl_Position = uTransform * vec4(vPos.x * uSize.x + uPos.x, vPos.y * uSize.y + uPos.y, 0.0, 1.0);
	fTexCoord = vTexCoord;
}

#endif

#ifdef FRAG

uniform sampler2D uTex;

in vec2 fTexCoord;

out vec4 out_colour;

void main() {
	float alpha = texture(uTex, fTexCoord).r;
	float brightness = 1 - alpha;
	out_colour = vec4(brightness, brightness, brightness, alpha);
}

#endif
