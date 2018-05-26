// Task : Main file for testing the different deeprust units
// Author : Vigneshwer 
// Date : 18th FEB 2016
// Version : 1.0
#[macro_use(Confusionmatrix)]
extern crate deeprust;

use deeprust::metrics::confusion_matrix;
use deeprust::reg::l2_reg as l2;

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


    // test variables
    let lambda = 4.0_f64;
    let m = 5.0; // Number of training samples y.len()
    let theta : Vec<f64> = vec![1.0,2.0,3.0,4.0];
    let mut loss = 3.0;
    // printing input
    println!("\nlambda = {0} \nm = {1} \ntheta = {2:?} \nloss = {3}", lambda, m, theta, loss);
    println!("theta^2 = {0} ", l2::regularization_term(lambda, theta.clone())/lambda);
    println!("lambda*theta_square = {0} ", l2::regularization_term(lambda, theta.clone()));
    println!("(lambda*theta_square)/(2.0*m) = {0} ", l2::regularization_term(lambda, theta.clone())/(2.0*m));
    // function call
    loss = loss + l2::regularization_term(lambda, theta)/(2.0*m);
    println!("loss = {} \n", loss);  
}
