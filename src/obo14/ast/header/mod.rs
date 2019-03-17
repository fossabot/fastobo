use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;

use crate::error::Error;
use crate::error::Result;
use super::super::parser::FromPair;
use super::super::parser::Parser;
use super::super::parser::Rule;
use super::ClassId;
use super::Id;
use super::IdPrefix;
use super::Iri;
use super::Line;
use super::NaiveDate;
use super::NamespaceId;
use super::PropertyValue;
use super::QuotedString;
use super::RelationId;
use super::SubsetId;
use super::SynonymScope;
use super::SynonymTypeId;
use super::UnquotedString;



/// The header frame, containing metadata about an OBO document.
pub struct HeaderFrame {
    clauses: Vec<HeaderClause>,
}

/// An clause appearing in a header frame.
pub enum HeaderClause {
    FormatVersion(UnquotedString),
    DataVersion(UnquotedString),
    DateTag(NaiveDate),
    SavedBy(UnquotedString),
    AutoGeneratedBy(UnquotedString),
    ImportTag(Import),
    Subsetdef(SubsetId, QuotedString),
    SynonymTypeDef(SynonymTypeId, QuotedString, Option<SynonymScope>),
    DefaultNamespaceTag(NamespaceId),
    Idspace(IdPrefix, Iri, Option<QuotedString>),
    TreatXrefsAsEquivalent(IdPrefix),
    TreatXrefsAsGenusDifferentia(IdPrefix, RelationId, ClassId),
    TreatXrefsAsReverseGenusDifferentia(IdPrefix, RelationId, ClassId),
    TreatXrefsAsRelationship(IdPrefix, RelationId),
    TreatXrefsAsIsA(IdPrefix),
    TreatXrefsAsHasSubclass(IdPrefix),
    PropertyValue(Line<PropertyValue>),
    Remark(UnquotedString),
    Ontology(UnquotedString),
    OwlAxioms(UnquotedString),
    Unreserved(UnquotedString, UnquotedString),
}

/// A reference to another document to be imported.
pub enum Import {
    Iri(Iri),
    Abbreviated(Id), // QUESTION(@althonos): UnprefixedID ?
}

impl From<Iri> for Import {
    fn from(iri: Iri) -> Self {
        Import::Iri(iri)
    }
}

impl From<Id> for Import {
    fn from(id: Id) -> Self {
        Import::Abbreviated(id)
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Import::*;
        match self {
            Iri(iri) => iri.fmt(f),
            Abbreviated(id) => id.fmt(f),
        }
    }
}

impl FromPair for Import {
    const RULE: Rule = Rule::Import;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::Iri => Iri::from_pair_unchecked(inner).map(From::from),
            Rule::Id => Id::from_pair_unchecked(inner).map(From::from),
            _ => unreachable!(),
        }
    }
}
