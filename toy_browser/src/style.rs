use crate::css::{Rule, SimpleSelector, Specificity, Stylesheet};
use crate::dom::NodeType;
use crate::style::Selector::Simple;
use crate::{
    css::{Selector, Value},
    dom::{ElementData, Node},
};
use std::collections::HashMap;

/// Map from CSS property names to values.
pub type PropertyMap = HashMap<String, Value>;

/// `StyledNode` wraps `Node` with related CSS properties.
/// It forms a tree as `Node` does.
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

/// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
///
/// This finds only the specified values at the moment. Eventually it should be extended to find the
/// computed values too, including inherited values.
pub fn to_styled_node<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            NodeType::Text(_) => HashMap::new(),
        },
        children: root
            .children
            .iter()
            .map(|child| to_styled_node(child, stylesheet))
            .collect(),
    }
}

/// Apply styles to a single element, returning the specified styles.
///
/// To do: Allow multiple UA/author/user stylesheets, and implement the cascade.
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut rules = matching_rules(elem, stylesheet);

    // Sort rules by specificity
    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

    // Apply the declarations of each rule in order
    rules
        .iter()
        .flat_map(|&(_, rule)| &rule.declarations)
        .map(|declaration| (declaration.name.clone(), declaration.value.clone()))
        .collect()
}

/// A single CSS rule and the specificity of its most specific matching selector.
type MatchedRule<'a> = (Specificity, &'a Rule);

/// Find all CSS rules that match the given element.
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    // For now, we just do a linear scan of all the rules.
    // For large documents, it would be more efficient to store the rules in hash tables
    // based on tag name, id, class, etc.
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

/// If `rule` matches `elem`, return a `MatchedRule`. Otherwise return `None`.
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    // Find the first (most specific) matching selector.
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match selector {
        Simple(simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // Check if the type selector matches (if present)
    if let Some(tag_name) = &selector.tag_name {
        if elem.tag_name != *tag_name {
            return false;
        }
    }

    // Check if the ID selector matches (if present)
    if let Some(id) = &selector.id {
        if elem.id() != Some(id) {
            return false;
        }
    }

    // Check if all class selectors match
    selector
        .class
        .iter()
        .all(|class| elem.classes().contains(&**class))
}
