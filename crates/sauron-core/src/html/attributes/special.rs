//! special attributes which is treated differently
//!
//!
use super::{attr, Attribute, Value};
use crate::vdom::{Element, Node};

/// Special Node attributes that are treated differently
/// such as key and skip which both greatly affects the diffing algorithm
///
pub trait Special {
    /// return the value of "key" attribute
    fn get_key(&self) -> Option<&Value> {
        self.get_value("key")
    }

    /// get the first attribute value with this attribute name
    fn get_value<'a>(&'a self, att_name: &'static str) -> Option<&'a Value>;
}

impl<MSG> Special for Node<MSG> {
    fn get_value<'a>(&'a self, att_name: &'static str) -> Option<&'a Value> {
        self.attribute_value(&att_name)
            .and_then(|att_values| att_values.first().and_then(|v| v.get_simple()))
    }
}

impl<MSG> Special for Element<MSG> {
    fn get_value<'a>(&'a self, att_name: &'static str) -> Option<&'a Value> {
        self.attribute_value(&att_name)
            .and_then(|att_values| att_values.first().and_then(|v| v.get_simple()))
    }
}

/// creates a key attribute using a formatter
/// # Examples
/// ```rust
/// use sauron::{*, html::{*, attributes::*}};
///
/// let number = 42;
/// let button:Node<()> = button([key!("content-{}", 42)], [text("Click")]);
///
/// assert_eq!(node!{<button key=format!("content-42")>"Click"</button>}, button);
/// ```
#[macro_export]
macro_rules! key {
    ( $($arg: tt)* ) => {
        $crate::html::attributes::key(format!($($arg)*))
    };
}

/// key attributes is used to match
/// old element and new element when diffing
pub fn key<V, MSG>(v: V) -> Attribute<MSG>
where
    V: Into<Value>,
{
    attr("key", v)
}

/// if the value is true, then the diffing of this element
/// and its descendants are skip entirely
pub fn skip<MSG>(v: bool) -> Attribute<MSG> {
    attr("skip", v)
}

/// if the value of this attribute of the old element and the new element is the same
/// the diffing of this element and its descentdants are skip entirely
pub fn skip_criteria<V, MSG>(v: V) -> Attribute<MSG>
where
    V: Into<Value>,
{
    attr("skip_criteria", v.into())
}

/// if the value is true, then this node is made to replace the old
/// node it matches
pub fn replace<MSG>(v: bool) -> Attribute<MSG> {
    attr("replace", v)
}
