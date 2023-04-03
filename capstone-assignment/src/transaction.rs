use chrono::NaiveDate;
use crate::location::*;
#[derive(Debug)]
pub struct Transaction{


transaction_id:u32,
client_id:u32,
asset_name:String,
country:Country,
continent:Continent,
amount:f64,
days_under_management:i64



}



impl Transaction{

pub fn from_csv_line(line:&str)->Result<Transaction,String>{

   let fields:Vec<_>=line.split(',').collect();
    //fields=['1','101','Apple']
    if fields.len()!=7 {

        return Err("Incorrect number of columns".to_owned());
    }

    let transaction_id=fields[0].parse::<u32>().unwrap();
    let client_id=fields[1].parse::<u32>().unwrap();
    let asset_name=fields[2].to_uppercase();
    let transaction_start_date=NaiveDate::parse_from_str(fields[3], "%Y-%m-%d").unwrap();
    let transaction_end_date=NaiveDate::parse_from_str(fields[4], "%Y-%m-%d").unwrap();
    let country=fields[5].parse::<Country>()?;    //? for recoverable error
    let amount=fields[6].parse::<f64>().unwrap();
    let days_under_management=(transaction_end_date-transaction_start_date).num_days();
    let continent=country.country_to_continent();
    let transaction=Transaction{

        transaction_id:transaction_id,
       client_id:client_id,
       asset_name:asset_name,
       country:country,
       continent:continent,
       amount:amount,
      days_under_management:days_under_management


    };

    Ok(transaction)

}



}