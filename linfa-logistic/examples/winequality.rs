use linfa::prelude::*;
use linfa_logistic::{LogisticRegression};

fn main() -> Result<()> {
    // everything above 6.5 is considered a good wine
    let (train, valid) = linfa_datasets::winequality()
        .map_targets(|x| if *x > 6 {/*"good"*/true} else {/*"bad"*/false})
        .split_with_ratio(0.9);

    println!(
        "Fit Logistic Regression classifier with #{} training points",
        train.nsamples()
    );

    // fit a SVM with C value 7 and 0.6 for positive and negative classes
    let model = LogisticRegression::default()
        .fit(&train).unwrap();

    // predict and map targets
    let pred = model.predict(&valid);

    // create a confusion matrix
    let cm = pred.confusion_matrix(&valid).unwrap();

    // Print the confusion matrix, this will print a table with four entries. On the diagonal are
    // the number of true-positive and true-negative predictions, off the diagonal are
    // false-positive and false-negative
    println!("{:?}", cm);

    // Calculate the accuracy and Matthew Correlation Coefficient (cross-correlation between
    // predicted and targets)
    println!("accuracy {}, MCC {}", cm.accuracy(), cm.mcc());

    Ok(())
}
