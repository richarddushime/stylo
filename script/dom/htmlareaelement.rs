/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLAreaElementBinding;
use dom::bindings::codegen::InheritTypes::HTMLAreaElementDerived;
use dom::bindings::js::{JSRef, Temporary};
use dom::bindings::utils::{Reflectable, Reflector};
use dom::document::Document;
use dom::element::HTMLAreaElementTypeId;
use dom::eventtarget::{EventTarget, NodeTargetTypeId};
use dom::htmlelement::HTMLElement;
use dom::node::{Node, NodeHelpers, ElementNodeTypeId};

use servo_util::str::DOMString;

#[jstraceable]
#[must_root]
#[privatize]
pub struct HTMLAreaElement {
    htmlelement: HTMLElement
}

impl HTMLAreaElementDerived for EventTarget {
    fn is_htmlareaelement(&self) -> bool {
        *self.type_id() == NodeTargetTypeId(ElementNodeTypeId(HTMLAreaElementTypeId))
    }
}

impl HTMLAreaElement {
    fn new_inherited(localName: DOMString, prefix: Option<DOMString>, document: JSRef<Document>) -> HTMLAreaElement {
        HTMLAreaElement {
            htmlelement: HTMLElement::new_inherited(HTMLAreaElementTypeId, localName, prefix, document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString, prefix: Option<DOMString>, document: JSRef<Document>) -> Temporary<HTMLAreaElement> {
        let element = HTMLAreaElement::new_inherited(localName, prefix, document);
        Node::reflect_node(box element, document, HTMLAreaElementBinding::Wrap)
    }
}

impl Reflectable for HTMLAreaElement {
    fn reflector<'a>(&'a self) -> &'a Reflector {
        self.htmlelement.reflector()
    }
}
