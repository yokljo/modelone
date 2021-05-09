#ifdef VERT

in vec2 vPos;
in vec4 vColour;
in int vBorder;

out vec2 fPos;
out vec4 fColour;
flat out int fBorder;

void main() {
	gl_Position = uTransform * vec4(vPos.x * uSize.x + uPos.x, vPos.y * uSize.y + uPos.y, 0.0, 1.0);
	fPos = vPos;
	fColour = vColour;
	fBorder = vBorder;
}

#endif

#ifdef FRAG

uniform float uGradient;

in vec2 fPos;
in vec4 fColour;
flat in int fBorder;

out vec4 out_colour;

void main() {
	out_colour = fColour;
	if (fBorder == 0) {
		//out_colour *= sin(uGradient * fPos.x*5) + cos(uGradient * fPos.y*5);
	}
}

#endif
