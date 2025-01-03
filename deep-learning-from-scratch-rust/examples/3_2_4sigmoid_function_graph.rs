use ndarray::prelude::*;
use plotters::prelude::*;

use deep_learning_from_scratch::ch3::ch3_2_4::sigmoid_for_graph;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 描画領域の設定
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // チャートの設定
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-6.0..6.0, -0.1..1.1)?;

    // 軸とグリッドの描画
    chart.configure_mesh().draw()?;

    let x = Array1::range(-5.0, 5.0, 0.1);

    chart.draw_series(LineSeries::new(
        x.iter().map(|&x| (x, sigmoid_for_graph(x))),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}
