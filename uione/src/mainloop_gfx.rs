use crate::vec2::*;
use crate::item_graphics_pipeline::*;
use crate::item;
use crate::item::*;
use crate::resources::GraphicResource;

use std;
use time;

use glutin::{self, GlContext};
use glutin::dpi::{LogicalSize, PhysicalSize};

#[cfg(feature = "vulkan")]
use gfx_backend_vulkan;

use cgmath;

use std::sync::Arc;
use std::sync::atomic;
use std::thread;

use modelone::object;
use modelone::model::{Change, Changeable};
use modelone::change_value::ValueChange;

struct GfxWindow {
	window: winit::Window,
	instance: gfx_backend_vulkan::Instance,
	surface: gfx_backend_vulkan::Surface,
}

impl GfxWindow {
	fn new(events_loop: &glutin::EventsLoop) -> GfxWindow {
		let window_builder = winit::WindowBuilder::new()
			.with_title("Awesome".to_string())
			.with_dimensions(winit::dpi::LogicalSize::new(800., 600.));
			
		let window = window_builder.build(&events_loop).unwrap();
		let instance = gfx_backend_vulkan::Instance::create("GfxWindow", 1);
		let surface = instance.create_surface(&window);
		let adapters = instance.enumerate_adapters();

		GfxWindow {
			window,
			instance,
			
		}
	}
	
	fn update_view(&mut self) {
		if let Some(size) = self.gl_window.get_inner_size() {
			self.gl_window.resize(PhysicalSize::from_logical(size, 1.));
			unsafe {
				gl::Viewport(0, 0, size.width as i32, size.height as i32);
			}
		} else {
			println!("Window has no size!");
		}
		//gfx_window_glutin::update_views(&self.window, &mut self.render_target_view, &mut self.depth_stencil_view);
	}
}

fn glutin_mouse_to_item_mouse(glutin_button: glutin::MouseButton) -> item::MouseButton {
	match glutin_button {
		glutin::MouseButton::Left => item::MouseButton::Left,
		glutin::MouseButton::Right => item::MouseButton::Right,
		glutin::MouseButton::Middle => item::MouseButton::Middle,
		_ => item::MouseButton::None
	}
}

pub fn exec<T, C, V>(
	manager: &mut object::Manager<T, C, V>,
	get_root_item_change: fn(ItemDataChange) -> C,
	make_resources: &Fn() -> Vec<Box<GraphicResource>>)
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
	
	let mut events_loop = glutin::EventsLoop::new();
	
	//let mut root_apply_handle = manager.apply_handle();
	
	//let gl_context = glutin::ContextBuilder::new();

	let mut window = GlutinWindow::new(&events_loop);
	unsafe { window.gl_window.make_current() }.unwrap();
	gl::load_with(|s| window.gl_window.get_proc_address(s) as *const _);
	
	let async_change_notifier = manager.get_async_change_notifier();
	
	let events_proxy = events_loop.create_proxy();
	
	let thread_running = running.clone();
	let check_event_thread = thread::spawn(move || {
		while thread_running.load(atomic::Ordering::Relaxed) {
			async_change_notifier.wait();
			events_proxy.wakeup().unwrap();
			
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
	
	let mut events = vec![];
	
	while running.load(atomic::Ordering::Relaxed) {
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
				events_loop.run_forever(|event| {
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
				glutin::Event::WindowEvent{window_id: _, ref event} => {
					match *event {
						glutin::WindowEvent::Resized(size) => {
							window.update_view();
							manager.apply(get_root_item_change(ItemDataChange::size(ValueChange(Vec2f::new(size.width as f64, size.height as f64)))));
						}
						glutin::WindowEvent::CloseRequested => {
							running.store(false, atomic::Ordering::Relaxed);
						}
						glutin::WindowEvent::CursorMoved{device_id: _, position, modifiers: _} => {
							display_cache.process_mouse_pos(Vec2f::new(position.x, position.y));
						}
						glutin::WindowEvent::MouseInput{device_id: _, state, button, modifiers: _} => {
							match state {
								glutin::ElementState::Pressed => {
									display_cache.process_mouse_down(glutin_mouse_to_item_mouse(button));
								}
								glutin::ElementState::Released => {
									display_cache.process_mouse_up(glutin_mouse_to_item_mouse(button));
								}
							}
							
						}
						glutin::WindowEvent::KeyboardInput{device_id: _, input} => {
							match input.state {
								glutin::ElementState::Pressed => {
									if let Some(key) = input.virtual_keycode {
										if key == glutin::VirtualKeyCode::Escape {
											running.store(false, atomic::Ordering::Relaxed);
										}
									}
								}
								glutin::ElementState::Released => {}
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
		
		let window_size = window.gl_window.get_inner_size().unwrap();
		
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
	}
	
	running.store(false, atomic::Ordering::Relaxed);
	manager.get_async_change_notifier().notify();
	check_event_thread.join().unwrap();
}
