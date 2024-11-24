use mathcraft_rs::arithmetic::average::AverageProblem;

fn main() {
    let problema = AverageProblem::new(3, 90.0);
    println!("Total: {}", problema.calculate_total());
}
