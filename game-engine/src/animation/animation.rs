#[derive(Clone)]
pub struct Animation<T> where T : Clone {

    pub tag : String,
    pub frames : Vec<T>,
    pub fps : u32,
    pub params : AnimationParams,
    // Can be sprite animation, texture animation, NUMBER ANIMATION IT IS GENERIC
}

#[derive(Clone)]
pub struct AnimationParams {
    pub repeat : bool,
    pub reverse : bool, 
    pub start_frame : usize,
}

impl Default for AnimationParams {

    fn default() -> AnimationParams {
        AnimationParams {

            repeat : true,
            reverse : false,
            start_frame : 0,
        }
    }
}

impl<T : Clone> Animation<T> {

    pub fn new(tag : &str, frames : Vec<T>, fps : u32, params : &AnimationParams) -> Animation<T> {

        Animation { 
            tag : String::from(tag),
            frames,
            fps,
            params : params.clone(),
        }
    }

    pub fn from(animation : &Animation<T>) -> Animation<T> {

        let mut frames : Vec<T> = Vec::new();

        for frame in animation.frames.iter() {

            frames.push(frame.clone());
        }
        
        Animation {
            tag : animation.tag.clone(),
            frames,
            fps : animation.fps,
            params : animation.params.clone(),
        }

    }
}