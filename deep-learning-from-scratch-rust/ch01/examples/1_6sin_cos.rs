use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 描画領域の設定
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // チャートの設定
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..6.0, -1.0..1.0)?;

    // 軸とグリッドの描画
    chart.configure_mesh().draw()?;

    // sin関数の描画
    chart.draw_series(LineSeries::new(
        (0..60).map(|x| x as f64 / 10.0).map(|x| (x, x.sin())),
        &RED,
    ))?;
    chart.draw_series(LineSeries::new(
        (0..60).map(|x| x as f64 / 10.0).map(|x| (x, x.cos())),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}
