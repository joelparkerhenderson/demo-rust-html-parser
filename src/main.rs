#[macro_use] extern crate html5ever;

use std::env;
use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::iter::repeat;
use std::default::Default;
use std::string::String;
use std::path::Path;
use std::ffi::OsStr;

use html5ever::rcdom::{Node, NodeData, RcDom, Handle};
use html5ever::tendril::TendrilSink;
use html5ever::{ParseOpts, parse_document};
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::tendril::*;

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_indent_args_zero() {
        assert_eq!("a", indent(0, "a"));
    }

    #[test]
    fn test_indent_args_some() {
        assert_eq!("      a", indent(3, "a"));
    }

    #[test]
    fn test_indent_unit() {
        assert_eq!("  ", indent_unit());
    }

    #[test]
    fn test_indent_units_args_zero() {
        assert_eq!("", indent_units(0));
    }

    #[test]
    fn test_indent_units_args_some() {
        assert_eq!("      ", indent_units(3));
    }

    #[test]
	fn test_parse_html_string_with_blank() {
		assert_eq!(
			"#Document\n  <html>\n    <head>\n    <body>\n",
			parse_html_string("")
		);
	}

    #[test]
	fn test_parse_html_string_with_single_tag() {
		assert_eq!(
			"#Document\n  <html>\n    <head>\n    <body>\n      <foo>\n",
			parse_html_string("<foo/>")
		);
	}

    #[test]
	fn test_parse_html_string_with_single_tag_with_attribute() {
		assert_eq!(
			"#Document\n  <html>\n    <head>\n    <body>\n      <foo goo=\"hoo\">\n", 
			parse_html_string("<foo goo=\"hoo\"/>")
		);
	}

    #[test]
	fn test_parse_html_string_with_double_tag() {
		assert_eq!(
			"#Document\n  <html>\n    <head>\n    <body>\n      <foo>\n",
			parse_html_string("<foo></foo>")
		);
	}

    #[test]
	fn test_parse_html_string_with_text() {
		assert_eq!(
			"#Document\n  <html>\n    <head>\n    <body>\n      #text:foo\n",
			parse_html_string("foo")
		);
	}

	#[test]
	fn test_parse_html_string_with_comment() {
		assert_eq!(
			"#Document\n  <!--  foo  -->\n  <html>\n    <head>\n    <body>\n", 
			parse_html_string("<!-- foo -->")
		);
	}

}

//////////////////////////////////////////////////////////////////////////////
//
// Text utilities
//
//////////////////////////////////////////////////////////////////////////////

fn indent(size: usize, s: &str) -> String {
    format!("{}{}", indent_units(size), s)
}

fn indent_unit() -> String {
    "  ".to_string()
}

fn indent_units(size: usize) -> String {
    indent_unit().repeat(size)
}

// FIXME: Copy of str::escape_default from std, which is currently unstable
pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}
 
//////////////////////////////////////////////////////////////////////////////

fn node_to_string(node: &Handle) -> String {
    match node.data {

        NodeData::Document => {
            "#Document".to_string()
        }

        NodeData::Doctype { ref name, ref public_id, ref system_id } => {
            format!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id)
        }

        NodeData::Text { ref contents } => {
            format!("#text:{}", escape_default(&contents.borrow()))
        }

        NodeData::Comment { ref contents } => {
            format!("<!-- {} -->", escape_default(contents))
        }

        NodeData::Element { ref name, ref attrs, .. } => {
            assert!(name.ns == ns!(html));
            let mut s = format!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                s.push_str(&format!(" {}=\"{}\"", attr.name.local, attr.value));
            }
            s.push_str(">");
            s
        }

        NodeData::ProcessingInstruction { .. } => unreachable!()
    }	
}

fn walk(node: Handle, indent_size: usize) -> String {
    let mut s = format!("{}{}\n", indent_units(indent_size), node_to_string(&node));
    for child in node.children.borrow().iter() {
        s.push_str(&walk(child.clone(), indent_size + 1));
    }
    s
}

pub fn parse_html_string(s: &str) -> String {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut s.as_bytes())
        .unwrap();

    let s = walk(dom.document, 0);

    if !dom.errors.is_empty() {
        println!("\nParse errors:");
        for err in dom.errors.into_iter() {
            println!("    {}", err);
        }
    }
    s
}

fn parse_html_path(path: &Path) -> String {
	println!("parse_html_path path:{:?}", path);
    let mut file = File::open(path)
    .expect("file open failed");
    let mut s = String::new();
    file.read_to_string(&mut s)
    .expect("file read failed");
    parse_html_string(&s)
}

fn main() {
	let mut args = std::env::args(); args.next();
    let arg: String = args.next().expect("Missing arg");
    let html_path = Path::new(&arg);
    println!("{}",parse_html_path(html_path));
}
