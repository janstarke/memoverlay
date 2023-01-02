use crate::Patch;

pub enum PatchSearchResult<'a> {
    NoMorePatches,
    PatchFollows{next_patch: &'a Patch},
    CurrentPatchIsTheLastOne{current_patch: &'a Patch},
    CurrentPatchIsFollowedBy{current_patch: &'a Patch, next_patch: &'a Patch}
}