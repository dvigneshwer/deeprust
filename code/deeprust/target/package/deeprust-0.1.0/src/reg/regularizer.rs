// Task :- l2 regularization for sgd
// Author :- @abhijayghildyal 
// Version :- 0.0.1
// Date : - 5th Feb 2017

//!
//! Regularizer module contains the following modules:
//! * L2 regularization 

/// L2 Regularizer - Takes in lambda and theta as arguments for calculating the l2_reg value
///
/// # Example: 
/// ```ignore
/// ~~~~
///extern crate deeprust;
///use deeprust::reg::regularizer as l2;
///fn main() {
///    let lambda = 4.0_f64;
///    let m = 5.0; // Number of training samples y.len()
///    let theta : Vec<f64> = vec![1.0,2.0,3.0,4.0];
///    let mut loss = 3.0;

///    println!("\nlambda = {0} \nm = {1} \ntheta = {2:?} \nloss = {3}", lambda, m, theta, loss);
///    println!("theta^2 = {0} ", l2::l2_reg(lambda, theta.clone())/lambda);
///    println!("lambda*theta_square = {0} ", l2::l2_reg(lambda, theta.clone()));
///    println!("(lambda*theta_square)/(2.0*m) = {0} ", l2::l2_reg(lambda, theta.clone())/(2.0*m));

///    loss = loss + l2::l2_reg(lambda, theta)/(2.0*m);
///    println!("loss = {} \n", loss);  
///}
/// ~~~~
pub fn l2_reg(lambda: f64, theta: Vec<f64>) -> f64 {
    let theta_square: f64 = theta[1..theta.len()].iter().zip(theta[1..theta.len()].iter()).map(|(x, y)| x*y).sum();
	return lambda*theta_square;
}
