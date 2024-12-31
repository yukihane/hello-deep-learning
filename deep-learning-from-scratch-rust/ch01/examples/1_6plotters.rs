use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 描画領域の設定
    let root = BitMapBackend::new("sine.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // チャートの設定
    let mut chart = ChartBuilder::on(&root)
        .caption("y = sin(x)", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-3.14..3.14, -1.2..1.2)?;

    // 軸とグリッドの描画
    chart.configure_mesh().draw()?;

    // sin関数の描画
    chart.draw_series(LineSeries::new(
        (-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}
