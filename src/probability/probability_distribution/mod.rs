pub mod outcome_to_counts_helpers;
pub mod probability_distribution_factory;
pub mod probability_distribution_struct;
pub mod probability_distribution_to_table;
pub mod probability_distribution_total_outcome_count;

pub use self::outcome_to_counts_helpers::add_outcome_to_map;
pub use self::probability_distribution_struct::ProbabilityDistribution;
pub use self::probability_distribution_to_table::ToTable;
