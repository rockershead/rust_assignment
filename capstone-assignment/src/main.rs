use std::fs::File;
use std::io::{self, BufReader, BufRead};
use transaction::Transaction;

mod transaction;
mod location;


fn main() {


let file=File::open("./transactions.csv").unwrap();
let reader=BufReader::new(file);

let mut transactions:Vec<Transaction>=Vec::new();
let mut skipped_lines:Vec<(usize, String, String)>=Vec::new();


for(idx,line) in reader.lines().enumerate(){                   //enumerate gives tuples in format:(index,content)

   if idx==0{

    continue;
   }
  
  let line_str=line.unwrap();
  let parsed_transaction=Transaction::from_csv_line(&line_str);

  match parsed_transaction{

     Ok(transaction)=>transactions.push(transaction),
     Err(err)=>skipped_lines.push((idx,err,line_str))
  }

  for transaction in transactions.iter(){

     println!("{:?}",transaction);

  }

  for skipped_transaction in skipped_lines.iter(){

    println!("{:?}",skipped_transaction);

 }

}


}
