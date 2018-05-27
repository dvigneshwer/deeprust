# Deeprust Codebase 

Machine learning crate in Rust

## Installing the package

~~~~
cargo install deeprust
~~~~

or alternatively you could install it by mentioning the library name in the Cargo.toml dependecies section

## Basic Example 

1. Import the library 

~~~~
extern crate deeprust;
~~~~

2. Use the API's available according to the application that you are trying to build, for example I want to get the confusion matrix of my model's performance

~~~~
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
~~~~

For more details on the different APIs of the crate head over to the [official documentation](https://docs.rs/deep_rust) There are more examples over there.

## Setting up the project locally

1. Clone the repo
~~~~
// Downloading the project
git clone https://github.com/dvigneshwer/deeprust
cd to_proj_dir
cd ./code/deeprust
~~~~

2. Run the build script 
~~~~
bash ./build_project.sh
~~~~

## Additional info:

* To get a mathematical gist of all the implementation here, check out this [link](https://gist.github.com/dvigneshwer/9301df385be7adfd2d3ce41365aa5ad7) 

* If you are new to Rust language, please feel free to checkout the [rust training dir](./rust_training)

* Stuck at any place or spotted an issue with the project or do you have a feedback, feel free to to raise an issue

* We are always looking for contributors,checkout the issue section & join the gang 