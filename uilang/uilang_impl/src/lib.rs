#![recursion_limit="256"]

extern crate proc_macro as original_proc_macro;

mod ast;
mod ir;

use syn::parse::Result as ParseResult;
use syn::parse::Error as ParseError;
use syn::parse::{Parse, ParseStream};
use syn::Token;
use proc_macro2::{TokenStream, TokenTree, Delimiter};

/*
struct Thing {
	items: Vec<String>,
}

The compiler could actually generate private changes:
enum PrivateThingChange {
	AddItem(usize, String),
	SecretThing(String),
}
// Note that this is pub, but the inner is not.
pub struct ThingChange(PrivateChange);

impl ThingChange {
	pub fn add_item(data: (usize, String)) -> ThingChange {
		ThingChange(PrivateThingChange::AddItem(data.0, data.1)
	}
}

*/

/*static CURRENT_ID: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

fn get_new_id() -> usize {
	CURRENT_ID.fetch_add(1, atomic::Ordering::SeqCst)
}*/

fn print_syn(toks: TokenStream) {
	print_syn_helper(toks, 0, Delimiter::Brace);
}

fn print_syn_helper(toks: TokenStream, ind: usize, parent_delim: Delimiter) {
	let ind_str = "    ".repeat(ind);
	print!("{}", ind_str);
	let mut prev_word = false;
	for tok in toks {
		match tok {
			TokenTree::Ident(ident) => {
				match ident.to_string().as_str() {
					"impl" | "struct" | "enum" => {
						print!("\n{}", ind_str);
					}
					"as" => print!(" "),
					_ => {
						if prev_word { print!(" "); }
					}
				}
				print!("{}", ident);
				match ident.to_string().as_str() {
					"as" => print!(" "),
					_ => {}
				}
				prev_word = true;
			}
			TokenTree::Punct(punct) => {
				match punct.as_char() {
					'#' => print!("\n{}", ind_str),
					'=' if prev_word => print!(" "),
					_ => {}
				}
				print!("{}", punct);
				
				match punct.as_char() {
					',' => {
						if parent_delim == Delimiter::Brace {
							print!("\n{}", ind_str);
						} else {
							print!(" ");
						}
					}
					';' => print!("\n{}", ind_str),
					':' if punct.spacing() == proc_macro2::Spacing::Alone => print!(" "),
					_ => {}
				}
				
				prev_word = false;
				//prev_punct = true;
			}
			TokenTree::Literal(lit) => {
				print!("{} ", lit);
				prev_word = false;
			}
			TokenTree::Group(group) => {
				let mut next_ind = 0;
				match group.delimiter() {
					Delimiter::Parenthesis => print!("("),
					Delimiter::Brace => {
						println!(" {{");
						next_ind = ind + 1;
					}
					Delimiter::Bracket => print!("["),
					Delimiter::None => print!("("),
				}
				print_syn_helper(group.stream(), next_ind, group.delimiter());
				match group.delimiter() {
					Delimiter::Parenthesis => print!(") "),
					Delimiter::Brace => print!("\n{}}}\n{}", ind_str, ind_str),
					Delimiter::Bracket => print!("] "),
					Delimiter::None => print!(") "),
				}
				prev_word = false;
			}
		}
	}
}

#[proc_macro_attribute]
pub fn item_builder(_attributes: original_proc_macro::TokenStream, tokens: original_proc_macro::TokenStream) -> original_proc_macro::TokenStream {
	item_builder2(tokens.into()).into()
}
fn item_builder2(tokens: TokenStream) -> TokenStream {
    let root_item: ParseResult<RootItem> = syn::parse2(tokens);
	println!("{:#?}", root_item);
	match root_item {
		Ok(root_item) => {
			let mut desc = ir::FinalDesc::new();
			desc.process_instance(&root_item.root_instance, None);
			desc.resolve();
			let out_stream = desc.generate();
			print_syn(out_stream.clone());
			//println!("{:#?}", out_stream);
			
			
			//let out_str = out_stream.to_string();
			//println!("{:?}", &out_str);
			//let out_syn: ParseResult<syn::File> = syn::parse_str(&out_str);
			//println!("{:?}", out_syn);
			/*if let Err(e) = out_syn {
				println!("{:?}", e.to_compile_error());
				return e.to_compile_error();
			}*/
			out_stream
		}
		Err(err) => {
			return TokenStream::from(err.to_compile_error());
		}
	}
}

/*fn item_builder_impl(tokens: TokenStream) -> TokenStream {
	
	//let mut parser = DefParser::parse(ident);
	
	/*let mut ast: syn::Item = syn::parse(input.into()).expect("Failed to parse AST");
	println!("{:#?}", ast);
	if let syn::Item::Macro(syn::ItemMacro{ident, mac, ..}) = ast {
		if let Some(ident) = ident {
			println!("{:#?}", ident.to_string());
			parser.parse_body(&mut mac.tts.into_iter());
		} else {
			// Error
		}
	} else {
		// Error
	}*/
	quote! {
		struct Thing {}
	}
}*/

#[derive(Debug)]
struct RootItem {
	item_keyword: syn::Ident,
	bang: syn::token::Bang,
	root_instance: ast::Instance,
	//end_semi: Token![;],
}

impl Parse for RootItem {
	fn parse(input: ParseStream) -> ParseResult<RootItem> {
		let item_keyword: syn::Ident = input.parse()?;
		if item_keyword != "item" {
			return Err(ParseError::new(item_keyword.span(), "expected `item`"));
		}
		let bang: Token![!] = input.parse()?;
		//let name: syn::Ident = input.parse()?;
		let group: proc_macro2::Group = input.parse()?;
		//println!("group {:?}", group);
		//println!("{:?}", mac);
		//let def: Def = syn::parse2(group.stream())?;
		//let root_instance: Instance = group.stream().parse()?;
		let root_instance: ast::Instance = syn::parse2(group.stream())?;
		
		//let end_semi: Token![;] = input.parse()?;
		
		let root_item = RootItem {
			item_keyword,
			bang,
			root_instance,
			//end_semi,
		};
		let toks: TokenStream = input.parse()?;
		println!("extra toks {:?}", toks);
		
		//let bang: Token![!] = input.parse()?;
		//let mac: Token![!] = input.parse()?;
		Ok(root_item)
	}
}

/*impl DefParser {
	fn new(name: syn::Ident) -> DefParser {
		DefParser {
			name,
		}
	}
	
	fn parse_body(&mut self, stream: ParseStream) {
		while let Some(tok) = toks.next() {
			if let TokenTree::Ident(ident) = tok {
				match ident.to_string().as_ref() {
					"prop" => self.parse_prop(toks),
					_ => {}
				}
			}
			//println!("{:?}", token);
		}
	}
	
	fn parse_prop(&mut self, toks: &mut TokenIter) {
		let name = toks.next().expect("Prop name");
		let colon = toks.next().expect("Colon");
		println!("prop: {}", name);
	}
}*/
