use syn;
use syn::Token;
use syn::parse::{Parse, ParseStream};
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use quote::ToTokens;
use proc_macro2::{self, TokenStream, TokenTree, Delimiter};
use std;

#[derive(Debug, Clone)]
pub struct BindingDep {
	pub inst_name: Option<syn::Ident>,
	pub prop_path: Vec<syn::Ident>,
}

impl Parse for BindingDep {
	fn parse(input: ParseStream) -> ParseResult<BindingDep> {
		let mut inst_name = None;
		let mut prop_path = vec![];
		let prop_name: syn::Ident = input.parse()?;
		prop_path.push(prop_name);
		while input.peek(Token![.]) || input.peek(Token![:]) {
			if input.peek(Token![:]) {
				let colon_tok: Token![:] = input.parse()?;
				let inst_path = std::mem::replace(&mut prop_path, vec![]);
				if let [found_inst_name] = inst_path.as_slice() {
					inst_name = Some(found_inst_name.clone());
				} else {
					return Err(syn::parse::Error::new(colon_tok.span(), "Instance name should not include dots"));
				}
				//inst_path = std::mem::replace(&mut prop_path, vec![]);
			} else {
				let _dot_tok: Token![.] = input.parse()?;
			}
			
			let prop_name = input.parse()?;
			prop_path.push(prop_name);
		}
		
		Ok(BindingDep {
			inst_name,
			prop_path,
		})
	}
}

#[derive(Debug, Clone)]
pub enum BindingExprPart {
	Tokens(TokenStream),
	Dep(BindingDep),
}

#[derive(Debug, Clone)]
pub struct BindingExpr {
	pub expr: Vec<BindingExprPart>,
}

impl BindingExpr {
	fn parse(input: ParseStream) -> ParseResult<BindingExpr> {
		let mut expr = vec![];
		let mut toks = TokenStream::new();
		while !input.peek(Token![;]) {
			let is_normal_tok = if input.peek(Token![#]) {
				let hash_tok: Token![#] = input.parse()?;
				
				if let Ok(group) = input.parse::<proc_macro2::Group>() {
					let push_toks = std::mem::replace(&mut toks, TokenStream::new());
					expr.push(BindingExprPart::Tokens(push_toks));
					
					if group.delimiter() == Delimiter::Brace {
						let expr_dep: BindingDep = syn::parse2(group.stream())?;
						expr.push(BindingExprPart::Dep(expr_dep));
						false
					} else {
						true
					}
				} else {
					toks.extend(hash_tok.into_token_stream());
					false
				}
			} else {
				true
			};
			
			if is_normal_tok {
				let tok: TokenTree = input.parse()?;
				toks.extend(std::iter::once(tok));
			}
		}
		expr.push(BindingExprPart::Tokens(toks));
		Ok(BindingExpr {
			expr,
		})
	}
}

#[derive(Debug)]
pub struct Binding {
	pub prop_name: syn::Ident,
	pub assign_token: Token![=],
	pub body: BindingExpr,
	//resolved_prop: Option<PropertyId>,
}

impl Binding {
	fn parse(input: ParseStream) -> ParseResult<Binding> {
		let prop_name: syn::Ident = input.parse()?;
		let assign_token: Token![=] = input.parse()?;
		let body = input.call(BindingExpr::parse)?;
		
		Ok(Binding {
			prop_name,
			assign_token,
			body,
		})
	}
}

#[derive(Debug, Clone)]
pub struct PropertyChangeType {
	pub change_ty: syn::Type,
}

impl Parse for PropertyChangeType {
	fn parse(input: ParseStream) -> ParseResult<PropertyChangeType> {
		let change_ty: syn::Type = input.parse()?;
		
		Ok(PropertyChangeType {
			change_ty,
		})
	}
}

#[derive(Debug, Clone)]
pub struct Property {
	pub name: syn::Ident,
	pub colon_token: Token![:],
	pub ty: syn::Type,
	pub change_ty: Option<PropertyChangeType>,
	pub binding_expr: Option<(Token![=], BindingExpr)>,
	pub semi_token: Token![;],
	
	pub resolved_field_name: Option<syn::Ident>,
}

impl Property {
	fn parse(input: ParseStream) -> ParseResult<Property> {
		let name: syn::Ident = input.parse()?;
		let colon_token: Token![:] = input.parse()?;
		let ty: syn::Type = input.parse()?;
		let mut change_ty = None;
		if let Ok(change_group) = input.parse::<proc_macro2::Group>() {
			change_ty = Some(syn::parse2(change_group.stream())?);
		}
		
		let binding_expr = if input.peek(Token![=]) {
			let assign_token = input.parse()?;
			let expr = input.call(BindingExpr::parse)?;
			Some((assign_token, expr))
		} else {
			None
		};
		
		let semi_token: Token![;] = input.parse()?;
		
		Ok(Property {
			name,
			colon_token,
			ty,
			change_ty,
			binding_expr,
			semi_token,
			resolved_field_name: None,
		})
	}
}

#[derive(Debug)]
pub struct InstanceBody {
	pub properties: Vec<Property>,
	pub bindings: Vec<Binding>,
	pub children: Vec<Instance>,
}

impl Parse for InstanceBody {
	fn parse(input: ParseStream) -> ParseResult<InstanceBody> {
		let mut properties = vec![];
		let mut bindings = vec![];
		let mut children = vec![];
		
		while let Ok(keyword) = input.fork().parse::<syn::Ident>() {
			match keyword.to_string().as_ref() {
				"prop" => {
					let _keyword_token = input.parse::<syn::Ident>();
					let property = Property::parse(input)?;
					properties.push(property);
				}
				_ => {
					if input.peek2(Token![=]) {
						let binding = Binding::parse(input)?;
						bindings.push(binding);
					} else {
						let child = input.call(Instance::parse)?;
						children.push(child);
					}
				}
			}
		}
		
		let toks: TokenStream = input.parse()?;
		println!("extra toks {:?}", toks);
		Ok(InstanceBody {
			properties,
			bindings,
			children,
		})
	}
}

#[derive(Debug, Clone)]
pub struct InstanceName {
	pub name: syn::Ident,
}

impl Parse for InstanceName {
	fn parse(input: ParseStream) -> ParseResult<InstanceName> {
		let name: syn::Ident = input.parse()?;
		
		Ok(InstanceName {
			name,
		})
	}
}

#[derive(Debug)]
pub struct Instance {
	pub name: Option<syn::Ident>,
	pub type_name: syn::Type,
	pub body: InstanceBody,
	end_semi: Option<Token![;]>,
}

impl Parse for Instance {
	fn parse(input: ParseStream) -> ParseResult<Instance> {
		let _name: Option<InstanceName> = None;
		
		/*let name_group: ParseResult<proc_macro2::Group> = input.parse();
		if let Ok(group) = name_group {
			if group.delimiter() == Delimiter::Parenthesis {
				name = Some(syn::parse2(group.stream())?);
				println!("{:?}", name);
			}
		}*/
		
		let type_name: syn::Type = input.parse()?;
		
		let name: Option<syn::Ident> = input.parse().ok();
		
		let group: proc_macro2::Group = input.parse()?;
		let body: InstanceBody = syn::parse2(group.stream())?;
		let end_semi: Option<Token![;]> = input.parse()?;
		
		//let toks: TokenStream = input.parse()?;
		//println!("extra toks {:?}", toks);
		
		let instance = Instance {
			name,
			type_name,
			body,
			end_semi,
		};
		
		Ok(instance)
	}
}
