use chapt_11::{self, Guess};

#[test]
#[should_panic]
fn integration_test(){
    Guess::new(300);
}