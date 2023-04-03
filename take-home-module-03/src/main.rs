// define struct of UserAccount with field: name (String), and age (Option<u32>)
struct UserAccount{

name:String,
age:Option<u32>



}

// define a trait called Balance, and within, function get_balance returning integer of 10

trait Balance {

   fn get_balance(&self) -> u32 {
      // special interest rate for FD
      10
  }
  
  }

// implement trait Balance to UserAccount struct
impl Balance for UserAccount{}

// create function increase_balance which takes as arguments
// - a type that implements Balance trait
// - an u32 amount parameter containing the increase amount
// within this function,
// - if increase amount is <= 10, return an OK containing the get_balance + amount
// - if increase amount is > 10, return an Err with error message "Increase must be less than 10!"
// Tip: this function should return a Result<u32, String>



fn increase_balance<T:Balance>(balance:&T,amount:u32)->Result<u32,String>{

   if(amount<=10){
     // println!("Output {}",balance.get_balance()+amount);
      //let result:u32=balance.get_balance()+amount;
      return Ok(balance.get_balance());  //ok is u32
   }
   else{
     // println!("Increase must be less than 10");
      return Err("Increase must be less than 10!".to_owned());   //err is string
   }

   //let f= match f {
    //  Ok(amount<=10) => println!("Output {}",balance.get_balance()+amount),
    //  Err(error) => panic!("Increase must be less than 10"), // best not to panic! best ask user to retry
 // };
  
}




fn main() {
   // create user_account, and set his age as Option::None
   let user_account=UserAccount{
      name:"Peter".to_owned(),
     // age:Some(32)
     age:None
   };

   // You want to increase the user_account's balance by 11
   let f=increase_balance(&user_account,11);  //f is a tuple
   
   
   // use a match, if the result of increase_balance is
   // - Ok: print "UserAccount balance increased to {}" where {} is the new balance value
   // - Err: print the error message returned

    match f {
      Ok(x) => println!("Output {}",x),
      Err(e) => println!("{}",e), // best not to panic! best ask user to retry
  };

   // use an if...let...else statement to print the UserAccount age if it is a Option::Some

   if let Some(age)=user_account.age{

      println!("Age is {}",age);
   }

}

