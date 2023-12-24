use z3::ast::Ast;

fn main() {
    let input = include_str!("../../input.txt");

    let starttime = std::time::Instant::now();

    let answer = process(input);
    println!("Part 1 answer: {answer}");

    let elapsed = starttime.elapsed();
    println!("took {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hailstone {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
}

fn parse(input: &str) -> Vec<Hailstone> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Hailstone {
    let numbers: Vec<i64> = line
        .split(|c: char| !c.is_digit(10) && c != '-')
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    Hailstone {
        position: (numbers[0], numbers[1], numbers[2]),
        velocity: (numbers[3], numbers[4], numbers[5]),
    }
}

fn process(input: &str) -> i64 {
    let hailstones = parse(input);
    let cfg = z3::Config::new();
    let context = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&context);

    let x = z3::ast::Int::new_const(&context, "x");
    let y = z3::ast::Int::new_const(&context, "y");
    let z = z3::ast::Int::new_const(&context, "z");
    let vx = z3::ast::Int::new_const(&context, "vx");
    let vy = z3::ast::Int::new_const(&context, "vy");
    let vz = z3::ast::Int::new_const(&context, "vz");

    for (i, hs) in hailstones.iter().take(3).enumerate() {
        let a = z3::ast::Int::from_i64(&context, hs.position.0);
        let va = z3::ast::Int::from_i64(&context, hs.velocity.0);
        let b = z3::ast::Int::from_i64(&context, hs.position.1);
        let vb = z3::ast::Int::from_i64(&context, hs.velocity.1);
        let c = z3::ast::Int::from_i64(&context, hs.position.2);
        let vc = z3::ast::Int::from_i64(&context, hs.velocity.2);

        let t = z3::ast::Int::new_const(&context, format!("t{i}"));
        solver.assert(&t.gt(&z3::ast::Int::from_i64(&context, 0)));
        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
    }
    if solver.check() == z3::SatResult::Sat {
        let m = solver.get_model().unwrap();
        return m.eval(&(x + y + z), true).unwrap().as_i64().unwrap();
    }
    panic!("Failed to solve!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part2() {
        assert_eq!(
            process(indoc!(
                "
                    19, 13, 30 @ -2,  1, -2
                    18, 19, 22 @ -1, -1, -2
                    20, 25, 34 @ -2, -2, -4
                    12, 31, 28 @ -1, -2, -1
                    20, 19, 15 @  1, -5, -3
                    "
            )),
            47
        );
    }
}
