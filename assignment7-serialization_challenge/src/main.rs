use std::{fs, io::Read};

fn read_file(path:String)->Result<u32,std::error::E>{
    let mut f=fs::File::open(path)?;
    let mut x=String::new();
    f.read_to_string(&mut x);
    let y:u32=x.parse()?;
    Ok(y)
}

fn main(){
    println!("wow");
}


use borsh::{BorshDeserialize,BorshSerialize};
use std::{
    collections::VecDeque, env
};

#[derive(BorshDeserialize,BorshSerialize)]
struct Todo{
    id:u32,
    description:String,
    status:bool,
    time:u32
}

struct Queue<T>{
    items:VecDeque<T>
}

impl<T> Queue<T>{
    fn add(&mut self, todo:T){
        self.items.push_back(todo);
    }
    fn len(&self)->usize{
        self.items.len()
    }
    //pop, is_empty,
    
}


fn main(){
    let todo=Todo{id:1,description:"code 8 hours".to_string(),status:false,time:100};

    let mut queue=Queue{items:VecDeque::new()};
    queue.add(todo);

    let args: Vec<String> = env::args().collect();
    println!("args : {:?}",args);

    match args[1].as_str(){
        "add"=>{

        },
        _=>{}
    }
}





















// use borsh::{BorshDeserialize, BorshSerialize};
// use std::{
//     collections::VecDeque,
//     env,
//     fs::{self, File},
//     io::{Read, Write},
//     path::Path,
//     time::{SystemTime, UNIX_EPOCH},
// };

// const FILE_PATH: &str = "todos.bin";

// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// struct Todo {
//     id: u64,
//     description: String,
//     created_at: u64,
// }

// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// pub struct Queue<T> {
//     items: VecDeque<T>,
// }

// impl<T> Queue<T> {
//     // Create empty queue
//     pub fn new() -> Self {
//         Self {
//             items: VecDeque::new(),
//         }
//     }

//     // Add item to back
//     pub fn enqueue(&mut self, item: T) {
//         self.items.push_back(item);
//     }

//     // Remove oldest item
//     pub fn dequeue(&mut self) -> Option<T> {
//         self.items.pop_front()
//     }

//     // View oldest item without removing
//     pub fn peek(&self) -> Option<&T> {
//         self.items.front()
//     }
//     pub fn len(&self) -> usize {
//         self.items.len()
//     }
//     pub fn is_empty(&self) -> bool {
//         self.items.is_empty()
//     }
// }

// // Save queue to disk using Borsh serialization.
// fn save_queue(queue: &Queue<Todo>) -> Result<(), Box<dyn std::error::Error>> {
//     let serialized_data = borsh::to_vec(queue)?;
//     // Create/overwrite file
//     let mut file = File::create(FILE_PATH)?;
//     file.write_all(&serialized_data)?;

//     Ok(())
// }
// /// Load queue from disk.
// fn load_queue() -> Result<Queue<Todo>, Box<dyn std::error::Error>> {
//     if !Path::new(FILE_PATH).exists() {
//         return Ok(Queue::new());
//     }

//     // Read all bytes
//     let mut file = File::open(FILE_PATH)?;
//     let mut buffer = Vec::new();

//     file.read_to_end(&mut buffer)?;

//     // Deserialize bytes into queue
//     let queue = Queue::<Todo>::try_from_slice(&buffer)?;
//     Ok(queue)
// }

// // Utility Functions
// // Generate current unix timestamp
// fn current_timestamp() -> u64 {
//     SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_secs()
// }

// // Generate next ID.
// // use: current queue length + 1
// fn next_id(queue: &Queue<Todo>) -> u64 {
//     queue.len() as u64 + 1
// }

// // CLI Logic
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Load previous queue from disk
//     let mut queue = load_queue()?;
//     let args: Vec<String> = env::args().collect();

//     // Example:
//     // cargo run -- add "Buy milk"
//     if args.len() < 2 {
//         println!("Usage:");
//         println!("todo add \"task\"");
//         println!("todo list");
//         println!("todo done");

//         return Ok(());
//     }

//     match args[1].as_str() {
//         //ADD TASK
//         "add" => {
//             if args.len() < 3 {
//                 println!("Please provide task description");
//                 return Ok(());
//             }
//             let description = args[2..].join(" ");

//             let todo = Todo {
//                 id: next_id(&queue),
//                 description,
//                 created_at: current_timestamp(),
//             };

//             queue.enqueue(todo);
//             // Persist updated queue
//             save_queue(&queue)?;
//             println!("Task added successfully!");
//         }
//         // LIST TASKS
//         "list" => {
//             if queue.is_empty() {
//                 println!("No pending tasks.");
//                 return Ok(());
//             }

//             println!("Pending Tasks:\n");
//             for todo in &queue.items {
//                 println!(
//                     "ID: {}\nTask: {}\nCreated At: {}\n",
//                     todo.id,
//                     todo.description,
//                     todo.created_at
//                 );
//             }
//         }

//         // COMPLETE TASK
//         "done" => {
//             match queue.dequeue() {
//                 Some(todo) => {
//                     println!("Completed task:");
//                     println!("{:?}", todo);

//                     // Persist updated queue
//                     save_queue(&queue)?;
//                 }

//                 None => {
//                     println!("No tasks to complete.");
//                 }
//             }
//         }
//         _ => {
//             println!("Unknown command");
//         }
//     }

//     Ok(())
// }
