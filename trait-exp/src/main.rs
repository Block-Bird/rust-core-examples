
struct Sheep {
    naked: bool, 
    name: &'static str
}

trait  Animal {
    
    fn new (name : &'static str) -> Self ;

    fn noice (&self) -> & str; 

    fn name (&self ) -> & str; 

    fn talk (&self) {
        println!("{} says {}", self.name(), self.noice() );
    }

}


impl Sheep {
        
    fn is_naked (&self) -> bool {
        self.naked
    }

    fn shear (&mut self ) {
        if self.is_naked() {
            println!("{} is already Naked ", self.name);
        }
        else {
            println!("{} got a hair-cut ", self.name); 
            self.naked = true; 
        }
    }
   

}

impl Animal for Sheep {
    
    fn new (nameOfAnimal: &'static str ) -> Sheep {
        Sheep {name : nameOfAnimal, naked: false }
    }

    fn noice (&self) -> & str {
        if self.is_naked() {
            println!("{} speaks Baaaah! ", self.name);
        }
        else {
            println!("{} else speaks Baaaah! ", self.name);
        }

        &self.name
    }

    fn name (&self ) -> & str {
        println!("Name Function is being called and name is "); 
        &self.name
    }

}

fn main () {
    let mut aliza : Sheep = Animal::new(" Aliza-Batool"); 
    println!("Name Function is {}" , aliza.name());
    print!("noice Function is {}" , aliza.noice());
    aliza.talk()
    ;


    
}