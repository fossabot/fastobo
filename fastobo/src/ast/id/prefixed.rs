use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use opaque_typedef::OpaqueTypedefUnsized;
use pest::iterators::Pair;

use crate::share::Share;
use crate::share::Cow;
use crate::share::Redeem;
use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;
use super::IdPrefix;
use super::IdentPrefix;
use super::IdLocal;
use super::IdentLocal;

/// An identifier with a prefix.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq)]
pub struct PrefixedIdent {
    pub prefix: IdentPrefix,
    pub local: IdentLocal,
}

impl PrefixedIdent {
    /// Create a new `PrefixedIdent` from a prefix and a local identifier.
    ///
    /// Thanks to conversion traits, the `prefix` and `local` arguments can be
    /// passed either as strings or `ast` structures:
    ///
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// let id1 = PrefixedIdent::new("MS", "1000031");
    /// let id2 = PrefixedIdent::new(IdentPrefix::new("MS"), IdentLocal::new("1000031"));
    /// assert_eq!(id1, id2);
    /// ```
    ///
    /// # Example
    ///
    pub fn new<P, L>(prefix: P, local: L) -> Self
    where
        P: Into<IdentPrefix>,
        L: Into<IdentLocal>
    {
        Self {
            prefix: prefix.into(),
            local: local.into()
        }
    }

    /// Check if the prefixed identifier is canonical or not.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// # use std::str::FromStr;
    /// let canonical_id = PrefixedIdent::from_str("GO:0046154").unwrap();
    /// assert!(canonical_id.is_canonical());
    ///
    /// let noncanonical_id = PrefixedIdent::from_str("PATO:something").unwrap();
    /// assert!(!noncanonical_id.is_canonical());
    pub fn is_canonical(&self) -> bool {
        self.prefix.is_canonical() && self.local.is_canonical()
    }

    // /// The prefix of the prefixed identifier.
    // pub fn prefix(&self) -> IdPrefix<'_> {
    //     self.prefix.share()
    // }
    //
    // /// The local part of the prefixed identifier.
    // pub fn local(&self) -> IdLocal<'_> {
    //     self.local.share()
    // }
}

impl Display for PrefixedIdent {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.share().fmt(f)
    }
}

impl<'i> FromPair<'i> for PrefixedIdent {
    const RULE: Rule = Rule::PrefixedId;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inners = pair.into_inner();
        let prefix = IdentPrefix::from_pair_unchecked(inners.next().unwrap())?;
        let local = IdentLocal::from_pair_unchecked(inners.next().unwrap())?;
        Ok(Self::new(prefix, local))
    }
}
impl_fromstr!(PrefixedIdent);

impl PartialOrd for PrefixedIdent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.prefix.partial_cmp(&other.prefix) {
            None => None,
            Some(Ordering::Equal) => self.local.partial_cmp(&other.local),
            Some(ord) => Some(ord),
        }
    }
}

impl<'a> Share<'a, PrefixedId<'a>> for PrefixedIdent {
    fn share(&'a self) -> PrefixedId<'a> {
        PrefixedId::new(
            self.prefix.share(),
            self.local.share(),
        )
    }
}

/// A borrowed `PrefixedIdentifier`
#[derive(Clone, Debug, Hash)]
pub struct PrefixedId<'a> {
    prefix: Cow<'a, IdPrefix<'a>>,
    local: Cow<'a, IdLocal<'a>>,
}

impl<'a> PrefixedId<'a> {
    /// Create a new `PrefixedId` from references.
    pub fn new(prefix: IdPrefix<'a>, local: IdLocal<'a>) -> Self {
        Self {
            prefix: Cow::Borrowed(prefix),
            local: Cow::Borrowed(local),
        }
    }
}

impl<'a> Display for PrefixedId<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.prefix
            .fmt(f)
            .and(f.write_char(':'))
            .and(self.local.fmt(f))
    }
}

impl<'i> FromPair<'i> for Cow<'i, PrefixedId<'i>> {
    const RULE: Rule = Rule::PrefixedId;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inners = pair.into_inner();
        let prefix = Cow::<IdPrefix>::from_pair_unchecked(inners.next().unwrap())?;
        let local = Cow::<IdLocal>::from_pair_unchecked(inners.next().unwrap())?;
        Ok(Cow::Borrowed(PrefixedId { prefix, local }))
    }
}
impl_fromslice!('i, Cow<'i, PrefixedId<'i>>);

impl<'a> Redeem<'a> for PrefixedId<'a> {
    type Owned = PrefixedIdent;
    fn redeem(&'a self) -> PrefixedIdent {
        PrefixedIdent::new(self.prefix.redeem(), self.local.redeem())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn from_str() {
        let actual = PrefixedIdent::from_str("GO:0046154").unwrap();
        let expected = PrefixedIdent::new(IdentPrefix::new(String::from("GO")), IdentLocal::new(String::from("0046154")));
        assert_eq!(actual, expected);

        let actual = PrefixedIdent::from_str("PSI:MS").unwrap();
        let expected = PrefixedIdent::new(IdentPrefix::new(String::from("PSI")), IdentLocal::new(String::from("MS")));
        assert_eq!(actual, expected);

        let actual = PrefixedIdent::from_str("CAS:22325-47-9").unwrap();
        let expected = PrefixedIdent::new(IdentPrefix::new(String::from("CAS")), IdentLocal::new(String::from("22325-47-9")));
        assert_eq!(actual, expected);

        let actual = PrefixedIdent::from_str("Wikipedia:https\\://en.wikipedia.org/wiki/Gas").unwrap();
        let expected = PrefixedIdent::new(
            IdentPrefix::new(String::from("Wikipedia")),
            IdentLocal::new(String::from("https://en.wikipedia.org/wiki/Gas")),
        );
        assert_eq!(actual, expected);

        assert!(PrefixedIdent::from_str("[Term]").is_err());
        assert!(PrefixedIdent::from_str("").is_err());
        assert!(PrefixedIdent::from_str("Some\nthing:spanning").is_err());
        assert!(PrefixedIdent::from_str("GO:0046154 remaining").is_err());
    }

    #[test]
    fn to_string() {
        let id = PrefixedIdent::new(IdentPrefix::new(String::from("GO")), IdentLocal::new(String::from("0046154")));
        assert_eq!(id.to_string(), "GO:0046154")
    }
}
