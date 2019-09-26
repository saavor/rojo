//! Defines the data structures used for describing instance patches.

use std::collections::HashMap;

use rbx_dom_weak::{RbxId, RbxValue};

use super::{InstanceMetadata, InstanceSnapshot};

/// A set of different kinds of patches that can be applied to an RbxTree.
///
/// These patches shouldn't be persisted: there's no mechanism in place to make
/// sure that another patch wasn't applied before this one that could cause a
/// conflict!
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PatchSet<'a> {
    pub removed_instances: Vec<RbxId>,
    pub added_instances: Vec<PatchAdd<'a>>,
    pub updated_instances: Vec<PatchUpdate>,
}

impl<'a> PatchSet<'a> {
    pub fn new() -> Self {
        PatchSet {
            removed_instances: Vec::new(),
            added_instances: Vec::new(),
            updated_instances: Vec::new(),
        }
    }
}

/// A patch containing an instance that was added to the tree.
#[derive(Debug, Clone, PartialEq)]
pub struct PatchAdd<'a> {
    pub parent_id: RbxId,
    pub instance: InstanceSnapshot<'a>,
}

/// A patch indicating that properties of an instance changed.
#[derive(Debug, Clone, PartialEq)]
pub struct PatchUpdate {
    pub id: RbxId,
    pub changed_name: Option<String>,
    pub changed_class_name: Option<String>,

    /// Contains all changed properties. If a property is assigned to `None`,
    /// then that property has been removed.
    pub changed_properties: HashMap<String, Option<RbxValue>>,

    /// Changed Rojo-specific metadata, if any of it changed.
    pub changed_metadata: Option<InstanceMetadata>,
}

/// Applied patch sets have the same rough shape as PatchSet, but are
/// descriptive of the operation that happened instead of prescribing what
/// mutations to apply to the tree.
///
/// Applied patch sets are generated by applying a patch to a tree, and are
/// suitable for sending over the network to a synchronized tree like the Rojo
/// Studio plugin.
///
// TODO: Introduce machinery to detect conflicts, like keeping previous +
// current values in all fields.
#[derive(Debug, Clone, Default)]
pub struct AppliedPatchSet {
    pub removed: Vec<RbxId>,
    pub added: Vec<AppliedPatchAdd>,
    pub updated: Vec<AppliedPatchUpdate>,
}

impl AppliedPatchSet {
    pub fn new() -> Self {
        AppliedPatchSet {
            removed: Vec::new(),
            added: Vec::new(),
            updated: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppliedPatchAdd {
    pub instance_id: RbxId,
}

#[derive(Debug, Clone)]
pub struct AppliedPatchUpdate {
    pub id: RbxId,

    // TODO: Store previous values in order to detect application conflicts
    pub changed_name: Option<String>,
    pub changed_class_name: Option<String>,
    pub changed_properties: HashMap<String, Option<RbxValue>>,
    pub changed_metadata: Option<InstanceMetadata>,
}
