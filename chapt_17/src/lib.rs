pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button{
    fn draw(&self) {
        // code to actually draw a button
    }
}

// State pattern example

pub struct Post{
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post{
        Post { 
            state: Some(Box::new(Draft{})),
            content: String::new()
        }
    }

    pub fn add_text(&mut self,text: &str){
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str{
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self){
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str{
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft{}

impl State for Draft{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{count: 1})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview{
    count: u32
}

impl State for PendingReview{
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        if self.count == 2 {
            Box::new(Published{})
        }else{
            self.count += 1;
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft{})
    }
}


struct Published{}


impl State for Published{
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}