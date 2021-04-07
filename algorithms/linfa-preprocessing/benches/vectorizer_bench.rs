use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flate2::read::GzDecoder;
use linfa_preprocessing::count_vectorization::CountVectorizer;
use linfa_preprocessing::tf_idf_vectorization::TfIdfVectorizer;
use std::path::Path;
use tar::Archive;

#[tokio::main]
async fn download_20news_bydate() -> Vec<std::path::PathBuf> {
    let file_paths = load_test_filenames();
    if file_paths.is_err() {
        let target = "http://qwone.com/~jason/20Newsgroups/20news-bydate.tar.gz";
        let response = reqwest::get(target).await.unwrap();
        let content = response.bytes().await.unwrap().to_vec();
        let tar = GzDecoder::new(content.as_slice());
        let mut archive = Archive::new(tar);
        archive.unpack(".").unwrap();
        load_test_filenames().unwrap()
    } else {
        file_paths.unwrap()
    }
}

fn load_train_filenames() -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let mut file_paths = Vec::new();
    let path = Path::new("./20news-bydate-train");
    let dir_content = std::fs::read_dir(path)?;
    for sub_dir in dir_content {
        let sub_dir = sub_dir?;
        if sub_dir.file_type().unwrap().is_dir() {
            for sub_file in std::fs::read_dir(sub_dir.path())? {
                let sub_file = sub_file?;
                if sub_file.file_type().unwrap().is_file() {
                    file_paths.push(sub_file.path());
                }
            }
        }
    }
    Ok(file_paths)
}

fn load_test_filenames() -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let mut file_paths = Vec::new();
    let path = Path::new("./20news-bydate-test");
    let dir_content = std::fs::read_dir(path)?;
    for sub_dir in dir_content {
        let sub_dir = sub_dir?;
        if sub_dir.file_type().unwrap().is_dir() {
            for sub_file in std::fs::read_dir(sub_dir.path())? {
                let sub_file = sub_file?;
                if sub_file.file_type().unwrap().is_file() {
                    file_paths.push(sub_file.path());
                }
            }
        }
    }
    Ok(file_paths)
}

fn fit_transform_vectorizer(file_names: &Vec<std::path::PathBuf>) {
    CountVectorizer::default()
        .fit_files(
            file_names,
            encoding::all::ISO_8859_1,
            encoding::DecoderTrap::Strict,
        )
        .unwrap()
        .transform_files(
            file_names,
            encoding::all::ISO_8859_1,
            encoding::DecoderTrap::Strict,
        );
}
fn fit_transform_tf_idf(file_names: &Vec<std::path::PathBuf>) {
    TfIdfVectorizer::default()
        .fit_files(
            file_names,
            encoding::all::ISO_8859_1,
            encoding::DecoderTrap::Strict,
        )
        .unwrap()
        .transform_files(
            file_names,
            encoding::all::ISO_8859_1,
            encoding::DecoderTrap::Strict,
        );
}

fn benchmark_count_vectorizer(c: &mut Criterion) {
    let file_names = download_20news_bydate();
    c.bench_function("count vectorizer", |b| {
        b.iter(|| fit_transform_vectorizer(black_box(&file_names)))
    });
}

fn benchmark_tf_idf(c: &mut Criterion) {
    let file_names = download_20news_bydate();
    c.bench_function("tf_idf", |b| {
        b.iter(|| fit_transform_tf_idf(black_box(&file_names)))
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(10);
    targets = benchmark_count_vectorizer, benchmark_tf_idf
}
criterion_main!(benches);