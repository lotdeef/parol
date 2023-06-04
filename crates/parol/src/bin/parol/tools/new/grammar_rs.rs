use derive_builder::Builder;
use parol::generators::NamingHelper;

#[derive(Builder, Debug, Default)]
pub(crate) struct GrammarRsData<'a> {
    crate_name: &'a str,
    grammar_name: String,
}

impl std::fmt::Display for GrammarRsData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let GrammarRsData {
            crate_name,
            grammar_name,
        } = self;

        let grammar_trait_mod = format!("{crate_name}_grammar_trait");
        let grammar_trait = format!("{grammar_name}GrammarTrait");
        let grammar = format!("{grammar_name}Grammar");
        let grammar_start_symbol = NamingHelper::to_lower_snake_case(grammar_name);
        f.write_fmt(ume::ume! {
            use crate::#grammar_trait_mod::{#grammar_name, #grammar_trait};
            #[allow(unused_imports)]
            use parol_runtime::Result;
            use std::fmt::{Debug, Display, Error, Formatter};
        })?;

        write!(f, "\n\n")?;

        write!(
            f,
            "
			///
			/// Data structure that implements the semantic actions for our {grammar_name} grammar
			/// !Change this type as needed!
			///
		"
        )?;
        f.write_fmt(ume::ume! {
            #[derive(Debug, Default)]
            pub struct #grammar<'t> {
                pub #grammar_start_symbol: Option<#grammar_name<'t>>,
            }
        })?;

        write!(f, "\n\n")?;

        f.write_fmt(ume::ume! {
            impl #grammar<'_> {
                pub fn new() -> Self {
                    #grammar::default()
                }
            }
        })?;

        write!(f, "\n\n")?;

        f.write_fmt(ume::ume! {
            impl Display for #grammar_name<'_> {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
                    write!(f, "{:?}", self)
                }
            }
        })?;

        write!(f, "\n\n")?;

        f.write_fmt(ume::ume! {
            impl Display for #grammar<'_> {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
                    match &self.#grammar_start_symbol {
                        Some(#grammar_start_symbol) => writeln!(f, "{}", #grammar_start_symbol),
                        None => write!(f, "No parse result"),
                    }
                }
            }
        })?;

        write!(f, "\n\n")?;

        let comment = format!(
            "
			// !Adjust your implementation as needed!

			/// Semantic action for non-terminal '{grammar_name}'
		"
        );
        f.write_fmt(ume::ume! {
            impl<'t> #grammar_trait<'t> for #grammar<'t> {
                #comment
                fn #grammar_start_symbol(&mut self, arg: &#grammar_name<'t>) -> Result<()> {
                    self.#grammar_start_symbol = Some(arg.clone());
                    Ok(())
                }
            }
        })
    }
}
