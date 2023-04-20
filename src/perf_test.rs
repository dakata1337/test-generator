use std::time::Duration;

use crate::{data::Project, pdf_gen};

fn print_stats(times: &mut Vec<Duration>) {
    times.sort();
    let median = if times.len() % 2 == 0 {
        let a = times[times.len() / 2 - 1];
        let b = times[times.len() / 2];
        a + b
    } else {
        times[times.len() / 2 - 1]
    };

    println!("Median: {:?}", median);
    println!(
        "Average: {:?}",
        times.iter().sum::<Duration>().div_f64(times.len() as f64)
    );
}

fn test_pdf_generation(project: &Project) -> anyhow::Result<()> {
    const TEST_ITER: u16 = 1_000;
    let mut times = vec![];

    println!("Testing PDF generation {} times", TEST_ITER);
    for _ in 0..TEST_ITER {
        times.push(pdf_gen::generate_pdf(&project)?);
    }

    print_stats(&mut times);
    Ok(())
}

pub fn test(project: Project) -> anyhow::Result<()> {
    test_pdf_generation(&project)?;

    Ok(())
}
