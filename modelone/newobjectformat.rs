struct Button {
	border_width: f64,
}

impl_changeable_struct!{ButtonChange[ButtonSignal] for Button:
	border_width: ValueChange<f64>,
}

impl Object<Button, ButtonChange> for Button {
	fn event(&self, cxt: &mut ApplyContext<ButtonChange>) {
		if self.border_width < 30 {
			// This doesn't actually change self until the event pass has finished and the immutable
			// reference to the model root is released.
			cxt.apply(ButtonChange::border_width(50));
		}
	}
}

struct App {
	button: Button,
}

impl_changeable_struct!{AppChange[AppSignal] for App:
	button: ButtonChange,
}

impl Object<App, AppChange> for App {
	fn event(&self, cxt: ApplyContext<AppChange>) {
		self.button.event(&mut SubApplyContext::new(cxt, |c| AppChange::button(c)));
	}
}

trait ApplyContext<C> {
	fn apply(&self, change: C);
}

struct SubApplyContext<PC, C> {
	parent: &mut ApplyContext<PC>,
	wrap_fn: &Fn(C) -> PC,
	wrap_constructor_fn: &'p Fn(Box<ChangeConstructor<C>>) -> Box<ChangeConstructor<PC>>,
}

impl ApplyContext for SubApplyContext {
	fn apply(&self, change: C) {
		self.parent.apply((self.wrap_fn)(change));
	}
}
