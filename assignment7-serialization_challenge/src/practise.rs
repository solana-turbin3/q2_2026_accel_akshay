pub trait Storable{
    fn size(&self)->usize;
}

pub struct Image{mb:usize}
pub struct Video{gb:usize}

impl Storable for Image{
    fn size(&self)->usize {
        self.mb
    }
}
impl Storable for Video{
    fn size(&self)->usize {
        self.gb
    }
}

pub struct StorageSystem<T:Storable>{
    pub items:Vec<T>
}

impl<T:Storable> StorageSystem<T>{
    fn add(){}
    fn total_size(){}
}

fn main(){
    let i=Image{mb:10};
    let v=Video{gb:3};

    let s=StorageSystem{items:vec![]};
}









// trait Area{
//     fn area(&self);
// }

// struct Square{side:u32}
// struct Circle{rad:u32}

// impl Area for Square{
//     fn area(&self) {
//         println!("area is : {}",self.side*self.side);
//     }
// }
// impl Area for Circle{
//     fn area(&self) {
//         println!("area is : {}",self.rad*self.rad*3);
//     }
// }

// fn process_area<T:Area>(shape:&T){
//     shape.area();
// }

// fn process_areas<T:Area>(shapes:&[T]){
//     for shape in shapes{
//         shape.area();
//     }
// }

// pub struct AreaProcessor{
//     pub total_area:u32,
//     pub total_shapes:u32
// }

// impl AreaProcessor{
//     fn process(&self,shape:&dyn Area){
//         self.total_shapes+=1;
//         self.total_area+=shape.area();
//     }
// }

// fn main(){
//     let c1=Circle{rad:4};
//     let s1=Square{side:4};
//     let shapes=[Circle{rad:5}, Circle{rad:6}];
//     process_areas(&shapes);

//     println!("shape 1 area");
//     process_area(&c1);
//     process_area(&s1);
    
//     let mut areaP=AreaProcessor{total_area:0,total_shapes:0};
//     for shape in shapes{
//         areaP.process(&shape);
//     }
// }






// pub trait Payment{
//     fn pay(&self);
// }

// trait Log{
//     fn log(&self);
// }
// struct Upi;
// struct CreditCard;
// struct Cash;

// impl Payment for Upi{
//     fn pay(&self) {
//         println!("i am paying via upi");
//     }
// }
// impl Payment for CreditCard{
//     fn pay(&self) {
//         println!("i am paying via credit card");
//     }
// }

// impl Log for Upi{
//     fn log(&self) {
//         println!("upi payment done");
//     }
// }
// impl Log for CreditCard{
//     fn log(&self) {
//         println!("creditcard payment done");
//     }
// }

// fn payment_process<T:Payment+Log>(method:T){
//     let x=method.pay();
//     method.log();
// }
// fn payment_process2<T>(method:T) where T:Payment+Log{}

// pub struct PaymentProcessor<T:Payment>{
//     pub method:T
// }

// fn main(){
//     let u=Upi;
//     let c=CreditCard;
//     let cash=Cash;
//     payment_process(u);
//     payment_process(c);
//     //error in below
//     // payment_process(cash);


//     let p1=PaymentProcessor{method:Upi};
//     p1.method.pay();
// }