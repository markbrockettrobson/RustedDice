use polars::prelude::DataFrame;

/// A trait for probability distributions to be turned into a DataFrame
pub trait ToDataFrame {
    /// Turns the probability distribution into a DataFrame
    ///
    /// # Returns
    ///
    /// A DataFrame with the probability distribution
    ///
    fn to_dataframe(&self) -> DataFrame;
}
