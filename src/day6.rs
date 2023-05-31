use std::collections::{HashMap, HashSet};
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
    use crate::day6::{count_orbits, Orbit};

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
