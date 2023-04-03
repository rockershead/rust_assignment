// Define a User struct which contains 2 fields: 
    // name (string e.g "John")
    // balance (tuple e.g (100.00, "SGD"))

    struct User {
        name: String,
        balance: (f32,String)
        
     }

// Define a User method (using impl) 
    // called print_user_detail, 
        // which simply prints the name, balance and currency of the user.

        impl User {
            fn print_user_detail(&self) {
                  println!("{} has a balance of {} {}", self.name,self.balance.1,self.balance.0);
              }

              fn print_name(&self){
                println!("{}",self.name);
              }
              // TODO: Create 1 method for User struct called print_name, that prints the name
          }

// Define an accrue_interest function, which takes in a user and interest percentage as 2 separate parameters. 
    // Within the function, increase the users' balance by the interest percentage, and print out the user details by calling the print_user_detail method. 

    fn accrue_interest( user: &mut User, interest: f32) {
        user.balance.0 = user.balance.0 + (user.balance.0 * interest / 100.0);
        user.print_user_detail();
    }

fn main() {
    // In the main function, create a user variable of type User, populating the field values of name, and balance and currency.
    let mut user=User{
        name:"Peter".to_owned(),
        balance:(34.0,"SGD".to_owned())       
  };
  
    // Then, call the accrue_interest function.

    accrue_interest(&mut user,2.0);
    accrue_interest(&mut user,2.0);
    //accrue_interest(&mut user,2.0);
    
    // Bonus: After the call to accrue_interest, call it multiple times so that the user may benefit from compounding interest.
}
