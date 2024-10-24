use std::collections::{VecDeque, HashMap};

use super::{map::{is_in_map, is_wall, MapObject, MAP}, pos::{MapPos, Pos}};

pub static WAYS: [(i32, i32); 8] = [
    (-1, 0), (0, -1), (1, 0), (0, 1),
    (-1, -1), (1, -1), (1, 1), (-1, 1)
];

#[derive(Clone)]
pub struct PathFinding {
    // TODO: ejercicio: inyectar el grafo en el constructor
    graph: HashMap<MapPos, Vec<MapPos>>,
}

impl PathFinding {
    pub fn new() -> Self {
        Self { 
            graph: get_graph(),
        }
    }

    pub fn get_path(&self, start: MapPos, goal: MapPos, npc_positions: Vec<MapPos>) -> Option<MapPos> {
        let visited = self.bfs(start, goal, npc_positions);
        let mut path = vec![goal];
        let mut step = visited.get(&goal).copied().unwrap_or(start);

        while step != start {
            path.push(step);
            step = visited.get(&step).copied().unwrap_or(start);
        }

        path.pop() // Return the last point in the path
    }

    fn bfs(&self, start: MapPos, goal: MapPos, npc_positions: Vec<MapPos>) -> HashMap<MapPos, MapPos> {
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();

        queue.push_back(start);
        visited.insert(start, start);

        while let Some(cur_node) = queue.pop_front() {
            if cur_node == goal {
                break;
            }

            if let Some(next_nodes) = self.graph.get(&cur_node) {
                for &next_node in next_nodes {
                    if !visited.contains_key(&next_node)
                        && !npc_positions.contains(&next_node) 
                    {
                        queue.push_back(next_node);
                        visited.insert(next_node, cur_node);
                    }
                }
            }
        }

        visited
    }
}

fn get_next_nodes(x: i32, y: i32) -> Vec<(i32, i32)> {
    WAYS.iter()
        .map(|(dx, dy)| (x + *dx as i32, y + *dy as i32))
        .filter(|&(nx, ny)| is_in_map(nx, ny) && !is_wall(Pos::new(nx as f32, ny as f32))) // Assuming world_map contains coordinates that are blocked
        .collect()
}

fn get_graph() -> HashMap<MapPos, Vec<MapPos>> {
    let mut graph = HashMap::new();

    for (y, row) in MAP.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            if col == MapObject::F {
                let next_nodes = get_next_nodes(x as i32, y as i32);
                graph.entry(MapPos::new(x, y))
                    .or_insert(Vec::new())
                    .extend(next_nodes.into_iter().map(|(x, y)| MapPos::new(x as usize, y as usize)));
            }
        }
    }

    graph
}