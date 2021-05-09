use crate::vec2::*;
use crate::item_graphics_pipeline::*;
use crate::item;
use crate::item::*;
use crate::resources::GraphicResource;

use std;
use time;

use glutin::{self, Context as GlContext, event_loop::{ControlFlow, EventLoopWindowTarget}};
use glutin::dpi::{LogicalSize, PhysicalSize};

use gl;

use cgmath;

use std::sync::Arc;
use std::sync::atomic;
use std::thread;

use modelone::object;
use modelone::model::{Change, Changeable};
use modelone::change_value::ValueChange;

struct GlutinWindow {
	window_context: glutin::WindowedContext<glutin::PossiblyCurrent>,
}

impl GlutinWindow {
	fn new(events_loop: &glutin::event_loop::EventLoop<()>) -> GlutinWindow {
		let window_builder = glutin::window::WindowBuilder::new()
			.with_title("Awesome")
			.with_inner_size(LogicalSize::new(800., 600.));
		
		let context_builder = glutin::ContextBuilder::new()
			.with_gl(glutin::GlRequest::Latest)
			//.with_gl_debug_flag(true)
			.with_gl_robustness(glutin::Robustness::RobustNoResetNotification)
		;
		//EventLoopWindowTarget::
		//let gl_window = glutin::window::Window::new(window_builder, context_builder, events_loop).unwrap();
		let window_context = context_builder.build_windowed(window_builder, &events_loop).unwrap();
		let window_context = unsafe { window_context.make_current().unwrap() };
		
		//let display = glium::Display::new(window_builder, context_builder, events_loop).unwrap();
		
		GlutinWindow {
			window_context
		}
	}
	
	fn update_view(&mut self) {
		let size = self.window_context.window().inner_size();
		self.window_context.resize(size);
		unsafe {
			gl::Viewport(0, 0, size.width as i32, size.height as i32);
		}
		//gfx_window_glutin::update_views(&self.window, &mut self.render_target_view, &mut self.depth_stencil_view);
	}
}

fn glutin_mouse_to_item_mouse(glutin_button: glutin::event::MouseButton) -> item::MouseButton {
	match glutin_button {
		glutin::event::MouseButton::Left => item::MouseButton::Left,
		glutin::event::MouseButton::Right => item::MouseButton::Right,
		glutin::event::MouseButton::Middle => item::MouseButton::Middle,
		_ => item::MouseButton::None
	}
}

pub fn exec<T: 'static, C, V: 'static>(
	mut manager: object::Manager<T, C, V>,
	get_root_item_change: fn(ItemDataChange) -> C,
	make_resources: &dyn Fn() -> Vec<Box<dyn GraphicResource>>) -> !
where
	T: Changeable<C> + Item + object::Object<C>,
	C: 'static + Change + std::fmt::Debug + std::clone::Clone,
	V: object::Validator<C>
{
	println!("Starting event loop...");
	let running = Arc::new(atomic::AtomicBool::new(true));
	
	let mut display_cache = DisplayCache::new();
	
	for resource in make_resources() {
		display_cache.register_resource(resource).unwrap();
	}
	
	let mut events_loop = glutin::event_loop::EventLoop::new();
	
	//let mut root_apply_handle = manager.apply_handle();
	
	//let gl_context = glutin::ContextBuilder::new();

	let mut window = GlutinWindow::new(&events_loop);
	gl::load_with(|s| window.window_context.get_proc_address(s) as *const _);

	//unsafe { window.window_context.make_current() }.unwrap();
	
	let async_change_notifier = manager.get_async_change_notifier();
	
	let events_proxy = events_loop.create_proxy();
	
	let thread_running = running.clone();
	let check_event_thread = thread::spawn(move || {
		while thread_running.load(atomic::Ordering::Relaxed) {
			async_change_notifier.wait();
			events_proxy.send_event(()).unwrap();
			
		}
	});
	
	//let mut root_apply_handle = manager.apply_handle();
	//let root_apply_handle2 = root_apply_handle.clone();

	/*thread::spawn(move || {
		thread::sleep(Duration::from_millis(5000));
		root_apply_handle.invoke(get_root_item_change(ItemDataChange::size(ValueChange::Set(Vec2f::new(123f64, 123f64)))));
	});*/
	
	// Update UI cache.
	display_cache.process_item(manager.get(), Vec2f::new(0., 0.), None);
	display_cache.process_messages();
	
	manager.reset_view();
	
	println!("Event loop started.");
	
	//let mut events = vec![];

	events_loop.run(move |event, _window_target, control_flow| {
		let mut _start = time::PreciseTime::now();
		let mut _print_delay = |timeindex: i32| {
			let now = time::PreciseTime::now();
			println!("{} {}", timeindex, _start.to(now) * 1000);
			_start = now;
		};
		
		/*if !display_cache.is_animating() {
			if events.is_empty() {
				events_loop.run(|event| {
					events.push(event);
					glutin::ControlFlow::Break
				});
				
				events_loop.poll_events(|event| {
					events.push(event);
				});
			}
		} else {
			//println!("Animation events...");
		}*/
		
		match event {
			glutin::event::Event::WindowEvent{window_id: _, ref event} => {
				match *event {
					glutin::event::WindowEvent::Resized(size) => {
						window.update_view();
						manager.apply(get_root_item_change(ItemDataChange::size(ValueChange(Vec2f::new(size.width as f64, size.height as f64)))));
					}
					glutin::event::WindowEvent::CloseRequested => {
						running.store(false, atomic::Ordering::Relaxed);
					}
					glutin::event::WindowEvent::CursorMoved{device_id: _, position, modifiers: _} => {
						display_cache.process_mouse_pos(Vec2f::new(position.x, position.y));
					}
					glutin::event::WindowEvent::MouseInput{device_id: _, state, button, modifiers: _} => {
						match state {
							glutin::event::ElementState::Pressed => {
								display_cache.process_mouse_down(glutin_mouse_to_item_mouse(button));
							}
							glutin::event::ElementState::Released => {
								display_cache.process_mouse_up(glutin_mouse_to_item_mouse(button));
							}
						}
						
					}
					glutin::event::WindowEvent::KeyboardInput{input, ..} => {
						match input.state {
							glutin::event::ElementState::Pressed => {
								if let Some(key) = input.virtual_keycode {
									if key == glutin::event::VirtualKeyCode::Escape {
										running.store(false, atomic::Ordering::Relaxed);
									}
								}
							}
							glutin::event::ElementState::Released => {}
						}
					}
					_ => {}
				}
			}
			glutin::event::Event::RedrawRequested(window_id) => {
				display_cache.send_animation_signals(1f64/60f64);
		
				manager.resolve_signals();
				manager.try_process_async_changes();
				
				let window_size = window.window_context.window().inner_size();
				
				let transform = cgmath::ortho(0., window_size.width as f32, window_size.height as f32, 0., -1., 1.);
				
				// Update UI cache.
				display_cache.process_item(manager.get(), Vec2f::new(0., 0.), None);
				display_cache.process_messages();
				
				unsafe {
					gl::ClearColor(0., 0., 0., 1.);
					gl::Clear(gl::COLOR_BUFFER_BIT);
				}
				
				// Draw cached UI.
				display_cache.draw(&transform);
				
				window.window_context.swap_buffers().unwrap();
			}
			_ => {}
		}

		if display_cache.is_animating() {
		}

		if running.load(atomic::Ordering::Relaxed) {
			*control_flow = ControlFlow::Poll;
		} else {
			*control_flow = ControlFlow::Exit;
		}
		
		//println!("Events handled.");
		
	});
	
	/*while running.load(atomic::Ordering::Relaxed) {
		let mut _start = time::PreciseTime::now();
		let mut _print_delay = |timeindex: i32| {
			let now = time::PreciseTime::now();
			println!("{} {}", timeindex, _start.to(now) * 1000);
			_start = now;
		};
		
		events.clear();
		
		// NOTE: At the time of writing, doing run_forever prevents the Awakened event from working,
		// so doing poll_events first should prevent that from happening.
		// https://github.com/tomaka/winit/issues/462
		events_loop.poll_events(|event| {
			events.push(event);
		});
		
		if !display_cache.is_animating() {
			if events.is_empty() {
				events_loop.run(|event| {
					events.push(event);
					glutin::ControlFlow::Break
				});
				
				events_loop.poll_events(|event| {
					events.push(event);
				});
			}
		} else {
			//println!("Animation events...");
		}
		
		for event in &events {
			match *event {
				glutin::event::Event::WindowEvent{window_id: _, ref event} => {
					match *event {
						glutin::event::WindowEvent::Resized(size) => {
							window.update_view();
							manager.apply(get_root_item_change(ItemDataChange::size(ValueChange(Vec2f::new(size.width as f64, size.height as f64)))));
						}
						glutin::event::WindowEvent::CloseRequested => {
							running.store(false, atomic::Ordering::Relaxed);
						}
						glutin::event::WindowEvent::CursorMoved{device_id: _, position, modifiers: _} => {
							display_cache.process_mouse_pos(Vec2f::new(position.x, position.y));
						}
						glutin::event::WindowEvent::MouseInput{device_id: _, state, button, modifiers: _} => {
							match state {
								glutin::event::ElementState::Pressed => {
									display_cache.process_mouse_down(glutin_mouse_to_item_mouse(button));
								}
								glutin::event::ElementState::Released => {
									display_cache.process_mouse_up(glutin_mouse_to_item_mouse(button));
								}
							}
							
						}
						glutin::event::WindowEvent::KeyboardInput{input, ..} => {
							match input.state {
								glutin::event::ElementState::Pressed => {
									if let Some(key) = input.virtual_keycode {
										if key == glutin::event::VirtualKeyCode::Escape {
											running.store(false, atomic::Ordering::Relaxed);
										}
									}
								}
								glutin::event::ElementState::Released => {}
							}
						}
						_ => {}
					}
				}
				_ => {}
			}
		}
		
		//println!("Events handled.");
		
		display_cache.send_animation_signals(1f64/60f64);
		
		manager.resolve_signals();
		manager.try_process_async_changes();
		
		let window_size = window.window_context.window().get_inner_size().unwrap();
		
		let transform = cgmath::ortho(0., window_size.width as f32, window_size.height as f32, 0., -1., 1.);
		
		// Update UI cache.
		display_cache.process_item(manager.get(), Vec2f::new(0., 0.), None);
		display_cache.process_messages();
		
		unsafe {
			gl::ClearColor(0., 0., 0., 1.);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
		
		// Draw cached UI.
		display_cache.draw(&transform);
		
		window.gl_window.swap_buffers().unwrap();
	}*/
	
	running.store(false, atomic::Ordering::Relaxed);
	manager.get_async_change_notifier().notify();
	check_event_thread.join().unwrap();
}
