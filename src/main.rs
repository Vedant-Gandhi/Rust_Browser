mod dom;
mod htmlparser;
mod csselements;
mod cssparer;
fn main() {
   let mut var=String::new();
   var.push('f');
   println!("Var after refrence ={:?}",var);
   let x=var;
   println!("X after refrence ={:?}",x);
   let pointer=&x;
   println!("Pointer={:?}",pointer);
   println!("Derefrenced Pointer :{:?}",*pointer);
}
