//! Identifiers used in OBO documents.
//!
//! `Ident` refers to an *owned* identifier, while `Id` refers to its *borrowed*
//! counterpart.

mod local;
mod prefix;
mod prefixed;
mod subclasses;
mod unprefixed;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;
use url::Url;

pub use self::local::IdentLocal;
pub use self::local::IdLocal;
pub use self::prefix::IdentPrefix;
pub use self::prefix::IdPrefix;
pub use self::prefixed::PrefixedId;
pub use self::prefixed::PrefixedIdent;
pub use self::subclasses::ClassIdent;
pub use self::subclasses::InstanceIdent;
pub use self::subclasses::NamespaceIdent;
pub use self::subclasses::RelationIdent;
pub use self::subclasses::SubsetIdent;
pub use self::subclasses::SynonymTypeIdent;
pub use self::unprefixed::UnprefixedIdent;
pub use self::unprefixed::UnprefixedId;

use crate::borrow::Cow;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

use self::Ident::*;

/// An identifier, either prefixed, unprefixed, or a valid URL.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Ident {
    Prefixed(PrefixedIdent),
    Unprefixed(UnprefixedIdent),
    Url(Url),
}

impl From<PrefixedIdent> for Ident {
    fn from(id: PrefixedIdent) -> Self {
        Prefixed(id)
    }
}

impl From<UnprefixedIdent> for Ident {
    fn from(id: UnprefixedIdent) -> Self {
        Unprefixed(id)
    }
}

impl From<Url> for Ident {
    fn from(url: Url) -> Self {
        Ident::Url(url)
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Prefixed(id) => id.fmt(f),
            Unprefixed(id) => id.fmt(f),
            Url(url) => url.fmt(f),
        }
    }
}

impl<'i> FromPair<'i> for Ident {
    const RULE: Rule = Rule::Id;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::PrefixedId => PrefixedIdent::from_pair_unchecked(inner).map(From::from),
            Rule::UnprefixedId => UnprefixedIdent::from_pair_unchecked(inner).map(From::from),
            // FIXME(@althonos): need proper error report if the parser fails.
            Rule::UrlId => Ok(Ident::Url(Url::parse(inner.as_str()).unwrap())),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(Ident);

/// A borrowed `Identifier`.
pub enum Id<'a> {
    Prefixed(Cow<'a, PrefixedId<'a>>),
    Unprefixed(Cow<'a, &'a UnprefixedId>),
    Url(Cow<'a, &'a Url>),
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn from_str() {
        let actual = Ident::from_str("http://purl.obolibrary.org/obo/po.owl").unwrap();
        let expected = Ident::Url(Url::parse("http://purl.obolibrary.org/obo/po.owl").unwrap());
        assert_eq!(actual, expected);

        let actual = Ident::from_str("GO:0046154").unwrap();
        let expected = Ident::Prefixed(PrefixedIdent::new(
            IdentPrefix::new(String::from("GO")),
            IdentLocal::new(String::from("0046154")),
        ));
        assert_eq!(actual, expected);

        let actual = Ident::from_str("goslim_plant").unwrap();
        let expected = Ident::Unprefixed(UnprefixedIdent::new(String::from("goslim_plant")));
        assert_eq!(actual, expected);
    }
}
