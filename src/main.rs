use plotly::traces::scatter::GroupNorm;
use plotly::{Plot, Scatter};

fn main() {
    // Define data points
    let x_values = vec![1, 2, 3, 4, 5];
    let y_values = vec![10, 20, 30, 40, 50];

    // Create a scatter plot
    let trace = Scatter::new(x_values, y_values).group_norm(GroupNorm::Percent);

    // Create a plot and add the trace
    let mut plot = Plot::new();
    plot.add_trace(trace);

    // Show the plot in the default web browser
    plot.show();
}
