use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input.txt");

    let starttime = std::time::Instant::now();

    let answer = process(input);
    println!("Part 1 answer: {:?}", answer);

    let elapsed = starttime.elapsed();
    println!(
        "took {}ms ({}us)  ",
        elapsed.as_millis(),
        elapsed.as_micros()
    );
}

fn process(input: &str) -> Option<usize> {
    let mapping = parse(&input);
    let found_cables = search_single_cable_by_removing_two_of_them(&mapping, 3670111);
    dbg!(&found_cables);
    let mut banned = HashSet::new();
    // Bridge found: (jzj, vkb)
    let a1 = (&String::from("jzj"), &String::from("vkb"));
    let a2 = (&String::from("vkb"), &String::from("jzj"));
    banned.insert(a1);
    banned.insert(a2);
    // Bridge found: (grh, nvh)
    let b1 = (&String::from("grh"), &String::from("nvh"));
    let b2 = (&String::from("nvh"), &String::from("grh"));
    banned.insert(b1);
    banned.insert(b2);
    // Bridge found: (hhx, vrx)
    let c1 = (&String::from("hhx"), &String::from("vrx"));
    let c2 = (&String::from("vrx"), &String::from("hhx"));
    banned.insert(c1);
    banned.insert(c2);
    let mut group = HashSet::new();
    group.insert(mapping.keys().nth(0).unwrap());
    expand_group(&mut group, &mapping, &banned);
    Some(group.len() * (mapping.len() - group.len()))
}

fn search_single_cable_by_removing_two_of_them(
    mapping: &HashMap<String, HashSet<String>>,
    debug_count_skip: usize,
) -> Vec<(&String, &String)> {
    let mut debug_count = 0;
    let mut all_found_cables = Vec::new();
    let mut cables = HashSet::new();
    for (key, list) in mapping {
        for value in list {
            let mut pair = vec![key];
            pair.push(value);
            pair.sort();
            cables.insert((pair[0], pair[1]));
        }
    }
    let mut stop = false;
    let mut cables = cables.into_iter().collect::<Vec<_>>();
    cables.sort();
    for x in 0..cables.len() {
        for y in x..cables.len() {
            debug_count += 1;
            if debug_count < debug_count_skip {
                continue;
            }
            let mut banned = HashSet::new();
            for i in [x, y] {
                let cable = &cables[i];
                banned.insert((cable.0, cable.1));
                banned.insert((cable.1, cable.0));
            }
            let mut visited = HashSet::new();
            let mut disc = HashMap::new();
            let mut low = HashMap::new();
            let mut time = 0;
            let mut found_cables = Vec::new();
            for u in mapping.keys() {
                if visited.contains(u) {
                    continue;
                }
                dfs(
                    &mapping,
                    &banned,
                    u,
                    None,
                    &mut time,
                    &mut visited,
                    &mut disc,
                    &mut low,
                    &mut found_cables,
                );
            }
            all_found_cables.append(&mut found_cables);
            if all_found_cables.len() >= 1 {
                for i in [x, y] {
                    let cable = &cables[i];
                    all_found_cables.push(*cable);
                }
                stop = true;
                break;
            }
        }
        if stop {
            break;
        }
    }
    all_found_cables
}

fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    let mut mapping = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(": ");
        let name = split.next().unwrap();
        for connected in split.next().unwrap().split(' ') {
            let list = mapping.entry(name.to_string()).or_insert(HashSet::new());
            list.insert(connected.to_string());
            let list = mapping
                .entry(connected.to_string())
                .or_insert(HashSet::new());
            list.insert(name.to_string());
        }
    }
    mapping
}

fn dfs<'a>(
    mapping: &'a HashMap<String, HashSet<String>>,
    banned: &HashSet<(&String, &String)>,
    u: &'a String,
    parent: Option<&String>,
    time: &mut usize,
    visited: &mut HashSet<&'a String>,
    disc: &mut HashMap<&'a String, usize>,
    low: &mut HashMap<&'a String, usize>,
    found_cables: &mut Vec<(&'a String, &'a String)>,
) {
    *time += 1;
    disc.insert(u, *time);
    low.insert(u, *time);
    visited.insert(u);
    for v in mapping.get(u).unwrap() {
        if banned.contains(&(v, u)) {
            continue;
        }
        if !visited.contains(v) {
            dfs(
                mapping,
                banned,
                v,
                Some(u),
                time,
                visited,
                disc,
                low,
                found_cables,
            );
            *low.get_mut(&u).unwrap() = (*low.get(&u).unwrap()).min(*low.get(&v).unwrap());
            if *low.get(&v).unwrap() > *disc.get(&u).unwrap() {
                found_cables.push((u, v));
            }
        } else if Some(v) != parent {
            *low.get_mut(&u).unwrap() = (*low.get(&u).unwrap()).min(*disc.get(&v).unwrap());
        }
    }
}

fn expand_group<'a>(
    group: &mut HashSet<&'a String>,
    mapping: &'a HashMap<String, HashSet<String>>,
    banned: &HashSet<(&String, &String)>,
) {
    let mut neighbors = HashSet::new();
    for member in group.iter() {
        for neighbor in mapping.get(*member).unwrap() {
            if group.contains(neighbor) {
                continue;
            }
            if banned.contains(&(member, neighbor)) {
                continue;
            }
            neighbors.insert(neighbor);
        }
    }
    if neighbors.len() > 0 {
        for neighbor in neighbors {
            group.insert(neighbor);
        }
        expand_group(group, mapping, banned);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn part1_process() {
//         let input = "";
//         assert_eq!(400, process(input))
//     }
// }
