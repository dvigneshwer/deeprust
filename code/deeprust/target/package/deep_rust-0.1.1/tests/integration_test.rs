// Task : Test cases for different functions of the 
// Author : Vigneshwer 
// Date : 28th May 2018
// Version : 0.0.1

#[macro_use(Confusionmatrix)]
extern crate deeprust;

use deeprust::metrics::confusion_matrix;

#[cfg(test)]
mod tests {
	use super::*;

	// Testing the confusion matrix functionalities 
    #[test]
    fn check_confusion_matrix() {
    	let sample = Confusionmatrix!(100,50,10,5);
        assert_eq!(165, sample.total());
        assert_eq!(90.91, sample.accuracy());
        assert_eq!(90.91, sample.precision());
        assert_eq!(95.24, sample.true_positive_rate());
        assert_eq!(16.67, sample.false_positive_rate());
        assert_eq!(9.09, sample.misclassification_rate());
        assert_eq!(83.33, sample.specificity());
        assert_eq!(63.34, sample.prevalance());
    }
}