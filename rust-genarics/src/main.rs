struct Cat
{
   weight: f64,
   speed: f64
}

struct Dog
{
   weight: f64,
   speed: f64
}

trait Animal
{
   fn max_speed(&self) -> f64;
}

impl Animal for Cat
{
   fn max_speed(&self) -> f64
   {
      self.speed
   }
}

impl Animal for Dog
{
   fn max_speed(&self) -> f64
   {
      self.speed
   }
}

struct SqueakyToy
{
   weight: f64
}

struct Stick
{
   weight: f64
}

trait Toy
{
   fn weight(&self) -> f64;
}

impl Toy for SqueakyToy
{
   fn weight(&self) -> f64
   {
      self.weight
   }
}

impl Toy for Stick
{
   fn weight(&self) -> f64
   {
      self.weight
   }
}

struct AnimalChasingToy<A: Animal, T: Toy>
{
   animal: A,
   toy: T
}

trait AnimalChasesToy<A: Animal, T: Toy>
{
   fn chase(&self);
}

impl<A: Animal, T: Toy> AnimalChasesToy<A, T> for AnimalChasingToy<A, T>
{
   fn chase(&self)
   {
      println!("chase")
   }
}

fn main () {

}

struct JSJIT(u64);

enum JSJITorExpr {
   Jit { label: Box<JSJIT> },
   Expr { expr: Box<JSExpr> }
}

enum JSExpr {
   Integer { value: u64 },
   String { value: String },
   OperatorAdd { lexpr: Box<JSJITorExpr>, rexpr: Box<JSJITorExpr> },
   OperatorMul { lexpr: Box<JSJITorExpr>, rexpr: Box<JSJITorExpr> }
}

fn jump(l: JSJIT) -> JSJITorExpr
{
   //jump to compiled code
   //this depends on implementation
   //so we will just leave this as a stub
   JSJITorExpr::Jit { label: JSJIT(0) }
}

fn eval(e: JSJITorExpr) -> JSJITorExpr
{
   match e
   {
      JSJITorExpr::Jit { label: label } => jump(label),
      JSJITorExpr::Expr { expr: expr } => {
         let rawexpr = *expr;
         match rawexpr
         {
            JSExpr::Integer {..} => JSJITorExpr::Expr { expr: Box::new(rawexpr) },
            JSExpr::String {..} => JSJITorExpr::Expr { expr: Box::new(rawexpr) },
            JSExpr::OperatorAdd { lexpr: l, rexpr: r } => {
               let l = eval(*l);
               let r = eval(*r);
               //call add op codes for possible l,r representations
               //should return wrapped value from above
               JSJITorExpr::Jit { label: JSJIT(0) }
            }
            JSExpr::OperatorMul { lexpr: l, rexpr: r } => {
               let l = eval(*l);
               let r = eval(*r);
               //call mul op codes for possible l,r representations
               //should return wrapped value from above
               JSJITorExpr::Jit { label: JSJIT(0) }
            }
         }
      }
   }
}

pub trait HList: Sized {}

pub struct HNil;
impl HList for HNil {}

pub struct HCons<H, T> {
   pub head: H,
   pub tail: T,
}
impl<H, T: HList> HList for HCons<H, T> {}
impl<H, T> HCons<H, T> {
   pub fn pop(self) -> (H, T) {
      (self.head, self.tail)
   }
}



