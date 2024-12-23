use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::collections::BTreeSet;
use std::io::BufRead;

pub fn run<R>(reader: R) -> (usize, String)
where
    R: BufRead,
{
    let mut conn_min = HashMap::new();
    let mut all_vertices = BTreeSet::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (from, to) = line.split_once("-").unwrap();
        let from = from.to_string();
        let to = to.to_string();
        all_vertices.insert(from.clone());
        all_vertices.insert(to.clone());
        if from < to {
            conn_min
                .entry(from.clone())
                .or_insert(Vec::new())
                .push(to.clone());
        } else {
            conn_min.entry(to).or_insert(Vec::new()).push(from);
        }
    }

    let mut count_tri_loop = 0;
    for (val, followers) in &conn_min {
        let val_t = val.starts_with("t");
        for follower in followers {
            let foll_t = follower.starts_with("t");
            if let Some(follow_ons) = conn_min.get(follower) {
                for follow_on in follow_ons {
                    let foll_on_t = follow_on.starts_with("t");
                    let any_t = val_t || foll_t || foll_on_t;
                    if any_t && followers.contains(follow_on) {
                        count_tri_loop += 1;
                    }
                }
            }
        }
    }

    let connected = bron_kerbosch(HashSet::new(), all_vertices, HashSet::new(), &conn_min);
    let (_mx_len, group) = connected
        .iter()
        .map(|group| (group.len(), group))
        .max_by(|(g_a, _), (g_b, _)| g_a.cmp(g_b))
        .unwrap();
    let mut max_connected: Vec<&String> = group.iter().collect();
    max_connected.sort();
    let max_group = max_connected.iter().join(",");
    (count_tri_loop, max_group)
}

fn bron_kerbosch(
    r: HashSet<String>,
    mut p: BTreeSet<String>,
    mut x: HashSet<String>,
    neighbors: &HashMap<String, Vec<String>>,
) -> Vec<HashSet<String>> {
    if p.len() == 0 && x.len() == 0 {
        return vec![r];
    }
    let mut found_cliques = Vec::new();
    loop {
        let vertex = p.first();
        if vertex.is_none() {
            break;
        }
        let vertex = vertex.unwrap().clone();

        let mut r_cp = r.clone();
        r_cp.insert(vertex.clone());
        let n_val = Vec::new();
        let vert_neighbors = neighbors.get(&vertex).unwrap_or(&n_val);
        let mut p_cp = BTreeSet::new();
        for vert_neighbor in vert_neighbors {
            if p.contains(vert_neighbor) {
                p_cp.insert(vert_neighbor.clone());
            }
        }
        let mut x_cp = HashSet::new();
        for vert_neighbor in vert_neighbors {
            if x.contains(vert_neighbor) {
                x_cp.insert(vert_neighbor.clone());
            }
        }

        for val in bron_kerbosch(r_cp, p_cp, x_cp, neighbors) {
            found_cliques.push(val);
        }

        p.remove(&vertex);
        x.insert(vertex);
    }
    found_cliques
}
