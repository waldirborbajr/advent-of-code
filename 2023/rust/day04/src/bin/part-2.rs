use day04::process_part2;

fn main() {
    let file = include_str!("../../input01.txt");

    let starttime = std::time::Instant::now();

    let output = process_part2(file);
    dbg!(output);

    let elapsed = starttime.elapsed();
    println!("took {}ms ({}us)", elapsed.as_millis(), elapsed.as_micros());
}
