use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Orbit {
    orbiter: String,
    orbitee: String,
}
impl Orbit {
    fn from_notation(orbit: String) -> Orbit {
        let orbit_parsed: Vec<String> = orbit.split(')').map(|x| x.parse().unwrap()).collect();
        Orbit {
            orbiter: (*orbit_parsed[1]).to_string(),
            orbitee: (*orbit_parsed[0]).to_string(),
        }
    }
}
#[derive(Debug)]
struct OrbitalGraph {
    nodes: Vec<String>,
    graph: HashMap<usize, Vec<usize>>,
}
impl OrbitalGraph {
    fn from_vec(orbits: Vec<Orbit>) -> OrbitalGraph {
        let mut graph = HashMap::new();
        let mut nodes: Vec<String> = Vec::new();
        for orbit in orbits.iter() {
            nodes.push(orbit.orbiter.clone());
            nodes.push(orbit.orbitee.clone());
        }
        nodes.sort();
        nodes.dedup();

        for orbit in orbits.iter() {
            let orbiter_index = nodes.iter().position(|r| *r == orbit.orbiter).unwrap();
            let orbitee_index = nodes.iter().position(|r| *r == orbit.orbitee).unwrap();
            graph.entry(orbitee_index).or_insert(Vec::new());
            graph.entry(orbiter_index).or_insert(Vec::new());
            graph
                .entry(orbitee_index)
                .and_modify(|x: &mut Vec<usize>| x.push(orbiter_index));
            graph
                .entry(orbiter_index)
                .and_modify(|x: &mut Vec<usize>| x.push(orbitee_index));
        }
        OrbitalGraph { nodes, graph }
    }
    fn dijkstra(&self, source: &String, destination: &String) -> Vec<&String> {
        println!("{:?}", self);
        let source_index = self.nodes.iter().position(|r| r == source).unwrap();
        let destination_index = self.nodes.iter().position(|r| r == destination).unwrap();
        println!("{:?} is at index {:?}", source, source_index);
        println!("{:?} is at index {:?}", destination, destination_index);

        let mut unvisited_nodes: Vec<usize> = (0..self.nodes.len()).collect();
        println!("Unvisited Nodes: {:?}", unvisited_nodes);

        let mut tentative_distance: Vec<usize> = unvisited_nodes
            .iter()
            .map(|&x| if x == source_index { 0 } else { usize::MAX })
            .collect();
        println!("Tentative distance: {:?}", tentative_distance);

        let mut node_heap = BinaryHeap::from([(tentative_distance[source_index], source_index)]);
        println!("Next Node Heap: {:?}", node_heap);

        let mut previous_node: Vec<usize> = vec![0; self.nodes.len()];

        let mut path: Vec<&String> = vec![&self.nodes[destination_index]];
        while !node_heap.is_empty() {
            println!("Next Node Heap: {:?}", node_heap);
            let (current_distance, current_node) = node_heap.pop().unwrap();
            if current_node == destination_index {
                break;
            }
            for neighbor in self.graph[&current_node].iter() {
                if !unvisited_nodes.contains(neighbor) {
                    continue;
                }
                unvisited_nodes.remove(
                    unvisited_nodes
                        .iter()
                        .position(|&x| x == *neighbor)
                        .unwrap(),
                );
                println!("Neighbor {:?}", neighbor);
                let new_tentative_distance = current_distance + 1;
                if new_tentative_distance < tentative_distance[*neighbor] {
                    println!("Found shorter path.");
                    tentative_distance[*neighbor] = new_tentative_distance;
                    node_heap.push((new_tentative_distance, *neighbor));
                    previous_node[*neighbor] = current_node;
                }
            }
        }
        println!("{:?}", previous_node);
        let mut current_node = destination_index;
        while current_node != source_index {
            current_node = previous_node[current_node];
            path.push(&self.nodes[current_node]);
        }
        path.reverse();
        path
    }
}
#[derive(Debug)]
struct OrbitalSystem {
    parent_map: HashMap<String, String>,
    orbital_bodies: HashSet<String>,
}
impl OrbitalSystem {
    fn from_vec(orbits: Vec<Orbit>) -> OrbitalSystem {
        let mut parent_map = HashMap::new();
        let mut orbital_bodies = HashSet::new();
        for orbit in orbits {
            parent_map.insert(orbit.orbiter.clone(), orbit.orbitee.clone());
            orbital_bodies.insert(orbit.orbiter);
            orbital_bodies.insert(orbit.orbitee);
        }
        OrbitalSystem {
            parent_map,
            orbital_bodies,
        }
    }
}
pub fn day6(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let orbits: Vec<Orbit> = contents
        .split('\n')
        .map(|x| x.parse().unwrap())
        .map(Orbit::from_notation)
        .collect();

    println!("{:?}", count_orbits(orbits));
}
fn count_orbits(orbits: Vec<Orbit>) -> u32 {
    let orbital_system = OrbitalSystem::from_vec(orbits);
    let mut counts = HashMap::new();
    let mut orbit_count = 0;
    for orbital_body in orbital_system.orbital_bodies {
        counts.insert(orbital_body.clone(), 0);
        if orbital_body == "COM" {
            continue;
        }
        let mut temp_orbital_body = &orbital_system.parent_map[&orbital_body];
        while temp_orbital_body != "COM" {
            temp_orbital_body = &orbital_system.parent_map[&temp_orbital_body.clone()];
            orbit_count += 1;
            counts.entry(orbital_body.clone()).and_modify(|x| *x += 1);
        }
        // Plus one for final orbit
        orbit_count += 1;
    }
    println!("{:?}", counts);
    orbit_count
}
#[cfg(test)]
mod tests {
    use crate::day6::{count_orbits, Orbit, OrbitalGraph};

    #[test]
    fn test_orbital_graph() {
        let orbital_system = vec![
            Orbit {
                orbiter: "B".to_string(),
                orbitee: "COM".to_string(),
            },
            Orbit {
                orbiter: "C".to_string(),
                orbitee: "B".to_string(),
            },
            Orbit {
                orbiter: "D".to_string(),
                orbitee: "C".to_string(),
            },
            Orbit {
                orbiter: "E".to_string(),
                orbitee: "D".to_string(),
            },
            Orbit {
                orbiter: "F".to_string(),
                orbitee: "E".to_string(),
            },
            Orbit {
                orbiter: "G".to_string(),
                orbitee: "B".to_string(),
            },
            Orbit {
                orbiter: "H".to_string(),
                orbitee: "G".to_string(),
            },
            Orbit {
                orbiter: "I".to_string(),
                orbitee: "D".to_string(),
            },
            Orbit {
                orbiter: "J".to_string(),
                orbitee: "E".to_string(),
            },
            Orbit {
                orbiter: "K".to_string(),
                orbitee: "J".to_string(),
            },
            Orbit {
                orbitee: "K".to_string(),
                orbiter: "L".to_string(),
            },
        ];
        assert_eq!(
            OrbitalGraph::from_vec(orbital_system).dijkstra(&'L'.to_string(), &'I'.to_string()),
            vec!["L", "K", "J", "E", "D", "I"]
        );
    }
    #[test]
    fn test_count_orbits() {
        let orbital_system = vec![
            Orbit {
                orbiter: "B".to_string(),
                orbitee: "COM".to_string(),
            },
            Orbit {
                orbiter: "C".to_string(),
                orbitee: "B".to_string(),
            },
            Orbit {
                orbiter: "D".to_string(),
                orbitee: "C".to_string(),
            },
            Orbit {
                orbiter: "E".to_string(),
                orbitee: "D".to_string(),
            },
            Orbit {
                orbiter: "F".to_string(),
                orbitee: "E".to_string(),
            },
            Orbit {
                orbiter: "G".to_string(),
                orbitee: "B".to_string(),
            },
            Orbit {
                orbiter: "H".to_string(),
                orbitee: "G".to_string(),
            },
            Orbit {
                orbiter: "I".to_string(),
                orbitee: "D".to_string(),
            },
            Orbit {
                orbiter: "J".to_string(),
                orbitee: "E".to_string(),
            },
            Orbit {
                orbiter: "K".to_string(),
                orbitee: "J".to_string(),
            },
            Orbit {
                orbitee: "K".to_string(),
                orbiter: "L".to_string(),
            },
        ];
        assert_eq!(count_orbits(orbital_system), 42);
    }
}
