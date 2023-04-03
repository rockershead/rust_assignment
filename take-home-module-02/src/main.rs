// Define an Enum PaymentType with variants:
    // DigitalToken
    // Cash
    #[derive(Debug,PartialEq)]
    enum PaymentType {
        DigitalToken,
        Cash
        
    }

// Define a Seller struct which contains 3 fields:
    // payment_type (PaymentType)
    // price (f32)
    // balance (f32)

    struct Seller{
       
        payment_type:PaymentType,
        price:f32,
        balance:f32

    }

// Define a Buyer struct which contains 3 fields:
    // name (String)
    // payment_type (PaymentType)
    // balance (f32)
    #[derive(Debug)]
    struct Buyer{
      
        name:String,
        payment_type:PaymentType,
        balance:f32




    }

// Define a BuyerGroup struct which contains:
    // a vector of members (a vector of Buyer struct).
   struct BuyerGroup{
    
    members: Vec<Buyer>


   }
  

// Implement methods on BuyerGroup:
    // define method add_member
        // which adds a Buyer into members vector 

        impl BuyerGroup {
            fn add_member(&mut self,buyer:Buyer) {
                  self.members.push(buyer);
                 
              }

               // define method find_buyer which accepts a PaymentType input
        // that finds returns index of Buyer with matching payment_type, otherwise return -1
             fn find_buyer(&self,payment_type:&PaymentType)->i32{
                let mut count=0;
                for i in &self.members {
                    
                    if i.payment_type == *payment_type   //need to deference paymnet_type because i.payment type is a deferenced type and payment type is a refernced type
                    {  
                        return count;
                    }
                    count=count+1;
                }

                
                return -1


             }

              // define buy method which accepts a buyer index and a reference to a seller
        // keeps transferring value of seller's price from buyer to seller, until buyer's balance is insufficient

             fn buy(&mut self,buyer_index:i32,seller:&mut Seller){
               
            let mut buyer=   &mut self.members[buyer_index as usize];

            while buyer.balance>=seller.price{                                  //no need dereferene bcoz .something is alr derefrenced

             seller.balance=seller.balance+seller.price;
             buyer.balance=buyer.balance-seller.price;

             println!("{} balance:{}.Seller balance:{}",buyer.name,buyer.balance,seller.balance);

            }

            println!("{} balance {} insufficient.Seller balance:{}",buyer.name,buyer.balance,seller.balance);


                
             }
          }
    
   

   



fn main() {
    // Create 2 buyers with names John, Sally, with payment_type of DigitalToken, Cash, and balance of 100.00 and 100.00 respectively
     
     let mut buyer1=  Buyer {
        name: "John".to_owned(),
        payment_type:PaymentType::DigitalToken,
        balance: 100.0,
    };

    let mut buyer2=  Buyer {
        name: "Sally".to_owned(),
        payment_type:PaymentType::Cash,
        balance: 100.0,
    };
    // Create an empty BuyerGroup
    
    let mut buyers: Vec<Buyer> = Vec::new();  //empty array
    
     let mut buyer_group=BuyerGroup{
        members:  buyers
     };
    // Add 2 buyers (John and Sally) into buyer_group sequentially

    buyer_group.add_member(buyer1);
    buyer_group.add_member(buyer2);

    // Call find_buyer method on the buyer group to get index of buyer with Cash payment type

     let y:i32=buyer_group.find_buyer(&PaymentType::Cash);
     println!("{}",y);
    
    //println!("{:?}",buyers);
    

    // Create 1 seller with payment_type of Cash, price of 10, balance of 0
    let mut seller1=Seller{
        payment_type:PaymentType::Cash,
        price:10.0,
        balance:0.0

    };
      
      buyer_group.buy(y,&mut seller1);
    

    // Call buy method on the buyer group passing the index of we have obtained right before and the seller
}
