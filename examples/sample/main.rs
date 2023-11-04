use alpha_stable::AlphaStable;

use rand::thread_rng;

use plotters::{prelude::*, style::full_palette::ORANGE};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let n_sets = 10;
    let n_samples = 1000;

    plot_sample_sets(n_sets, n_samples, "Alpha(alpha: 1.1, beta: 0.0, sigma: 1.0, mu: 0.0)", "Samples_1.png", &AlphaStable::new(1.1, 0., 1.0, 0.0)?)?;
    plot_sample_sets(n_sets, n_samples, "Alpha(alpha: 1.5, beta: 0.0, sigma: 1.0, mu: 0.0)", "Samples_2.png", &AlphaStable::new(1.5, 0., 1.0, 0.0)?)?;
    plot_sample_sets(n_sets, n_samples, "Alpha(alpha: 2.0, beta: 0.0, sigma: 1.0, mu: 0.0)", "Samples_3.png", &AlphaStable::new(2.0, 0., 1.0, 0.0)?)?;

    Ok(())
}

fn plot_sample_sets(n_sets: u32, n_samples: u32, title: &str, filename: &str, alpha_stable: &AlphaStable) -> Result<(), Box<dyn std::error::Error>> {

    let mut rng = thread_rng();
    let mut sample_sets = Vec::new();
    let mut ymin = Vec::new();
    let mut ymax = Vec::new();
    let colors = vec![&BLACK, &RED, &BLUE, &GREEN, &ORANGE];

    for _ in 0..n_sets {
        let mut sum = 0.0;
        let mut samples = Vec::new();
        for _ in 0..n_samples {
            samples.push(sum);
            sum += alpha_stable.sample(&mut rng);
        }
        let yminf = *samples.iter().min_by(|a, b| a.total_cmp(&b)).unwrap();
        let ymaxf = *samples.iter().max_by(|a, b| a.total_cmp(&b)).unwrap();

        sample_sets.push(samples);
        ymin.push(yminf); 
        ymax.push(ymaxf); 
    }

    let xminf = 0.0;  
    let xmaxf = n_samples as f64;  
    let yminf = *ymin.iter().min_by(|a, b| a.total_cmp(&b)).unwrap();
    let ymaxf = *ymax.iter().max_by(|a, b| a.total_cmp(&b)).unwrap();

    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif",32).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d(xminf..xmaxf, yminf..ymaxf)?;

    chart.configure_mesh().draw()?;

    for (i, samples) in sample_sets.iter().enumerate() {
        let color = i % colors.len();
        chart.draw_series(LineSeries::new(
            samples.iter().enumerate().map(|(x, y)| { (x as f64, *y)}),
            colors[color],
        ))?;
    }

    root.present()?;

    Ok(())
}