use crate::prelude::*;

use biome_js_syntax::{JsxNamespaceName, JsxNamespaceNameFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxNamespaceName;

impl FormatNodeRule<JsxNamespaceName> for FormatJsxNamespaceName {
    fn fmt_fields(&self, node: &JsxNamespaceName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxNamespaceNameFields {
            namespace,
            colon_token,
            name,
        } = node.as_fields();

        write![f, [namespace.format(), colon_token.format(), name.format()]]
    }
}
