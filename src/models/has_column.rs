/// Defines what column exists for a given model.
pub trait HasColumn {
    /// Returns true if the given column exists for this model.
    fn has_column(col: &str) -> bool;
}
