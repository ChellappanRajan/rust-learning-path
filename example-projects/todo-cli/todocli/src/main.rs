use std::env;


struct TodoItem{
    name: String,
    completed: char
}

struct  TodoItems{
    lists:Vec<TodoItem>
}

impl TodoItems {
    fn new()->TodoItems{
        TodoItems{lists: Vec::new()}
    }

    fn add_todo(&mut self,name:String){
        let todo_item = TodoItem::new(name);
        self.lists.push(todo_item);
    }
    fn print(&self){
        for TodoItem {name,completed} in self.lists.iter(){
            println!("* [{}] - {}",completed,name);
        }
    }
    fn mark_us_done(&mut self,index:usize){
        self.lists[index].completed = 'x';
    }
}


impl TodoItem {
    fn new(name:String)->Self {
        return  TodoItem{
            name,
            completed:' '
        };
    }
}

enum Command {
    Get,
    Add(String),
    Done(usize)
}

fn main() {
    //Read command from terminal
    let args:Vec<String> = env::args().collect();
    //Sice args return array of values we need to access second index value to get correct user entered command
    //Otherwise we will get realtive path string infor ["path","get"] 
    let query = &args[1];

    // let todo_item = TodoItem{
    //     name:"Learn Rust".to_string(),
    //     completed: ' '
    // };

    // let todo_item = TodoItem::new("Learn Rust".to_string());

    // let todo_item2 = TodoItem::new("Learn Daily".to_string());

    let mut todo_list = TodoItems::new();

    todo_list.add_todo("Learn Rust".to_string());
    todo_list.add_todo("Learn Daily".to_string());
    
    
   let command =  match query.as_str() {
        "get"=>Command::Get,
        "add"=>Command::Add(args[2].clone()),
        "done"=>{
            Command::Done(args[2].parse().expect("Error"))
        },
        _ =>panic!("Unrecognized Command")
    };


    match command {
        Command::Get=>{
            todo_list.print();
        }
        Command::Add(task)=>{
            todo_list.add_todo(task);
            todo_list.print();
        },
        Command::Done(index)=>{
            println!("{}",index);
            // todo_list.lists[index].completed = 'x';
            todo_list.mark_us_done(index);
            todo_list.print();
        }
    }
    // if query == "get" {
      
        // for TodoItem {name,completed} in todo_list.iter(){
        //     println!("* [{}] - {}",completed,name);
        // }
    // }

}
