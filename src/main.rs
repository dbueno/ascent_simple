use std::env;

use ascent::ascent;

ascent! {
   relation edge(i32, i32);
   relation path(i32, i32);
   relation edge_label(i32, i32, String);
   
   path(x, y) <-- edge(x, y);
   path(x, z) <-- edge(x, y), path(y, z);
}

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument is provided
    if args.len() < 2 {
        println!("Usage: {} <integer>", args[0]);
        return;
    }

    // Attempt to parse the first argument into an i32
    let input = match args[1].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Invalid integer provided");
            return;
        }
    };

   let mut prog = AscentProgram::default();
   let mut edges = Vec::new();
   for i in 1..input {
       edges.push((i, i+1));
   }
   prog.edge = edges;
   prog.edge_label = vec![(1, 2, "baa".to_string()), (2, 3, "oo".to_string())];
   prog.run();
   dbg!(prog.path.len());
   // println!("path: {:?}", prog.path);
   // for edge in prog.edge.iter() {
   //     println!("edge: {:?}", edge);
   // }
}
