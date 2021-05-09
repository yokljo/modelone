enum Easing {
	Linear,
	Parabolic,
}

impl Easing {
	fn get_scalar(progress: f64, easing: Easing) -> f64 {
		match easing {
			Easing::Linear => {
				progress
			},
			Easing::Parabolic => {
				progress * progress
			},
		}
	}
}

struct PropertyAnimation<T> {
	from: T,
	to: T,
	progress: f64,
	duration: f64,
}

impl<T: std::ops::Mul<f64>> PropertyAnimation<T> {
	fn step(&mut self, delta: f64) {
		
	}
}
