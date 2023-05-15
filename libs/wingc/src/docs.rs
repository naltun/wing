use std::collections::BTreeMap;

use crate::{type_check::{TypeRef, Type, FunctionSignature, Class}, ast::{Symbol, Phase}, jsify::codemaker::CodeMaker};

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
			Type::Anything => unsupported(symbol, "any"),
			Type::Number => unsupported(symbol, "number"),
			Type::String => unsupported(symbol, "string"),
			Type::Duration => unsupported(symbol, "duration"),
			Type::Boolean => unsupported(symbol, "boolean"),
			Type::Void => unsupported(symbol, "void"),
			Type::Json => unsupported(symbol, "json"),
			Type::MutJson => unsupported(symbol, "mutable json"),
			Type::Optional(_) => unsupported(symbol, "optional"),
			Type::Array(_) => unsupported(symbol, "array"),
			Type::MutArray(_) => unsupported(symbol, "mutable array"),
			Type::Map(_) => unsupported(symbol, "map"),
			Type::MutMap(_) => unsupported(symbol, "mutable map"),
			Type::Set(_) => unsupported(symbol, "set"),
			Type::MutSet(_) => unsupported(symbol, "mutable set"),
			Type::Function(f) => render_function(symbol, f),
			Type::Class(c) => render_class(c),
			Type::Resource(c) => render_class(c),
			Type::Interface(_) => unsupported(symbol, "interface"),
			Type::Struct(_) => unsupported(symbol, "struct"),
			Type::Enum(_) => unsupported(symbol, "enum"),
		}
	}
}

fn unsupported(symbol: &Symbol, t: &str) -> String {
	format!("{}, Unsupport {t}", symbol).to_string()
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

	markdown.empty_line();

	if !f.parameters.is_empty() {
		markdown.line("### Parameters");
		for p in &f.parameters {
			if let Some(s) = &p.docs.summary { 
				let name = p.clone().name.unwrap_or("p".to_string());
				markdown.line(format!(" * *{}* - {}", name, s));
			}
		}	
	}

	if let Some(s) = &f.docs.returns { 
		markdown.empty_line();
		markdown.line("### Returns");
		markdown.line(s);
	}

	if let Some(s) = &f.docs.remarks { 
		markdown.empty_line();
		markdown.line("### Remarks");
		markdown.line(s); 
	}
	
	if let Some(s) = &f.docs.example { 
		markdown.empty_line();
		markdown.line("### Example");
		markdown.line("```wing");
		markdown.line(s); 
		markdown.line("```");
	}


	markdown.empty_line();

	if let Some(_) = &f.docs.deprecated { 
		markdown.line("@deprecated"); 
	}

	f.docs.custom.iter().for_each(|(k, v)| {
		let value = if v == "true" { String::default() } else { format!("*{v}*") };
		markdown.line(format!("*@{}* {}", k, value));
	});

	if let Some(s) = &f.docs.see { 
		markdown.empty_line();
		markdown.line(format!("See [{}]({})", s, s));
	}

	markdown.to_string().trim().to_string()

	// pub remarks: Option<String>,
	// pub example: Option<String>,

	// pub returns: Option<String>,
	// pub deprecated: Option<String>,
	// pub see: Option<String>,

	// pub custom: Option<::std::collections::BTreeMap<String, String>>,


}

fn render_class(c: &Class) -> String {
	c.docs.summary.clone().unwrap_or_else(|| "I am a class".to_string())
}