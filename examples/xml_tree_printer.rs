#!/usr/bin/env run-cargo-script
//! This is a regular crate doc comment, but it also contains a partial
//! Cargo manifest.  Note the use of a *fenced* code block, and the
//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! xml5ever = "0.2.0"
//! tendril = "0.1.3"
//! ```
extern crate markup5ever_rcdom as rcdom;
extern crate xml5ever;

use std::default::Default;
use std::io;
use std::string::String;

use rcdom::{ArcDom, Handle, NodeData};
use xml5ever::driver::parse_document;
use xml5ever::tendril::TendrilSink;

fn walk(prefix: &str, handle: &Handle) {
    let node = handle;

    print!("{}", prefix);
    match node.data {
        NodeData::Document => println!("#document"),

        NodeData::Text { ref contents } => println!("#text {}", escape_default(&contents.borrow())),

        NodeData::Element { ref name, .. } => {
            println!("{}", name.local);
        }

        _ => {}
    }

    let new_indent = {
        let mut temp = String::new();
        temp.push_str(prefix);
        temp.push_str("    ");
        temp
    };

    for child in node
        .children
        .borrow()
        .iter()
        .filter(|child| match child.data {
            NodeData::Text { .. } | NodeData::Element { .. } => true,
            _ => false,
        })
    {
        walk(&new_indent, child);
    }
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn main() {
    let stdin = io::stdin();

    // To parse XML into a tree form, we need a TreeSink
    // luckily xml5ever comes with a static RC backed tree represetation.
    let dom: ArcDom = parse_document(ArcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap();

    // Execute our visualizer on ArcDom
    walk("", &dom.document);
}
