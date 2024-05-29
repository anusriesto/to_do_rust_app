mod to_do;
use to_do::enums::TaskStatus;
use to_do::to_do_factory;
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;
use to_do::traits::create::Create;
use to_do::ItemTypes;
use to_do::traits::delete::Delete;
use to_do::traits::edit::Edit;
use to_do::traits::get::Get;
mod state;
use std::env;
use state::{read_file,write_to_file};
use serde_json::value::Value;
use serde_json::{Map,json};

//abhi bas json file rakh rahe with state
fn main(){
    let args:Vec<String>=env::args().collect();
    let status:&String=&args[1];
    let title:&String=&args[2];
    let mut state:Map<String,Value>=read_file("./state.json");
    println!("Before operation:{:?}",state);
    state.insert(title.to_string(),json!(status));
    println!("After operation:{:?}",state);
    write_to_file("./state.json", &mut state);


}






// fn main() {
//     // let done=Done::new("Shopping");
//     // println!("{}",done.super_struct.title);
//     // println!("{}",done.super_struct.status.stringify());
//     // let pending=Pending::new("laundry");
//     // println!("{}", pending.super_struct.title);
//     // println!("{}", pending.super_struct.status.stringify());

//     //ye factory flow hai
//     let to_do_item=to_do_factory("washing", TaskStatus::DONE);
//     match to_do_item{
//         ItemTypes::Done(item)=>{
//             // println!("{}",item.super_struct.status.stringify());
//             // println!("{}",item.super_struct.title)
//             item.get(&item.super_struct.title);
//             item.delete(&item.super_struct.title);
//         },
//         ItemTypes::Pending(item)=>{
//             // println!("{}",item.super_struct.status.stringify());
//             // println!("{}",item.super_struct.title)
//             item.get(&item.super_struct.title);
//             item.set_to_done(&item.super_struct.title);

//         }
//     }

// }
