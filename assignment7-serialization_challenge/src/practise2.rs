

// use std::error::Error;
// use std::marker::PhantomData;

// use borsh::{BorshDeserialize, BorshSerialize};
// use serde::{Deserialize, Serialize};
// use wincode::{SchemaRead, SchemaWrite};

// // =========================================================
// // TEST DATA TYPE
// // =========================================================

// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Serialize,
//     Deserialize,
//     BorshSerialize,
//     BorshDeserialize,
//     SchemaRead,
//     SchemaWrite,
// )]
// pub struct Person {
//     pub name: String,
//     pub age: u8,
// }

// // =========================================================
// // SERIALIZER TRAIT
// // =========================================================

// trait Serializer {
//     fn to_bytes<T>(value: &T) -> Result<Vec<u8>, Box<dyn Error>>
//     where
//         T: Serialize
//             + for<'de> Deserialize<'de>
//             + BorshSerialize
//             + BorshDeserialize
//             + SchemaWrite
//             + SchemaRead;

//     fn from_bytes<T>(bytes: &[u8]) -> Result<T, Box<dyn Error>>
//     where
//         T: Serialize
//             + for<'de> Deserialize<'de>
//             + BorshSerialize
//             + BorshDeserialize
//             + SchemaWrite
//             + SchemaRead;
// }

// // =========================================================
// // BORSH SERIALIZER
// // =========================================================

// struct Borsh;

// impl Serializer for Borsh {
//     fn to_bytes<T>(value: &T) -> Result<Vec<u8>, Box<dyn Error>>
//     where
//         T: Serialize
//             + for<'de> Deserialize<'de>
//             + BorshSerialize
//             + BorshDeserialize
//             + SchemaWrite
//             + SchemaRead,
//     {
//         Ok(borsh::to_vec(value)?)
//     }

//     fn from_bytes<T>(bytes: &[u8]) -> Result<T, Box<dyn Error>>
//     where
//         T: Serialize
//             + for<'de> Deserialize<'de>
//             + BorshSerialize
//             + BorshDeserialize
//             + SchemaWrite
//             + SchemaRead,
//     {
//         Ok(T::try_from_slice(bytes)?)
//     }
// }

// // =========================================================
// // JSON SERIALIZER
// // =========================================================

// struct Json;

// impl Serializer for Json {
//     fn to_bytes<T>(value: &T) -> Result<Vec<u8>, Box<dyn Error>>
//     where
//         T: Serialize
//             + for<'de> Deserialize<'de>
//             + BorshSerialize
//             + BorshDeserialize
//             + SchemaWrite
//             + SchemaRead,
//     {
//         Ok(serde_json::to_vec(value)?)
//     }

//     fn from_bytes<T>(bytes: &[u8]) -> Result<T, Box<dyn Error>>
//     where
//         T: Serialize
//             + for<'de> Deserialize<'de>
//             + BorshSerialize
//             + BorshDeserialize
//             + SchemaWrite
//             + SchemaRead,
//     {
//         Ok(serde_json::from_slice(bytes)?)
//     }
// }

// // =========================================================
// // WINCODE SERIALIZER
// // =========================================================

// struct Wincode;

// impl Serializer for Wincode {
//     fn to_bytes<T>(value: &T) -> Result<Vec<u8>, Box<dyn Error>>
//     where
//         T: Serialize
//             + for<'de> Deserialize<'de>
//             + BorshSerialize
//             + BorshDeserialize
//             + SchemaWrite
//             + SchemaRead,
//     {
//         Ok(wincode::serialize(value)?)
//     }

//     fn from_bytes<T>(bytes: &[u8]) -> Result<T, Box<dyn Error>>
//     where
//         T: Serialize
//             + for<'de> Deserialize<'de>
//             + BorshSerialize
//             + BorshDeserialize
//             + SchemaWrite
//             + SchemaRead,
//     {
//         Ok(wincode::deserialize(bytes)?)
//     }
// }

// // =========================================================
// // GENERIC STORAGE
// // =========================================================

// struct Storage<T, S>
// where
//     S: Serializer,
// {
//     data: Option<Vec<u8>>,
//     serializer: S,
//     marker: PhantomData<T>,
// }

// impl<T, S> Storage<T, S>
// where
//     T: Serialize
//         + for<'de> Deserialize<'de>
//         + BorshSerialize
//         + BorshDeserialize
//         + SchemaWrite
//         + SchemaRead,
//     S: Serializer,
// {
//     // -----------------------------------------------------
//     // CREATE NEW STORAGE
//     // -----------------------------------------------------

//     fn new(serializer: S) -> Self {
//         Self {
//             data: None,
//             serializer,
//             marker: PhantomData,
//         }
//     }

//     // -----------------------------------------------------
//     // SAVE DATA
//     // -----------------------------------------------------

//     fn save(&mut self, value: &T) -> Result<(), Box<dyn Error>> {
//         let bytes = S::to_bytes(value)?;
//         self.data = Some(bytes);
//         Ok(())
//     }

//     // -----------------------------------------------------
//     // LOAD DATA
//     // -----------------------------------------------------

//     fn load(&self) -> Result<T, Box<dyn Error>> {
//         let bytes = self
//             .data
//             .as_ref()
//             .ok_or("No data stored")?;

//         let value = S::from_bytes(bytes)?;

//         Ok(value)
//     }

//     // -----------------------------------------------------
//     // CHECK DATA EXISTS
//     // -----------------------------------------------------

//     fn has_data(&self) -> bool {
//         self.data.is_some()
//     }
// }

// // =========================================================
// // MAIN
// // =========================================================

// fn main() -> Result<(), Box<dyn Error>> {
//     let person = Person {
//         name: "Akshay".to_string(),
//         age: 22,
//     };

//     // =====================================================
//     // BORSH STORAGE
//     // =====================================================

//     let mut borsh_storage = Storage::<Person, Borsh>::new(Borsh);

//     borsh_storage.save(&person)?;

//     let loaded_borsh = borsh_storage.load()?;

//     println!("Loaded from Borsh: {:?}", loaded_borsh);

//     // =====================================================
//     // JSON STORAGE
//     // =====================================================

//     let mut json_storage = Storage::<Person, Json>::new(Json);

//     json_storage.save(&person)?;

//     let loaded_json = json_storage.load()?;

//     println!("Loaded from Json: {:?}", loaded_json);

//     // =====================================================
//     // WINCODE STORAGE
//     // =====================================================

//     let mut wincode_storage = Storage::<Person, Wincode>::new(Wincode);

//     wincode_storage.save(&person)?;

//     let loaded_wincode = wincode_storage.load()?;

//     println!("Loaded from Wincode: {:?}", loaded_wincode);

//     Ok(())
// }

// // =========================================================
// // TESTS
// // =========================================================

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_borsh_storage() {
//         let person = Person {
//             name: "Akshay".to_string(),
//             age: 22,
//         };

//         let mut storage = Storage::<Person, Borsh>::new(Borsh);

//         storage.save(&person).unwrap();

//         let loaded = storage.load().unwrap();

//         assert_eq!(person, loaded);
//     }

//     #[test]
//     fn test_json_storage() {
//         let person = Person {
//             name: "Akshay".to_string(),
//             age: 22,
//         };

//         let mut storage = Storage::<Person, Json>::new(Json);

//         storage.save(&person).unwrap();

//         let loaded = storage.load().unwrap();

//         assert_eq!(person, loaded);
//     }

//     #[test]
//     fn test_wincode_storage() {
//         let person = Person {
//             name: "Akshay".to_string(),
//             age: 22,
//         };

//         let mut storage = Storage::<Person, Wincode>::new(Wincode);

//         storage.save(&person).unwrap();

//         let loaded = storage.load().unwrap();

//         assert_eq!(person, loaded);
//     }
// }
