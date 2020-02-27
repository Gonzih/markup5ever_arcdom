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
extern crate markup5ever_arcdom as arcdom;
extern crate xml5ever;

use std::default::Default;

use arcdom::{ArcDom, NodeData};
use xml5ever::driver::parse_document;
use xml5ever::tendril::TendrilSink;
use xml5ever::tree_builder::TreeSink;

fn main() {
    // To parse a string into a tree of nodes, we need to invoke
    // `parse_document` and supply it with a TreeSink implementation (ArcDom).
    let dom: ArcDom =
        parse_document(ArcDom::default(), Default::default()).one("<hello>XML</hello>");

    // Do some processing
    let doc = &dom.document;

    let hello_node = &doc.children.borrow()[0];
    let hello_tag = &*dom.elem_name(hello_node).local;
    let text_node = &hello_node.children.borrow()[0];

    let xml = {
        let mut xml = String::new();

        match &text_node.data {
            &NodeData::Text { ref contents } => {
                xml.push_str(&contents.borrow());
            }
            _ => {}
        };

        xml
    };

    println!("{:?} {:?}!", hello_tag, xml);
}
