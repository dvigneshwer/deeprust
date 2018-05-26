# Evaluation Metrics Explanations
## Confusion Matrix

Confusion matrix is a matrix used to measure the performance of a classifier.

Explanation of the confusion matrix terminology can be found in [DataSchool's blog post](http://www.dataschool.io/simple-guide-to-confusion-matrix-terminology/).


### Variable - details
Below are the variables used in the user defined rust structure Confusionmatrix.
* true_positive - i32 - This denotes the number of true positives.
* true_negative - i32 - This denotes the number of true negatives.
* false_positive - i32 - This denotes the number of false  positives.
* false_negative - i32 - This denotes the number of false negatives.

### Methods - details
Below are the functions that are defined as methods for the Confusionmatrix structure.
* total(&self) -> i32 : returns the total number of cases as an i32.
* accuracy(&self) -> f32 : returns accuracy of the classifier as an f32.
* precision(&self) -> f32 : returns precision of the classifier as an f32.
* true_poitive_rate(&self) -> f32 : returns true positive rate of the classifier as an f32.
* false_positive_rate(&self) -> f32 : returns false positive rate of the classifier as an f32.
* misclassification_rate(&self) -> f32 : returns misclassification rate of the classifier as an f32.
* specificity(&self) -> f32 : returns specificity of the classifier as an f32.
* prevalance(&self) -> f32 : returns prevalance of the classifier as an f32.

### Code Workflow

* Defined an user defined structure **Confusionmatrix** to represent a confusion matrix for a binary classifier.
* Defined methods that calculate various rates for the confusion matrix.
