// Task :- l2 regularization for sgd
// Author :- @abhijayghildyal 
// Version :- 1.10
// Date : - 5th Feb 2017

pub fn regularization_term(lambda: f64, theta: Vec<f64>) -> f64 {

    let theta_square: f64 = theta[1..theta.len()].iter().zip(theta[1..theta.len()].iter()).map(|(x, y)| x*y).sum();

	return lambda*theta_square;
}
