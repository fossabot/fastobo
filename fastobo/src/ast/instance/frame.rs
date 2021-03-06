use std::ops::Deref;
use std::ops::DerefMut;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// An instance frame, describing a particular individual.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InstanceFrame {
    id: Line<InstanceIdent>,
    clauses: Vec<Line<InstanceClause>>,
}

impl InstanceFrame {
    /// Create a new instance frame with the given ID but without any clause.
    pub fn new<I>(id: I) -> Self
    where
        I: Into<Line<InstanceIdent>>
    {
        Self::with_clauses(id, Vec::new())
    }

    /// Create a new instance frame with the provided ID and clauses.
    pub fn with_clauses<I>(id: I, clauses: Vec<Line<InstanceClause>>) -> Self
    where
        I: Into<Line<InstanceIdent>>
    {
        Self {
            id: id.into(),
            clauses
        }
    }

    /// Get the identifier of the `InstanceFrame`.
    pub fn id(&self) -> &Line<InstanceIdent> {
        &self.id
    }

    /// Get the `InstanceClause`s of the `InstanceFrame`.
    pub fn clauses(&self) -> &Vec<Line<InstanceClause>> {
        &self.clauses
    }
}

impl AsRef<Vec<Line<InstanceClause>>> for InstanceFrame {
    fn as_ref(&self) -> &Vec<Line<InstanceClause>> {
        &self.clauses
    }
}

impl AsRef<[Line<InstanceClause>]> for InstanceFrame {
    fn as_ref(&self) -> &[Line<InstanceClause>] {
        &self.clauses
    }
}

impl Deref for InstanceFrame {
    type Target = Vec<Line<InstanceClause>>;
    fn deref(&self) -> &Self::Target {
        &self.clauses
    }
}

impl DerefMut for InstanceFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.clauses
    }
}

impl Display for InstanceFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("[Instance]\nid: ").and(self.id.fmt(f))?;
        self.clauses.iter().try_for_each(|clause| clause.fmt(f))
    }
}

impl<'i> FromPair<'i> for InstanceFrame {
    const RULE: Rule = Rule::InstanceFrame;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let iid = InstanceIdent::from_pair_unchecked(inner.next().unwrap())?;
        let id = Eol::from_pair_unchecked(inner.next().unwrap())?.and_inner(iid);

        let mut clauses = Vec::new();
        for pair in inner {
            clauses.push(Line::<InstanceClause>::from_pair_unchecked(pair)?);
        }

        Ok(InstanceFrame { id, clauses })
    }
}
impl_fromstr!(InstanceFrame);

impl IntoIterator for InstanceFrame {
    type Item = Line<InstanceClause>;
    type IntoIter = <Vec<Line<InstanceClause>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.clauses.into_iter()
    }
}

impl<'a> IntoIterator for &'a InstanceFrame {
    type Item = &'a Line<InstanceClause>;
    type IntoIter = <&'a [Line<InstanceClause>] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.clauses.as_slice().iter()
    }
}
