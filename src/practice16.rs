use std::collections::HashSet;

fn solve() -> Vec<(u32, u32, u32)> {
    let mut solutions = Vec::new();

    // Перебираємо всі можливі значення для кожної літери
    for m in 1..=8 {
        for u in 1..=8 {
            if u == m { continue; }
            for x in 1..=8 {
                if x == m || x == u { continue; }
                for a in 1..=8 {
                    if a == m || a == u || a == x { continue; }
                    for s in 1..=8 {
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 1..=8 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 1..=8 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 1..=8 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }

                                    // Формуємо числа "muxa" і "slon"
                                    let muxa = 1000 * m + 100 * u + 10 * x + a;
                                    let slon = 1000 * s + 100 * l + 10 * o + n;

                                    // Перевіряємо рівність
                                    if muxa * a == slon {
                                        solutions.push((muxa, a, slon));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    solutions
}
#[test]
fn main() {
    let solutions = solve();
    println!("Знайдено рішень: {}", solutions.len());

    for (muxa, a, slon) in solutions {
        println!("{:>4} x {} = {}", muxa, a, slon);
    }
}
