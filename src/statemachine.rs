pub struct Statemachine

{
    //states : HashMap<String, Box<dyn State>>,
    // SWITCH TO &MUT DYN STATE IF PERFORMANCE BECOMES AN ISSUE
    current_state : Box<dyn State>,
}

impl Statemachine
{
    pub fn new(start_state : Box<dyn State>) -> Statemachine {

        //let mut states = HashMap::<String,Box<dyn State>>::new();
        //states.insert(tag.clone(), start_state);

        // haha
        let machine = Statemachine { current_state : start_state };
        machine.current_state.enter();

        machine
    }
    
    pub fn transition(&mut self, state : Box<dyn State>) {

        if self.current_state.get_tag() == state.get_tag() { return; }

        self.current_state.exit();
        self.current_state = state;
        self.current_state.enter();
    }

    pub fn update(&mut self) {

        self.current_state.update();
        //self.states.iter().for_each(|x| (x.1.update)());
    }
}  
  
pub trait State: Send + Sync {

    fn update(&self);
    // called after exit
    fn enter(&self);
    // called when transition occures
    fn exit(&self);

    fn get_tag(&self) -> String;
}