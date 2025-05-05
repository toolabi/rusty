pub trait Draw {
    fn draw(&self);
}


pub struct Screen{
    pub components: Vec<Box<dyn Draw>>
}


impl Screen {

    pub fn run(&self){
        for i in self.components.iter(){
            i.draw();
        }
    }
    
}


pub struct Button {
    pub width: usize
}


impl Draw for Button {

    fn draw(&self) {
        //draw 
    }
    
}