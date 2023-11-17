use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request{
     EventBusMsg(String),
}

pub struct EventBus{
     link : AgentLink<EventBus>,
     subscriber: HashSet<HandlerId>,
}

impl Agent for EventBus{
     type Reach = Context<Self>;
     type Message = ();
     type Input = Request;
     type Output = String;

     fn create(link: AgentLink<Self>) -> Self {
         Self { 
          link, 
          subscriber: HashSet::new(),
          }
     }
     fn update(&mut self, msg: Self::Message) {}

     fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
          match msg{
               Request::EventBusMsg(s) =>{
                    for sub in self.subscriber.iter(){
                         self.link.respond(*sub, s.clone())
                    }
               }
          }
     }

     fn connected(&mut self, id: HandlerId) {
     self.subscriber.insert(id);
     }
     fn disconnected(&mut self, id: HandlerId) {
         self.subscriber.remove(&id);
     }
}