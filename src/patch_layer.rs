use std::collections::BTreeSet;

use crate::{Patch, Contains, PatchSearchResult};

#[derive(Clone)]
pub struct PatchLayer {
    patches: BTreeSet<Patch>
}

impl PatchLayer {
    pub fn new_with(patch: Patch) -> Self {
        let mut patches = BTreeSet::new();
        patches.insert(patch);
        Self {
            patches
        }
    }

    pub fn may_contain(&self, patch: &Patch) -> bool {
        ! self.patches.iter().any(|ep| ep.overlaps(patch))
    }

    pub fn insert(&mut self, patch: Patch) -> bool {
        self.patches.insert(patch)
    }

    /// finds the patch which contains data for the given offset
    pub fn patch_for(&self, offset: u64) -> Option<&Patch> {
        self.patches.iter().find(|patch| patch.contains(offset))
    }

    pub fn next_patch_for(&self, offset: u64) -> PatchSearchResult {
        let mut current_patch = None;
        for next_patch in self.patches.iter() {
            if let Some(current_patch) = current_patch {
                return PatchSearchResult::CurrentPatchIsFollowedBy{current_patch, next_patch};
            }
            if next_patch.contains(offset) {
                assert!(current_patch.is_none());
                current_patch = Some(next_patch);
            } else if next_patch.begin() > offset {
                return PatchSearchResult::PatchFollows{next_patch}
            }
        }

        if let Some(current_patch) = current_patch {
            PatchSearchResult::CurrentPatchIsTheLastOne{current_patch}
        } else {
            PatchSearchResult::NoMorePatches
        }
    }

    pub fn iter_patches(&self) -> impl DoubleEndedIterator<Item=&Patch> {
        self.patches.iter()
    }
}