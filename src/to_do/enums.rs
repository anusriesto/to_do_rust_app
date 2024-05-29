use std::fmt;

pub enum TaskStatus{
    DONE,
    PENDING
}
//this enum will help when we have to define the task but when we have to write to file or status 
//of our task we will build a method to enable our enum to be represented in string format
//impl is one way we can other way also which I ll define below
impl TaskStatus{
    pub fn stringify(&self)-> String{
        match &self{
            &Self::DONE=>{"Done".to_string()},
            &Self::PENDING=>{"Pending".to_string()}
        }
    }
}
// ye bhi print he karta hai

// impl fmt::Display for TaskStatus{
//     fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
//         match &self {
//             &self::DONE=>{write!(f,"DONE")},
//             &self::PENDING=>{write!(f,"PENDING")}
//         }

//     }
// }
