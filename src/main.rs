use std::sync::{Arc, Mutex, Barrier};
use std::thread;
use std::collections::HashMap;

#[derive(Debug, Clone)] 
struct Node {
    id: u32,
    honest: bool,
    origin: bool
}

#[derive(Debug, Clone)] 
struct Message {
    content: String,
    sender_id: u32,
}
struct SharedTable {
    messages: HashMap<u32, Vec<Message>>
}

fn main() {
    let nodes = vec![
        Node { id: 0, honest: true, origin: true },
        Node { id: 1, honest: true, origin: false },
        Node { id: 2, honest: true, origin: false },
        Node { id: 3, honest: false, origin: false },
        Node { id: 4, honest: true, origin: false },
    ];
    let nb_nodes = nodes.len();

    let shared_table = Arc::new(Mutex::new(SharedTable { messages: HashMap::new() }));
    let shared_table_clone = Arc::clone(&shared_table);

    let barrier: Arc<Barrier> = Arc::new(Barrier::new(nb_nodes - 1)); // Adjusted for skipped node
    let mut handles = vec![];

    let handle_origin = thread::spawn(move || {
        let hello_message = Message { content: "ATTACK".to_string(), sender_id: 0 };
    
        let mut table = shared_table.lock().unwrap();
        for i in 1..nb_nodes {
            table.messages.entry(i.try_into().unwrap()).or_insert_with(Vec::new).push(hello_message.clone());
        }
    });

    // Wait for the thread to complete its execution
    handle_origin.join().unwrap();


    // SIMULATION
    for node in nodes[1..].to_vec() {  // Clone the slice starting from the second element

        let shared_table_clone = Arc::clone(&shared_table_clone);
        let barrier_clone = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            let mut message_counts: HashMap<String, usize> = HashMap::new();

            for epoch in 0..(nb_nodes - 1) {
                let mut table = shared_table_clone.lock().unwrap();
        
                // Determine the target node for this epoch
                // Offset by 1 to ensure that node 0 is also included
                let target_node_id = (node.id + epoch as u32 + 1) % nb_nodes as u32;
                
                // Ensure the target node is not the current node
                if target_node_id != node.id {
                    let message_content = if node.honest {
                        "ATTACK".to_string()  // Using the same message for simplicity
                    } else {
                        "FAKE".to_string()
                    };
        
                    let new_message = Message {
                        content: message_content,
                        sender_id: node.id,
                    };
                    table.messages.entry(target_node_id).or_insert_with(Vec::new).push(new_message);
                }
        
                // Collect messages for counting
                for msg in table.messages.get(&node.id).unwrap_or(&Vec::new()) {
                    *message_counts.entry(msg.content.clone()).or_insert(0) += 1;
                }

                drop(table);
                
                barrier_clone.wait();
            }

            // Decision logic
            let decided_message = message_counts.into_iter()
            .max_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0))) // Choose the message with highest count, tie-break by content
            .map(|(msg, _)| msg)
            .unwrap_or_else(|| "No consensus".to_string());

            println!("Node {} decided on message: {}", node.id, decided_message);
        });

        handles.push(handle);
    }
    

    for handle in handles {
        handle.join().unwrap();
    }

    // Access shared_table after all epochs
    let shared_table_lock = shared_table_clone.lock().unwrap();
    for (node_id, messages) in &shared_table_lock.messages {
        println!("Messages received by Node {}: ", node_id);
        for message in messages {
            println!("\tContent: '{}', Sender ID: {}", message.content, message.sender_id);
        }
    }
    
}