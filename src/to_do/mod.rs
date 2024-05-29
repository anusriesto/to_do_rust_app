pub mod structs;
pub mod enums;
pub mod traits;
//Factory flow yahan se sare factory flow ka use kar rahe
use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;

//ye jo enum hai ye factory types enum hai
pub enum ItemTypes{
    Pending(Pending),
    Done(Done)
}

pub fn to_do_factory(title:&str,status:TaskStatus)->ItemTypes{
    match status{
        TaskStatus::DONE=>{
            ItemTypes::Done(Done::new(title))
        },
        TaskStatus::PENDING=>{
            ItemTypes::Pending(Pending::new(title))
        }
    }
}