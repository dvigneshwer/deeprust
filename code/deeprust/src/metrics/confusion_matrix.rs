// Task : Evaluation metrics for deeplearning algorithms results
// Date :- 4th Feb 2017
// Authors :- @dvigneshwer
// Version :- 0.0.1

// Modified by : @nifey
// Date : 6th Feb 2017


//! ## Confusion Matrix
//! Confusion matrix is a matrix used to measure the performance of a classifier.
//! Explanation of the confusion matrix terminology can be found in [DataSchool's blog post](http://www.dataschool.io/simple-guide-to-confusion-matrix-terminology/).

// importing standard modules 
use std::{i32, f32};

/// Defining a struct to represent a confusion matrix for a classifier
/// ### Variable - details
/// Below are the variables used in the user defined rust structure Confusionmatrix.
/// * true_positive - i32 - This denotes the number of true positives.
/// * true_negative - i32 - This denotes the number of true negatives.
/// * false_positive - i32 - This denotes the number of false  positives.
/// * false_negative - i32 - This denotes the number of false negatives.
pub struct Confusionmatrix {
    pub true_positive: i32,
    pub true_negative: i32,
    pub false_positive: i32,
    pub false_negative: i32,
}

/// ## Functionalities of the confusion matrix 
/// ### Methods - details
/// Below are the functions that are defined as methods for the Confusionmatrix structure.
/// * total(&self) -> i32 : returns the total number of cases as an i32.
/// * accuracy(&self) -> f32 : returns accuracy of the classifier as an f32.
/// * precision(&self) -> f32 : returns precision of the classifier as an f32.
/// * true_poitive_rate(&self) -> f32 : returns true positive rate of the classifier as an f32.
/// * false_positive_rate(&self) -> f32 : returns false positive rate of the classifier as an f32.
/// * misclassification_rate(&self) -> f32 : returns misclassification rate of the classifier as an f32.
/// * specificity(&self) -> f32 : returns specificity of the classifier as an f32.
/// * prevalance(&self) -> f32 : returns prevalance of the classifier as an f32.
impl Confusionmatrix {
    /// To find total number of predictions
    pub fn total(&self) -> i32 {
        self.true_positive + self.true_negative + self.false_positive + self.false_negative
    }
    /// To find the accuracy of the model
    pub fn accuracy(&self) -> f32 {
        percentage((self.true_positive as f32 + self.true_negative as f32) / (self.total() as f32))
    }
    /// To find the precision of the model
    pub fn precision(&self) -> f32 {
        percentage((self.true_positive as f32) /
                   (self.true_positive as f32 + self.false_positive as f32))
    }
    /// To find the true positive rate of the model
    pub fn true_poitive_rate(&self) -> f32 {
        percentage((self.true_positive as f32) /
                   (self.true_positive as f32 + self.false_negative as f32))
    }
    /// To find the false positive rate of the model
    pub fn false_positive_rate(&self) -> f32 {
        percentage((self.false_positive as f32) /
                   (self.false_positive as f32 + self.true_negative as f32))
    }
    /// To find the misclassification rate of the model
    pub fn misclassification_rate(&self) -> f32 {
        percentage((self.false_positive as f32 + self.false_negative as f32) /
                   (self.total() as f32))
    }
    /// To find the specificity of the model
    pub fn specificity(&self) -> f32 {
        percentage((self.true_negative as f32) /
                   (self.false_positive as f32 + self.true_negative as f32))
    }
    /// To find the prevalance of the model
    pub fn prevalance(&self) -> f32 {
        percentage((self.true_positive as f32 + self.false_negative as f32) / (self.total() as f32))
    }
}

// ## Percentage function
// Converting to percentage function
// Takes an float value and converts it into float32
fn percentage(value: f32) -> f32 {
    value as f32 * 100.0
}
