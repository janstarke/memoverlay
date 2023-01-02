use thiserror::Error;

#[derive(Error, Debug)]
pub enum OverlayError {
    #[error("this patch contains no data, which makes no sense")]
    EmptyPatch
}