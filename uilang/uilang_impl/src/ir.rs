use crate::ast;

use std::collections::HashSet;
use syn;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use heck::SnakeCase;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct PropertyId(usize);

/*struct Property {
	name: syn::Ident,
	ty: syn::Type,
	change_ty: Option<syn::Type>,
	is_signal: bool,
}*/

#[derive(Debug)]
struct ChangeHandler {
	prop_name: syn::Ident,
	resolved_prop: Option<PropertyId>,
	body: ast::BindingExpr,
}

#[derive(Debug)]
struct MatchChangeHandler {
	prop_name: syn::Ident,
	resolved_prop: Option<PropertyId>,
	body: ast::BindingExpr,
}

#[derive(Debug, Default)]
struct Anchors {
	fill: Option<ast::BindingExpr>,
	fill_width: Option<ast::BindingExpr>,
	fill_height: Option<ast::BindingExpr>,
	left: Option<ast::BindingExpr>,
	right: Option<ast::BindingExpr>,
	top: Option<ast::BindingExpr>,
	bottom: Option<ast::BindingExpr>,
	margins: f64,
	margin_left: f64,
	margin_right: f64,
	margin_top: f64,
	margin_bottom: f64,
}

#[derive(Debug, Clone)]
struct PropPath {
	prop_id: usize,
	sub_prop_path: Vec<syn::Ident>,
}

#[derive(Debug, Clone)]
struct ProcessedBindingResolved {
	deps: Vec<PropPath>,
}

#[derive(Debug, Clone)]
struct ProcessedBinding {
	context_prop: usize,
	for_prop: PropPath,
	expr: Vec<ast::BindingExprPart>,
	resolved: Option<ProcessedBindingResolved>,
}

impl ProcessedBinding {
	fn resolve_deps(&self, _prop_graph: &PropertyGraph) -> ParseResult<ProcessedBindingResolved> {
		let mut deps = vec![];
		
		for expr_part in &self.expr {
			match expr_part {
				ast::BindingExprPart::Dep(_dep_part) => {
					let mut _is_first = true;
					let mut prop_id = self.context_prop;
					let mut sub_prop_path = vec![];
					
					/*for path_part in &dep_part.path {
						let mut found_known_sub = false;
						if is_first {
							if let Some(found_id) = prop_graph.lookup_prop(prop_id, &path_part.to_string()) {
								prop_id = found_id;
								found_known_sub = true;
							}
						} else if sub_prop_path.is_empty() {
							if let Some(found_id) = prop_graph.lookup_local_prop(prop_id, &path_part.to_string()) {
								prop_id = found_id;
								found_known_sub = true;
							}
						}
						
						if !found_known_sub {
							sub_prop_path.push(path_part.clone());
						}
						
						is_first = false;
					}*/
					
					/*for path_part in &dep_path.inst_path {
						
					}*/
					
					let prop_path = PropPath {
						prop_id,
						sub_prop_path,
					};
					
					deps.push(prop_path);
				}
				_ => {}
			}
		}
		
		Ok(ProcessedBindingResolved {
			deps,
		})
	}
	
	fn gen_expr(&self, resolver: &Fn(&PropPath) -> TokenStream) -> TokenStream {
		let mut final_expr = TokenStream::new();
		let mut dep_index = 0;
		let ref deps = self.resolved.as_ref().expect("Resolved expression").deps;
		
		for expr_part in &self.expr {
			match expr_part {
				ast::BindingExprPart::Dep(_dep_part) => {
					final_expr.extend(resolver(&deps[dep_index]));
					dep_index += 1;
				}
				ast::BindingExprPart::Tokens(toks) => {
					final_expr.extend(toks.clone().into_iter());
				}
			}
		}
		
		final_expr
	}
}

#[derive(Debug)]
struct ProcessedPropertyResolved {
	field_name: Option<syn::Ident>,
}

#[derive(Debug)]
struct ProcessedProperty {
	name: Option<syn::Ident>,
	main_type: syn::Type,
	change_type: syn::Type,
	//sub_properties: Vec<Property>,
	//bindings: Vec<Binding>,
	//binding_expr: Option<BindingExpr>,
	parent: Option<usize>,
	is_inst: bool,
	
	//is_mouse_item: bool,
	//anchors: Anchors,
	
	resolved: Option<ProcessedPropertyResolved>,
}

impl ProcessedProperty {
	fn resolve_locals(&self, prop_graph: &PropertyGraph, ident_source: &mut IdentSource) -> ParseResult<ProcessedPropertyResolved> {
		println!("resolving {:?}", self.parent);
		if self.parent.is_some() {
			let mut name = self.name.clone().unwrap_or_else(|| {
				let mut gen_name = "".to_string();
				for tok in self.main_type.clone().into_token_stream() {
					gen_name += &tok.to_string();
				}
				syn::Ident::new(&gen_name.to_snake_case(), self.main_type.span())
			});
			
			// Prefix the parent's name (as long as the parent isn't the root)
			if let Some(parent) = self.parent {
				let ref parent_prop = prop_graph.properties[parent];
				if parent_prop.parent.is_some() {
					if let Some(ref parent_name) = parent_prop.name {
						name = syn::Ident::new(&(parent_name.to_string() + "_" + &name.to_string()), name.span());
					}
				}
			}
			
			let resolved_name = ident_source.get(&name, IdentType::Snake);
			
			Ok(ProcessedPropertyResolved {
				field_name: Some(resolved_name),
			})
		} else {
			println!("RESOLVED TO NONE {:?}", self.name);
			Ok(ProcessedPropertyResolved {
				field_name: None,
			})
		}
	}
}

/*struct IdentStore {
	stored: HashMap<String, String>,
}

impl IdentStore {
	fn new() -> IdentStore {
		IdentStore {
			stored: HashMap::new(),
		}
	}
	
	fn insert(&mut self, id: String, ident: String) -> Result<(), String> {
		if let Some(val) = self.stored.get(&id) {
			return Err(format!("Trying to overwrite {} with {}", val, ident));
		}
		self.stored.insert(id, ident);
		Ok(())
	}
	
	fn contains(&self, id: &str) -> bool {
		self.stored.contains_key(id)
	}
	
	fn get(&self, id: &str) -> Option<&str> {
		self.stored.get(id).into()
	}
}*/

/*struct NameScope {
	/*names_to_ids: HashMap<String, usize>,
	ids_to_names: HashMap<usize, String>,*/
	taken_names: HashSet<String>,
}

impl NameScope {
	/*fn new_ref(&mut self, name: String) -> NameRef {
		let id = get_new_id();
		self.names_to_ids.insert(name, id);
		self.ids_to_names.insert(id, name);
		NameRef(id)
	}*/
}

struct NameRef(usize);*/

enum IdentType {
	None,
	Snake,
}

impl IdentType {
	fn apply(&self, name: &str) -> String {
		match self {
			IdentType::None => name.to_string(),
			IdentType::Snake => name.to_snake_case(),
		}
	}
}

struct IdentSource {
	store: HashSet<String>,
}

impl IdentSource {
	fn new() -> IdentSource {
		IdentSource {
			store: HashSet::new(),
		}
	}
	
	fn take_all(&mut self, names: &[&str]) {
		for name in names {
			self.store.insert(name.to_string());
		}
	}

	fn get(&mut self, from: &syn::Ident, ty: IdentType) -> syn::Ident {
		let mut name = ty.apply(&from.to_string());
		if self.store.contains(&name) {
			let base_name = name.clone();
			name = format!("{}_", base_name);
			let mut current_num = 1;
			while self.store.contains(&name) {
				current_num += 1;
				name = format!("{}_{}", base_name, current_num.to_string());
			}
		}
		self.store.insert(name.clone());
		syn::Ident::new(&name, from.span())
	}
}

struct PropertyGraph {
	properties: Vec<ProcessedProperty>,
}

impl PropertyGraph {
	fn add_property(&mut self, property: ProcessedProperty) -> usize {
		let id = self.properties.len();
		self.properties.push(property);
		id
	}

	fn lookup_local_prop(&self, source_id: usize, find_name: &str) -> Option<usize> {
		for (id, prop) in self.properties.iter().enumerate() {
			if prop.parent == Some(source_id) {
				if let Some(ref prop_name) = prop.name {
					if &prop_name.to_string() == find_name {
						//println!("looking for {:?} in {:?}, found {:?}", find_name, source_id, id);
						return Some(id);
					}
				}
			}
		}
		
		None
	}
	
	fn lookup_prop(&self, source_id: usize, find_name: &str) -> Option<usize> {
		// Search for locals:
		if let Some(id) = self.lookup_local_prop(source_id, find_name) {
			return Some(id);
		}
		
		// Search for instances:
		for (id, prop) in self.properties.iter().enumerate() {
			if prop.is_inst {
				if let Some(ref prop_name) = prop.name {
					if &prop_name.to_string() == find_name {
						return Some(id);
					}
				}
			}
		}
		
		None
	}
}

fn get_change_type(ty: &syn::Type) -> ParseResult<syn::Type> {
	let mut change_ty = ty.clone();
	if let syn::Type::Path(ref mut path) = change_ty {
		if let Some(ref mut last_seg) = path.path.segments.last_mut() {
			let mut ty_ident = last_seg.value_mut();
			let change_name = ty_ident.ident.to_string() + "Change";
			ty_ident.ident = syn::Ident::new(&change_name, ty_ident.span());
		} else {
			return Err(syn::parse::Error::new(ty.span(), "Type path has no parts"))
		}
	} else {
		return Err(syn::parse::Error::new(ty.span(), "Can't generate change type for non-path type"))
	}
	Ok(change_ty)
}

pub struct FinalDesc {
	struct_name: Option<syn::Ident>,
	prop_graph: PropertyGraph,
	bindings: Vec<ProcessedBinding>,
	ident_source: IdentSource,
}

/*fn get_change_type(from_type: &syn::Type) -> syn::Type {
	/*match from_type {
		syn::Type::
	}*/
	//syn::Ident::new(&(struct_name.to_string() + "Change"), struct_name.span())
	println!("{:?}", from_type);
	from_type.clone()
}*/

impl FinalDesc {
	pub fn new() -> FinalDesc {
		let mut ident_source = IdentSource::new();
		
		ident_source.take_all(&[
			"Self", "abstract", "alignof", "as", "become", "box", "break", "const", "continue",
			"crate", "do", "else", "enum", "extern crate", "extern", "false", "final", "fn", "for",
			"for", "if let", "if", "if", "impl", "impl", "in", "let", "loop", "macro", "match",
			"mod", "move", "mut", "offsetof", "override", "priv", "proc", "pub", "pure", "ref",
			"return", "self", "sizeof", "static", "struct", "super", "trait", "true", "type",
			"typeof", "unsafe", "unsized", "use", "use", "virtual", "where", "while", "yield",
		]);
		
		FinalDesc {
			struct_name: None,
			prop_graph: PropertyGraph {
				properties: vec![],
			},
			bindings: vec![],
			ident_source,
		}
	}
	
	pub fn process_instance(&mut self, instance: &ast::Instance, parent: Option<usize>) -> ParseResult<()> {
		if parent.is_none() {
			let instance_name: ast::InstanceName = syn::parse2(instance.type_name.clone().into_token_stream())?;
			self.struct_name = Some(instance_name.name);
		}
		
		let name = if parent.is_none() {
			// TODO: Ensure root is not called something other than root.
			Some(syn::Ident::new("root", instance.type_name.span()))
		} else {
			instance.name.clone()
		};
		
		//let change_type_name = instance.type_name.to_string() + "Change";
		//let change_type = syn::Ident::new(&change_type_name, instance_name.span());
		let change_type = get_change_type(&instance.type_name)?;
		
		let processed = ProcessedProperty {
			name,
			main_type: instance.type_name.clone(),
			change_type,
			parent,
			is_inst: true,
			//is_mouse_item: false,
			//anchors: Anchors::default(),
			resolved: None,
		};
		
		let id = self.prop_graph.add_property(processed);
		
		for property in &instance.body.properties {
			//self.properties.push(property.clone());
			self.process_property(property, id);
		}
		
		for binding in &instance.body.bindings {
			self.process_binding_expr(&binding.body, id, PropPath {
				prop_id: id,
				sub_prop_path: vec![binding.prop_name.clone()],
			});
		}
		
		for child in &instance.body.children {
			self.process_instance(child, Some(id))?;
		}
		
		Ok(())
	}
	
	fn process_property(&mut self, property: &ast::Property, parent: usize) -> ParseResult<()> {
		let change_type_toks = if let Some(ast::PropertyChangeType{ref change_ty}) = property.change_ty {
			quote! {
				#change_ty
			}
		} else {
			let ty = &property.ty;
			quote! {
				::modelone::change_value::ValueChange<#ty>
			}
		};
		
		let change_type = syn::parse2(change_type_toks)?;

		let processed = ProcessedProperty {
			name: Some(property.name.clone()),
			main_type: property.ty.clone(),
			change_type,
			parent: Some(parent),
			is_inst: false,
			resolved: None,
		};
		
		let id = self.prop_graph.add_property(processed);
		
		if let Some((_, ref binding_expr)) = property.binding_expr {
			self.process_binding_expr(binding_expr, parent, PropPath {
				prop_id: id,
				sub_prop_path: vec![],
			});
		}
		
		Ok(())
	}
	
	fn process_binding_expr(&mut self, binding_expr: &ast::BindingExpr, context_prop: usize, for_prop: PropPath) -> ParseResult<()> {
		self.bindings.push(ProcessedBinding {
			context_prop,
			for_prop,
			expr: binding_expr.expr.clone(),
			resolved: None,
		});
		Ok(())
	}
	
	pub fn resolve(&mut self) -> ParseResult<()> {
		for i in 0 .. self.prop_graph.properties.len() {
			println!("resolve {}: {:?}", i, self.prop_graph.properties[i]);
			let resolved = self.prop_graph.properties[i].resolve_locals(&self.prop_graph, &mut self.ident_source)?;
			self.prop_graph.properties[i].resolved = Some(resolved);
		}
		
		for binding in &mut self.bindings {
			println!("Resolving BINDING {:?}", binding);
			let resolved = binding.resolve_deps(&self.prop_graph)?;
			binding.resolved = Some(resolved);
		}
		
		Ok(())
	}

	pub fn generate(&self) -> TokenStream {
		let struct_name = self.struct_name.clone().expect("Struct name");
		let change_name = syn::Ident::new(&(struct_name.to_string() + "ChangePrivate"), struct_name.span());
		let signal_name = syn::Ident::new(&(struct_name.to_string() + "SignalPrivate"), struct_name.span());
		let pub_change_name = syn::Ident::new(&(struct_name.to_string() + "Change"), struct_name.span());
		let pub_signal_name = syn::Ident::new(&(struct_name.to_string() + "Signal"), struct_name.span());
		let args_struct_name = syn::Ident::new(&(struct_name.to_string() + "Builder"), struct_name.span());

		let mut args_struct_fields = TokenStream::new();
		let mut struct_fields = TokenStream::new();
		let mut struct_impl = TokenStream::new();
		let mut change_entries = TokenStream::new();
		let mut signal_entries = TokenStream::new();
		let mut changeable_apply_entries = TokenStream::new();
		let mut reset_view_entries = TokenStream::new();
		
		for property in &self.prop_graph.properties {
			let parent_id = if let Some(parent_id) = property.parent {
				parent_id
			} else {
				continue;
			};
			
			let ref parent = self.prop_graph.properties[parent_id];
			
			let field_name = &property.resolved.as_ref().expect("Resolved property").field_name;
			let field_type = &property.main_type;
			let change_type = &property.change_type;
			/*let change_type = if let Some(PropertyChangeType{ref change_ty}) = property.change_ty {
				quote! {
					#change_ty
				}
			} else {
				quote! {
					::modelone::change_value::ValueChange<#field_type>
				}
			};*/
			//let field_change_type = get_change_type(field_type);
			let field_make_change = syn::Ident::new(&(field_name.as_ref().expect("Field name").to_string() + "_change"), field_name.span());
			
			if parent.parent.is_none() && !property.is_inst {
				args_struct_fields.extend(quote! {
					#field_name: Option<#field_type>,
				});
			}
			struct_fields.extend(quote! {
				#field_name: #field_type,
			});
			change_entries.extend(quote! {
				#field_name(#change_type),
			});
			signal_entries.extend(quote! {
				#field_name(<#change_type as ::modelone::model::Change>::SignalType),
			});
			
			struct_impl.extend(quote! {
				fn #field_make_change(change: #change_type) -> #pub_change_name {
					#pub_change_name(#change_name::#field_name(change))
				}
			});
			
			/*if let Some(ref binding_expr) = property.binding_expr {
				let binding_update = syn::Ident::new(&(field_name.to_string() + "_update"), field_name.span());
				let expr_fn = syn::Ident::new(&(field_name.to_string() + "_binding_expr"), field_name.span());
				let resolved_binding_expr = &binding_expr.expr;
				struct_impl.extend(quote! {
					pub fn #binding_update(&self) -> #pub_change_name {
						// Using a move lambda here prevents the expression from using &self.
						(move || {
							#struct_name::#field_make_change(::modelone::change_value::ValueChange(#resolved_binding_expr))
						})()
					}
				});
			}*/
			
			changeable_apply_entries.extend(quote! {
				#change_name::#field_name(subchange) => {
					let mut watcher_fn = |signal| {
						watcher.send_signal(#pub_signal_name(#signal_name::#field_name(signal)));
					};
					
					/*$(
						impl_changeable_body!($mod #name ($($arg)*) $body);
						$mod(self);
					)*;*/
					
					self.#field_name.changeable_apply(subchange, &mut ::modelone::model::SubWatcher::new(&mut watcher_fn));
				},
			});
			
			reset_view_entries.extend(quote! {
				let changeable: &::modelone::model::Changeable<#change_type> = &self.#field_name;
				for subsignal in changeable.reset_view_signals() {
					signals.push(#pub_signal_name(#signal_name::#field_name(subsignal)));
				}
			});
		}
		
		for binding in &self.bindings {
			//let binding_update = syn::Ident::new(&(field_name.to_string() + "_update"), field_name.span());
			let for_prop = &self.prop_graph.properties[binding.for_prop.prop_id];
			println!("for_prop: {:?}", for_prop);
			let for_field_name = for_prop.resolved.as_ref().expect("Resolved for_prop").field_name.as_ref().expect("Field name for for_prop");
			let update_fn = syn::Ident::new(&(for_field_name.to_string() + "_update"), for_field_name.span());
			
			let expr_toks = binding.gen_expr(&|prop_path| {
				let prop = &self.prop_graph.properties[prop_path.prop_id];
				let resolved_prop = prop.resolved.as_ref().expect("Resolved prop");
				println!("for_prop: {:?}", binding);
				let name = resolved_prop.field_name.as_ref().expect("Field name for prop");
				//let name = format!("{:?}", prop_path);
				println!("resolved prop: {:?}, {:?}, {:?}", prop_path, name, prop);
				quote!{self.#name}
			});
			struct_impl.extend(quote! {
				fn #update_fn(&self) -> #pub_change_name {
					#pub_change_name(#change_name::#for_field_name(#expr_toks))
				}
			});
		}
		
		/*for instance in &self.instances {
			if instance.parent.is_some() {
				let field_name = instance.resolved_field_name.clone().expect("No resolved field name");
				let field_type = instance.type_name.clone();
				let change_type_name = field_type.to_string() + "Change";
				let change_type = syn::Ident::new(&change_type_name, field_type.span());
				struct_fields.extend(quote! {
					#field_name: #field_type,
				});
				change_entries.extend(quote! {
					#field_name(#change_type),
				});
				signal_entries.extend(quote! {
					#field_name(<#change_type as ::modelone::model::Change>::SignalType),
				});
			}
		}*/
		
		quote! {
			pub struct #args_struct_name {
				#args_struct_fields
			}
		
			pub struct #struct_name {
				#struct_fields
			}
			
			impl #struct_name {
				#struct_impl
			}
			
			#[allow(non_camel_case_types)]
			#[derive(Debug, Clone, PartialEq)]
			enum #change_name {
				#change_entries
			}
			
			#[derive(Debug, Clone, PartialEq)]
			pub struct #pub_change_name(#change_name);
			
			#[allow(non_camel_case_types)]
			#[derive(Debug, Clone, PartialEq)]
			enum #signal_name {
				#signal_entries
			}
			
			#[derive(Debug, Clone, PartialEq)]
			pub struct #pub_signal_name(#signal_name);
			
			impl ::modelone::model::Change for #pub_change_name {
				type SignalType = #pub_signal_name;
			}
			
			impl ::modelone::model::Changeable<#pub_change_name> for #struct_name {
				fn changeable_apply(&mut self, change: #pub_change_name, watcher: &mut ::modelone::model::Watcher<#pub_signal_name>) {
					match change.0 {
						#changeable_apply_entries
					}
				}
				
				fn reset_view_signals(&self) -> Vec<#pub_signal_name> {
					let mut signals = vec![];
					#reset_view_entries
					signals
				}
			}
		}
	}
}
