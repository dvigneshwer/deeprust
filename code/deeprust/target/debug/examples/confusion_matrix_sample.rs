// Task : Example file for confusion matrix module
// Author : Vigneshwer 
// Date : 18th FEB 2016
// Version : 0.0.1

#[macro_use(Confusionmatrix)]
extern crate deeprust;

use deeprust::metrics::confusion_matrix;

fn main() {

    // assigning random values to the confusion matrix
    let sample = Confusionmatrix!(100,50,10,5);

    println!("The total predictions {}", sample.total());
    // Calculating the accuracy of the model
    println!("Accuracy of the model {:.2}", sample.accuracy());
    // Calculating the precision of the model
    println!("Precision of the model {:.2}", sample.precision());
    // Calculating the true positive rate of the model
    println!("True positive rate of the model {:.2}",
             sample.true_poitive_rate());
    // Calculating the false positive rate of the model
    println!("False positive rate of the model {:.2}",
             sample.false_positive_rate());
    // Calculating the misclassification rate of the model
    println!("Misclassification rate of the model {:.2}",
             sample.misclassification_rate());
    // Calculating the specificity of the model
    println!("Specificity of the model {:.2}", sample.specificity());
    // Calculating the prevalance of the model
    println!("Prevalance of the model {:.2}", sample.prevalance());
 
}
