mod navigator;
mod transportation_strategy;


use crate::navigator::Navigator;
use crate::transportation_strategy::{walking, publictransport};

pub fn active(){
    
    //navigator의 상관없이 transportation의 interface를 호출함
 
    let navigator = Navigator::new(walking::Walking);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = Navigator::new(publictransport::PublicTransport);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");
}