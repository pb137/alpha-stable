use alpha_stable::AlphaStable;

use rand::thread_rng;

use plotters::{prelude::*, series::Histogram};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let n_samples = 5000;

    plot_histogram(n_samples, "Alpha(alpha: 1.1, beta: 0.0, sigma: 1.0, mu: 0.0)", "Hist_1.png", &AlphaStable::new(1.1, 0.0, 1.0, 0.0)?)?;
    plot_histogram(n_samples, "Alpha(alpha: 1.5, beta: 0.0, sigma: 1.0, mu: 0.0)", "Hist_2.png", &AlphaStable::new(1.5, 0.0, 1.0, 0.0)?)?;
    plot_histogram(n_samples, "Alpha(alpha: 2.0, beta: 0.0, sigma: 1.0, mu: 0.0)", "Hist_3.png", &AlphaStable::new(2.0, 0.0, 1.0, 0.0)?)?;
    plot_histogram(n_samples, "Alpha(alpha: 1.1, beta: 0.5, sigma: 1.0, mu: 0.0)", "Hist_4.png", &AlphaStable::new(1.1, 0.5, 1.0, 0.0)?)?;
    plot_histogram(n_samples, "Alpha(alpha: 1.1, beta: 0.5, sigma: 0.5, mu: 0.7)", "Hist_5.png", &AlphaStable::new(1.1, 0.5, 0.5, 0.7)?)?;
    plot_histogram(n_samples, "Alpha(alpha: 1.1, beta: -0.5, sigma: 1.0, mu: 0.0)", "Hist_6.png", &AlphaStable::new(1.1, -0.5, 1.0, 0.0)?)?;

    Ok(())
}


fn plot_histogram(n_samples: u32, title: &str, filename: &str, alpha_stable: &AlphaStable) -> Result<(), Box<dyn std::error::Error>> {

    let xmini = -200;
    let xmaxi = 200;
    let dx = 0.1;
    let dy = 1.0 / ( n_samples as f64  * dx); 
    let xmin = xmini as f64 * dx;
    let xmax = xmaxi as f64 * dx;
    let ymax = 0.5;
    let mut rng = thread_rng();
    
    let mut samples = Vec::new();
    for _ in 0..n_samples {
        samples.push(alpha_stable.sample(&mut rng));
    }

    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 32).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d(xmin..xmax, 0.0..ymax)?
        .set_secondary_coord((xmin..xmax).step(dx).use_round().into_segmented(), 0.0..ymax);

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let actual = Histogram::vertical(chart.borrow_secondary())
        .style(GREEN.filled())
        .margin(3)
        .data(samples.iter().map(|x| (*x, dy)));

    chart
        .draw_secondary_series(actual)?
        .label("Observed")
        .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], GREEN.filled()));

    let pdf = LineSeries::new(
        (xmini..xmaxi).map(|x| (x as f64 * dx , alpha_stable.pdf(x as f64 * dx ).unwrap_or(0.0))),
        &RED);

    chart
        .draw_series(pdf)?
        .label("PDF")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels().draw()?;

    root.present()?;

    Ok(())
}