use borsh::{BorshSerialize,BorshDeserialize};
use serde::{Serialize,Deserialize};
use wincode::{SchemaRead,SchemaWrite};

trait Serializer{
    // fn serialize(x:&Person)->&Vec<u8>;
    // fn deserialize(x:&[u8])->Person;

    fn serialize<T>(x:&T)->&Vec<u8>;
    fn deserialize<T>(x:&[u8])->T;
}

#[derive(BorshSerialize,BorshDeserialize,Debug, Serialize,Deserialize, SchemaRead,SchemaWrite)]
pub struct Person{
    pub name:String,
    pub age:u8
}

fn main() {
    println!("Hello, world!");
    let p=Person{name:"Akshay".to_string(), age:22};

    let borsh_serialised=borsh::to_vec(&p).unwrap();
    println!("borsh_serialised : {:?}",borsh_serialised);
    let borsh_deserialised=Person::try_from_slice(&borsh_serialised).unwrap();
    println!("borsh_deserialised : {:?}",borsh_deserialised);

    let serde_serialised=serde_json::to_vec(&p).unwrap();
    println!("serde_serialised : {:?}",serde_serialised);
    let serde_deserialised:Person=serde_json::from_slice(&serde_serialised).unwrap();
    println!("serde_deserialised : {:?}",serde_deserialised);
    
    let wincode_serialised=wincode::serialize(&p).unwrap();
    println!("wincode_serialised : {:?}",wincode_serialised);
    let wincode_deserialised:Person=wincode::deserialize(&wincode_serialised).unwrap();
    println!("wincode_deserialised : {:?}",wincode_deserialised);


    let x:u32=6;
    let le_format=x.to_le_bytes();
    let be_format=x.to_be_bytes();
    println!("le : {:?}, be: {:?}",le_format,be_format);
}
