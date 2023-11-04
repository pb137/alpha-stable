use alpha_stable::AlphaStable;

use plotters::{prelude::*, style::full_palette::ORANGE};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    standard()?;
    normalized()?;
    Ok(())
}

fn standard() -> Result<(), Box<dyn std::error::Error>> {

    let dist_1 = AlphaStable::new(0.50, 0.5, 1.0, 0.0)?;
    let dist_2 = AlphaStable::new(0.75, 0.5, 1.0, 0.0)?;
    let dist_3 = AlphaStable::new(1.00, 0.5, 1.0, 0.0)?;
    let dist_4 = AlphaStable::new(1.25, 0.5, 1.0, 0.0)?;
    let dist_5 = AlphaStable::new(1.50, 0.5, 1.0, 0.0)?;
    let dx = 0.01;
    let xmini = -450;
    let xmaxi =  450;
    let xmin = xmini as f64 * dx;  
    let xmax = xmaxi as f64 * dx;  
    let ymin = 0.0;
    let ymax = 0.6;

    let root = BitMapBackend::new("Distributions_1.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("S(alpha, 0.5, 1, 0)", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(xmin..xmax, ymin..ymax)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (xmini..xmaxi).map(|x| (x as f64 * dx , dist_1.pdf(x as f64 * dx).unwrap_or(0.0))),
            &BLACK,
        ))?
        .label("alpha=0.5")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    chart
        .draw_series(LineSeries::new(
            (xmini..xmaxi).map(|x| (x as f64 * dx , dist_2.pdf(x as f64 * dx).unwrap_or(0.0))),
            &RED,
        ))?
        .label("alpha=1.00")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));


    chart
        .draw_series(LineSeries::new(
            (xmini..xmaxi).map(|x| (x as f64 * dx , dist_3.pdf(x as f64 * dx).unwrap_or(0.0))),
            &BLUE,
        ))?
        .label("alpha=1.00")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));


    chart
        .draw_series(LineSeries::new(
            (xmini..xmaxi).map(|x| (x as f64 * dx , dist_4.pdf(x as f64 * dx).unwrap_or(0.0))),
            &GREEN,
        ))?
        .label("alpha=1.25")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(LineSeries::new(
            (xmini..xmaxi).map(|x| (x as f64 * dx , dist_5.pdf(x as f64 * dx).unwrap_or(0.0))),
            &ORANGE,
        ))?
        .label("alpha=1.50")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &ORANGE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn normalized() -> Result<(), Box<dyn std::error::Error>> {

    let dist_1 = AlphaStable::new_S0(0.50, 0.5, 1.0, 0.0)?;
    let dist_2 = AlphaStable::new_S0(0.75, 0.5, 1.0, 0.0)?;
    let dist_3 = AlphaStable::new_S0(1.00, 0.5, 1.0, 0.0)?;
    let dist_4 = AlphaStable::new_S0(1.25, 0.5, 1.0, 0.0)?;
    let dist_5 = AlphaStable::new_S0(1.50, 0.5, 1.0, 0.0)?;
    let dx = 0.01;
    let xmini = -450;
    let xmaxi =  450;
    let xmin = xmini as f64 * dx;  
    let xmax = xmaxi as f64 * dx;  
    let ymin = 0.0;
    let ymax = 0.6;

    let root = BitMapBackend::new("Distributions_2.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("S^0(alpha, 0.5, 1, 0)", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(xmin..xmax, ymin..ymax)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (xmini..xmaxi).map(|x| (x as f64 * dx , dist_1.pdf(x as f64 * dx).unwrap_or(0.0))),
            &BLACK,
        ))?
        .label("alpha=0.5")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    chart
        .draw_series(LineSeries::new(
            (xmini..xmaxi).map(|x| (x as f64 * dx , dist_2.pdf(x as f64 * dx).unwrap_or(0.0))),
            &RED,
        ))?
        .label("alpha=1.00")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));


    chart
        .draw_series(LineSeries::new(
            (xmini..xmaxi).map(|x| (x as f64 * dx , dist_3.pdf(x as f64 * dx).unwrap_or(0.0))),
            &BLUE,
        ))?
        .label("alpha=1.00")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));


    chart
        .draw_series(LineSeries::new(
            (xmini..xmaxi).map(|x| (x as f64 * dx , dist_4.pdf(x as f64 * dx).unwrap_or(0.0))),
            &GREEN,
        ))?
        .label("alpha=1.25")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .draw_series(LineSeries::new(
            (xmini..xmaxi).map(|x| (x as f64 * dx , dist_5.pdf(x as f64 * dx).unwrap_or(0.0))),
            &ORANGE,
        ))?
        .label("alpha=1.50")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &ORANGE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}