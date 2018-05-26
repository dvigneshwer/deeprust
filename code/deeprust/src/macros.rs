// Task : Create macros for ML units
// Author : Chinmay Kousik (ckousik)
// Date : 1-JUN-2017 

// Macro for confusion matrix
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
