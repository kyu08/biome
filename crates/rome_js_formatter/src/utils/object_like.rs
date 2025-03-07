use crate::prelude::*;
use crate::JsFormatContext;
use biome_js_syntax::{JsObjectExpression, JsSyntaxToken, TsObjectType};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, AstSeparatedList, SyntaxResult};
use rome_formatter::write;
use rome_formatter::{Format, FormatResult};

declare_node_union! {
    pub (crate) JsObjectLike = JsObjectExpression | TsObjectType
}
impl JsObjectLike {
    fn l_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsObjectLike::JsObjectExpression(oe) => oe.l_curly_token(),
            JsObjectLike::TsObjectType(ot) => ot.l_curly_token(),
        }
    }
    fn r_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsObjectLike::JsObjectExpression(oe) => oe.r_curly_token(),
            JsObjectLike::TsObjectType(ot) => ot.r_curly_token(),
        }
    }

    fn members_have_leading_newline(&self) -> bool {
        match self {
            JsObjectLike::JsObjectExpression(oe) => oe.members().syntax().has_leading_newline(),
            JsObjectLike::TsObjectType(ot) => ot.members().syntax().has_leading_newline(),
        }
    }

    fn members_are_empty(&self) -> bool {
        match self {
            JsObjectLike::JsObjectExpression(oe) => oe.members().is_empty(),
            JsObjectLike::TsObjectType(ot) => ot.members().is_empty(),
        }
    }

    fn write_members(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            JsObjectLike::JsObjectExpression(oe) => {
                write!(f, [oe.members().format()])
            }
            JsObjectLike::TsObjectType(ot) => {
                write!(f, [ot.members().format()])
            }
        }
    }
}

impl Format<JsFormatContext> for JsObjectLike {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let members = format_with(|f| self.write_members(f));

        write!(f, [self.l_curly_token().format(),])?;

        if self.members_are_empty() {
            write!(
                f,
                [format_dangling_comments(self.syntax()).with_block_indent(),]
            )?;
        } else {
            let should_expand = self.members_have_leading_newline();
            write!(
                f,
                [group(&soft_space_or_block_indent(&members)).should_expand(should_expand)]
            )?;
        }

        write!(f, [self.r_curly_token().format()])
    }
}
