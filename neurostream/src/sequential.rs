use anyhow::Result;
use neuroflow::{
    FeedForward,
    data::DataSet,
    activators::Type::Sigmoid
};

/// Create, train and evaluate the neural network.
///
/// * `app` - the name of the application which will be used for benchmark.
/// Training data will be loaded from inputs/app.data
/// and testing data, from inputs/app.test.
///
pub fn run(
    app: &str, architecture: &[i32],
    learning_rate: f64, momentum: f64, iterations: i64
) -> Result<()> {
    let mut nn = FeedForward::new(architecture);

    let data = DataSet::from_csv(
        format!("inputs/{}.data", app).as_str()
    ).unwrap();

    nn.activation(Sigmoid)
        .learning_rate(learning_rate)
        .momentum(momentum)
        .train(&data, iterations);

    Ok(())
}

