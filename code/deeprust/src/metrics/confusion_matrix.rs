// Task : Evaluation metrics for deeplearning algorithms results
// Date :- 4th Feb 2017
// Authors :- @dvigneshwer
// Version :- 0.0.1

// Modified by : @nifey
// Date : 6th Feb 2017

use std::{i32, f32};

//defining a struct to represent a confusion matrix for a binary classifier
pub struct Confusionmatrix {
    pub true_positive: i32,
    pub true_negative: i32,
    pub false_positive: i32,
    pub false_negative: i32,
}

impl Confusionmatrix {
    //to find total number of predictions
    pub fn total(&self) -> i32 {
        self.true_positive + self.true_negative + self.false_positive + self.false_negative
    }
    //to find the accuracy of the model
    pub fn accuracy(&self) -> f32 {
        percentage((self.true_positive as f32 + self.true_negative as f32) / (self.total() as f32))
    }
    //to find the precision of the model
    pub fn precision(&self) -> f32 {
        percentage((self.true_positive as f32) /
                   (self.true_positive as f32 + self.false_positive as f32))
    }
    //to find the true positive rate of the model
    pub fn true_poitive_rate(&self) -> f32 {
        percentage((self.true_positive as f32) /
                   (self.true_positive as f32 + self.false_negative as f32))
    }
    //to find the false positive rate of the model
    pub fn false_positive_rate(&self) -> f32 {
        percentage((self.false_positive as f32) /
                   (self.false_positive as f32 + self.true_negative as f32))
    }
    //to find the misclassification rate of the model
    pub fn misclassification_rate(&self) -> f32 {
        percentage((self.false_positive as f32 + self.false_negative as f32) /
                   (self.total() as f32))
    }
    //to find the specificity of the model
    pub fn specificity(&self) -> f32 {
        percentage((self.true_negative as f32) /
                   (self.false_positive as f32 + self.true_negative as f32))
    }
    //to find the prevalance of the model
    pub fn prevalance(&self) -> f32 {
        percentage((self.true_positive as f32 + self.false_negative as f32) / (self.total() as f32))
    }
}

// Converting to percentage
fn percentage(value: f32) -> f32 {
    value as f32 * 100.0
}
