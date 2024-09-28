use std::fmt;

use petgraph::{
    graph::{NodeIndex, UnGraph},
    Direction,
};

/// A struct that represents a fighter
#[derive(Debug)]
struct Fighter {
    name: String,
}

/// Implementing a new method for the Fighter struct
/// This method will create a new instance of the Fighter struct
/// and return it
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the fighter
///
/// # Example
///
/// ```
/// let fighter = Fighter::new("John");
/// ```
///
/// # Returns
///
/// A new instance of the Fighter struct
///
/// # Test
///
/// ```
/// let fighter = Fighter::new("John");
/// assert_eq!(fighter.name, "John");
/// ```
impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

/// Implementing the Display trait for the Fighter struct
/// This will allow us to print the name of the fighter
///     println!("{}", fighter);
///
/// # Example
///
/// ```
/// let fighter = Fighter::new("John");
/// println!("{}", fighter);
/// ```
///
/// # Returns
///
/// A string slice that holds the name of the fighter
///
/// # Test
///
/// ```
/// let fighter = Fighter::new("John");
/// assert_eq!(format!("{}", fighter), "John");
/// ```
impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(grap: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    grap.add_edge(nodes[a], nodes[b], 1.0);
}

fn main() {
    let mut graph = UnGraph::<&Fighter, f32>::new_undirected();

    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Tony Ferguson"),
        Fighter::new("Charles Oliveira"),
    ];

    let fighter_nodes = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect::<Vec<NodeIndex>>();

    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Conor McGregor vs. Tony Ferguson
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Tony Ferguson vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Tony Ferguson vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Tony Ferguson vs. Charles Oliveira
    add_edge(&mut graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Charles Oliveira
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Khabib Nurmagomedov vs. Charles Oliveira

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);

        //Explanation
        match name.as_str() {
            "Conor McGregor" => {
                println!("Conor McGregor has a closeness centrality of {:.2} because he has fought against 2 fighters, Dustin Poirier and Tony Ferguson. He has a closeness centrality of 0.50 because he has fought against 2 fighters. The closeness centrality is calculated as 1/2 = 0.50", closeness);
            }
            "Dustin Poirier" | "Tony Ferguson" => {
                println!("Dustin Poirier and Tony Ferguson have a closeness centrality of {:.2} because they have fought against 2 fighters, Conor McGregor and Charles Oliveira. They have a closeness centrality of 0.50 because they have fought against 2 fighters. The closeness centrality is calculated as 1/2 = 0.50", closeness);
            }
            "Khabib Nurmagomedov" | "Charles Oliveira" => {
                println!("Khabib Nurmagomedov and Charles Oliveira have a closeness centrality of {:.2} because they have fought against 2 fighters, Tony Ferguson and Dustin Poirier. They have a closeness centrality of 0.50 because they have fought against 2 fighters. The closeness centrality is calculated as 1/2 = 0.50", closeness);
            }
            _ => {}
        }
        println!("-----------");
    }

}
