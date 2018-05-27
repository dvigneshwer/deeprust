// Task : Main file for executing the different deeprust units
// Author : Vigneshwer 
// Date : 18th FEB 2016
// Version : 1.0

//! # DeepRust features:
//! The library contains the following functions:
//! * The reg module contains functions to perform regularization operations
//! * The metrics module contains functions to generate the confusion matrix
//!   * we use the confusion matrix macro to feed value into metric module functions


/// Confusionmatrix takes in four fields which are expresions which is converted into the confusion_matrix struct of the metrics module
///
/// # Example: 
/// ```ignore
/// ~~~~
/// #[macro_use(Confusionmatrix)]
/// extern crate deeprust;
/// fn main() {
/// 	let sample = Confusionmatrix!(100,50,10,5);
/// }
/// ~~~~
#[macro_export]
macro_rules! Confusionmatrix {
    ($true_positive:expr, $true_negative:expr, $false_positive:expr, $false_negative:expr) => {
        confusion_matrix::Confusionmatrix{
            true_positive: $true_positive,
            true_negative: $true_negative,
            false_positive: $false_positive,
            false_negative: $false_negative,
        }
    }
}

pub mod metrics;
pub mod reg;

