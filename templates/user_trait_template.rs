// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use id_tree::Tree;
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType, UserActionsTrait};
use anyhow::Result;
use crate::{{user_trait_module_name}}::{{user_type_name}};

///
/// The `{{{user_type_name}}}Trait` trait is automatically generated for the
/// given grammar.
/// All functions have default implementations.
/// 
pub trait {{{user_type_name}}}Trait {
{{{trait_functions}}}
}

impl UserActionsTrait for {{{user_type_name}}} {
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeStackEntry],
        parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        match prod_num {
{{{trait_caller}}}            _ => panic!("Unhandled production number: {}", prod_num),
        }
    }
}
