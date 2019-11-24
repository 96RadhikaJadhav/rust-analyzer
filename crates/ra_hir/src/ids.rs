//! hir makes heavy use of ids: integer (u32) handlers to various things. You
//! can think of id as a pointer (but without a lifetime) or a file descriptor
//! (but for hir objects).
//!
//! This module defines a bunch of ids we are using. The most important ones are
//! probably `HirFileId` and `DefId`.

use ra_db::{impl_intern_key, salsa};

/// This exists just for Chalk, because Chalk just has a single `StructId` where
/// we have different kinds of ADTs, primitive types and special type
/// constructors like tuples and function pointers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeCtorId(salsa::InternId);
impl_intern_key!(TypeCtorId);

/// This exists just for Chalk, because our ImplIds are only unique per module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalImplId(salsa::InternId);
impl_intern_key!(GlobalImplId);

/// This exists just for Chalk, because it needs a unique ID for each associated
/// type value in an impl (even synthetic ones).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssocTyValueId(salsa::InternId);
impl_intern_key!(AssocTyValueId);
