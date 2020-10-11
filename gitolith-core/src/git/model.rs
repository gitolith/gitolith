use crate::error::{
    AppError,
    Result,
};

/// Git is the main trait that dictates modules to implement
/// the features we want in common in concrete types.
// Find a way to optimize its memory management, since we are going to
// fetch all of the commits to process, having large quantity of commits
// may cause high memory usage. Possible solutions: Caching somewhere.
pub trait Git {
    // Trait Type.
    type Item;
    
    /// Summary of the repository. Will show who contributed how much in
    /// the following repository.
    ///
    /// As an example; the output should be something like this:
    ///
    /// 100 kondanta 60.0%
    ///  40 orhun    40.0%
    fn summary() -> Result<Self::Item>;
    
    /// List of the authors that has been commiting the repo.
    fn authors() -> Result<Self::Item>;

    /// List of the branches that are merged into the DEFAULT branch.
    fn merged_branches() -> Result<Self::Item>;

    /// List of the things that author did in the given time. Default
    /// value is 7.
    // NOTE: This will take a struct as a parameter not string and u8.
    // For the convenience, I put those types. @kondanta
    fn stand_up(author: String, days: u8) -> Result<Self::Item>;
}
