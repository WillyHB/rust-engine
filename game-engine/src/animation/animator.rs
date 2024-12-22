use macroquad::time::get_time;

use  crate::animation::animation::Animation;

pub struct Animator<T> where T : Clone{

    animation : Option<Animation<T>>,
    is_playing : bool,
    counter : f64,
    current_frame : usize,
}

impl<T : Clone> Animator<T> {

    pub fn new() -> Animator<T> {

        Animator { animation : None, 
            is_playing : false, 
            counter : 0.0,
             current_frame : 0 
        }
    }

    pub fn loop_frames_mut<F : FnMut(&mut T)>(&mut self, mut test : F) {

        match &mut self.animation {

            Some(x) => {

                for frame in x.frames.iter_mut() {

                    test(frame);
                }
            },
            None => {},
        } 
    }
    
    pub fn remove(&mut self) {

        self.animation = None;
    }

    pub fn play(&mut self, anim : Animation<T>) {

        if self.is_playing &&self.animation.is_some() && self.animation.as_ref().unwrap().tag == anim.tag { return; } 

        self.animation = Some(anim);

        self.current_frame = self.animation.as_ref().unwrap().params.start_frame;
        self.counter = get_time();
        self.is_playing = true;
    }


    pub fn play_once(&mut self, anim : Animation<T>) {

        if self.animation.is_some() && self.animation.as_ref().unwrap().tag == anim.tag { return; } 

        self.play(anim);
    }

    pub fn pause(&mut self) {
        if !self.is_playing { return; }

        self.is_playing = false;

    }

    pub fn stop(&mut self) {
        if !self.is_playing { return; }
        self.is_playing = false;
    }

    pub fn has_animation(&self) -> bool {

        self.animation.is_some()
    }

    pub fn get_animation(&mut self) -> Option<Animation<T>> {

        self.animation.clone()
    }

    pub fn get_frame(&self, index : usize) -> Result<T, String> {
         
        match &self.animation {

            Some(x) => { 
                
                if x.frames.len() <= index { return Err(String::from("index out of bounds")) }
                Ok(x.frames[index].clone()) 
            },
            None => { Err(String::from("No animation on animator"))},
        }
    }

    pub fn is_playing(&self) -> bool {

        self.is_playing
    }

    pub fn current_frame(&self) -> usize {

        self.current_frame
    }

    pub fn next_frame(&mut self) {

        match &self.animation {

            Some(x) => {


                if self.current_frame + 1 >= x.frames.len() {
                    
                    if !x.params.repeat {

                        self.stop();
                        return;
                    }
                   

                    self.current_frame = 0;
                }

                else {
                    self.current_frame += 1;
                }
            },

            None => {

            }
        }
    }

    pub fn previous_frame(&mut self) {

        match &self.animation {

            Some(x) => {

                if self.current_frame > 0 {
                    self.current_frame -= 1;
                } else {
                    self.current_frame = x.frames.len()-1;
                }
            },

            None => {}
        }
    }

    pub fn update(&mut self ) {
        
        if self.is_playing {

            match &self.animation {

                Some(x) => {

                    if get_time() - self.counter > 1.0 / x.fps as f64 {

                        self.next_frame();
                        self.counter = get_time();
                    }
                },

                None => {

                    self.stop();
                }
            };
        }

    }
}