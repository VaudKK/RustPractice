//! # Chapter 14 crate
//! `chapt 14` is a utility for adding one number


/// Adds one to a number
/// 
/// # Example
/// 
/// ```
/// let arg = 5;
/// let answer = chapt_14::add_one(arg);
/// 
/// assert_eq!(6,answer);
/// ```
pub fn add_one(num: i32) -> i32{
    num + 1
}