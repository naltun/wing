use std::collections::BTreeMap;

use itertools::Itertools;

use crate::{type_check::{TypeRef, Type, FunctionSignature, Class, jsii_importer::is_construct_base, VariableInfo, ClassLike, CLASS_INIT_NAME}, ast::{Symbol, Phase}, jsify::codemaker::CodeMaker};

#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
pub struct Docs {
	pub summary: Option<String>,
	pub remarks: Option<String>,
	pub example: Option<String>,

	pub returns: Option<String>,
	pub deprecated: Option<String>,
	pub see: Option<String>,

	pub custom: BTreeMap<String, String>,

	// ---
	pub default: Option<String>,
	pub stability: Option<String>,
	pub subclassable: Option<bool>,
}

pub trait Documented {
	fn render_docs(&self, symbol: &Symbol) -> String;
}

impl Docs {
	pub(crate) fn with_summary(summary: &str) -> Docs {
		Docs {
			summary: Some(summary.to_string()),
			..Default::default()
		}
	}

	pub(crate) fn with_error(message: &str) -> Docs {
		Self::with_summary(&format!("Error: {}", message))
	}

	pub(crate) fn unsupported(element: &str) -> Docs {
		Self::with_summary(&format!("*docs are not supported for {element} yet*"))
	}
}

impl Documented for TypeRef {
	fn render_docs(&self, symbol: &Symbol) -> String {
		match &**self {
			Type::Function(f) => render_function(symbol, f),
			Type::Class(c) => render_class(c),
			Type::Resource(c) => render_class(c),
			_ => {
				let mut markdown = CodeMaker::default();
				markdown.line("```wing");
				markdown.line(format!("{}", self));
				markdown.line("```");
			
				markdown.to_string().trim().to_string()
			}
			// Type::Anything => unsupported(symbol, "any"),
			// Type::Number => unsupported(symbol, "number"),
			// Type::String => unsupported(symbol, "string"),
			// Type::Duration => unsupported(symbol, "duration"),
			// Type::Boolean => unsupported(symbol, "boolean"),
			// Type::Void => unsupported(symbol, "void"),
			// Type::Json => unsupported(symbol, "json"),
			// Type::MutJson => unsupported(symbol, "mutable json"),
			// Type::Optional(_) => unsupported(symbol, "optional"),
			// Type::Array(_) => unsupported(symbol, "array"),
			// Type::MutArray(_) => unsupported(symbol, "mutable array"),
			// Type::Map(_) => unsupported(symbol, "map"),
			// Type::MutMap(_) => unsupported(symbol, "mutable map"),
			// Type::Set(_) => unsupported(symbol, "set"),
			// Type::MutSet(_) => unsupported(symbol, "mutable set"),
			// Type::Interface(_) => unsupported(symbol, "interface"),
			// Type::Struct(_) => unsupported(symbol, "struct"),
			// Type::Enum(_) => unsupported(symbol, "enum"),
		}
	}
}

impl Documented for VariableInfo {
	fn render_docs(&self, symbol: &Symbol) -> String {
		let mut markdown = CodeMaker::default();
		markdown.line("```wing");
		markdown.line(format!("{}: {}", symbol.name, self.type_));
		markdown.line("```");
	
		markdown.to_string().trim().to_string()
	}
}



fn render_docs(markdown: &mut CodeMaker, docs: &Docs) {
	if let Some(s) = &docs.returns { 
		markdown.empty_line();
		markdown.line("### Returns");
		markdown.line(s);
	}

	if let Some(s) = &docs.remarks { 
		markdown.empty_line();
		markdown.line("### Remarks");
		markdown.line(s); 
	}
	
	if let Some(s) = &docs.example { 
		markdown.empty_line();
		markdown.line("### Example");
		markdown.line("```wing");
		markdown.line(s); 
		markdown.line("```");
	}

	markdown.empty_line();

	if let Some(_) = &docs.deprecated { 
		markdown.line("@deprecated"); 
	}

	docs.custom.iter().for_each(|(k, v)| {
		// skip "@inflight" because it is already included in the type system
		if k == "inflight" { return; }

		let value = if v == "true" { String::default() } else { format!("*{v}*") };
		markdown.line(format!("*@{}* {}", k, value));
	});

	if let Some(s) = &docs.see { 
		markdown.empty_line();
		markdown.line(format!("See [{}]({})", s, s));
	}
}

fn render_function(symbol: &Symbol, f: &FunctionSignature) -> String {
	let phase_str = match f.phase {
		Phase::Inflight => "inflight ",
		Phase::Preflight => "preflight ",
		Phase::Independent => "",
	};

	let params_str = f
		.parameters
		.iter()
		.map(|a| match a.name {
			Some(ref name) => format!("{}: {}", name, a._type),
			None => format!("{}", a._type),
		})
		.collect::<Vec<String>>()
		.join(", ");

	let ret_type_str = f.return_type.to_string();

	let mut markdown = CodeMaker::default();

	markdown.line("```wing");
	markdown.line(format!("{phase_str}{symbol}({params_str}): {ret_type_str}"));
	markdown.line("```");
	markdown.line("---");

	if let Some(s) = &f.docs.summary { 
		markdown.line(s); 
	}

	if !f.parameters.is_empty() {
		markdown.empty_line();
		markdown.line("### Parameters");

		for p in &f.parameters {
			if let Some(s) = &f.docs.summary { 
				let name = p.clone().name.unwrap_or("p".to_string());
				markdown.line(format!(" * *{}* - {}", name, s));
			}
		}	
	}

	if let Some(s) = &f.docs.summary { 
		markdown.line(s); 
	}

	render_docs(&mut markdown, &f.docs);

	markdown.to_string().trim().to_string()
}

fn render_class(c: &Class) -> String {
	let mut markdown = CodeMaker::default();

	markdown.line("```wing");

	let extends = if let Some(t) = render_typeref(&c.parent) {
		format!(" extends {}", t)
	} else {
		String::default()
	};

	let interfaces = c.implements.iter().map(|i| render_typeref(&Some(*i))).map(|i| i.unwrap_or_default()).collect_vec();
	let implements = if !interfaces.is_empty() {
		format!(" impl {}", interfaces.join(", "))
	} else {
		String::default()
	};

	markdown.line(format!("class {}{}{}", c.name, extends, implements));

	markdown.line("```");
	markdown.line("---");

	if let Some(s) = &c.docs.summary { 
		markdown.line(s); 
	}
	render_docs(&mut markdown, &c.docs);

	if let Some(initializer) = c.get_init() {
		let rfn = render_function(&CLASS_INIT_NAME.into(), &initializer);
		markdown.line(rfn);
	}

	markdown.to_string().trim().to_string()
}

fn render_typeref(typeref: &Option<TypeRef>) -> Option<String> {
	let Some(t) = typeref else {
		return None;
	};

	if let Some(class) = t.as_class_or_resource() {
		// if the base class is "Resource" or "Construct" then we don't need to render it
		if let Some(fqn) = &class.fqn {
			if is_construct_base(&fqn) {
				return None;
			}
		}
	}

	// use TypeRef's Display trait to render the type
	Some(t.to_string())
}