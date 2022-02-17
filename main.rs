use std::collections::HashMap


impl Todo {
    fn main() {
    
        let action = std::env::args().nth(1).expect("Please specify an action");
        let item = std::env::args().nth(2).expect("Please specify and item");
    
        let mut todo = Todo {
            map: HashMap::new(),
        };
        if action == "add" {
            todo.insert(item);
            match todo.insert(item);
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occured: {}", why),
                
        }
    
    println!("{:?}, {:?}", action, item);
    }
    
    struct Todo {
        // use rust built in HashMap to store key = val pairs
        map: HashMap<String, bool>,
    }

    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }

    fn save(self) -> Results<(), std::io::Error> {
        let mut content = String::new();
        for (k,v) in self.map {
            let record = format!("{}\t{}n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
}

