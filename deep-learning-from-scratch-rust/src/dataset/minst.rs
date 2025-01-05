use anyhow::Result;
use enum_map::Enum;
use enum_map::{enum_map, EnumMap};
use flate2::read::GzDecoder;
use lazy_static::lazy_static;
use ndarray::Array2;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

// const URL_BASE: &str = "http://yann.lecun.com/exdb/mnist/";
const URL_BASE: &str = "https://ossci-datasets.s3.amazonaws.com/mnist/";

#[derive(Debug, Copy, Clone, Enum)]
enum Key {
    TrainImg,
    TrainLabel,
    TestImg,
    TestLabel,
}

static KEY_FILES: Lazy<EnumMap<Key, &'static str>> = Lazy::new(|| {
    enum_map! {
        Key::TrainImg => "train-images-idx3-ubyte.gz",
        Key::TrainLabel => "train-labels-idx1-ubyte.gz",
        Key::TestImg => "t10k-images-idx3-ubyte.gz",
        Key::TestLabel => "t10k-labels-idx1-ubyte.gz",
    }
});

static DATASET_DIR: Lazy<PathBuf> = Lazy::new(|| std::env::current_dir().unwrap().join("dataset"));

const TRAIN_NUM: usize = 60_000;
const TEST_NUM: usize = 10_000;
const IMG_DIM: (usize, usize, usize) = (1, 28, 28);
const IMG_SIZE: usize = 784;

const USER_AGENT: &str =
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:47.0) Gecko/20100101 Firefox/47.0";

pub async fn download_mnist(dataset_dir: &Path) -> Result<()> {
    let client = Client::builder().user_agent(USER_AGENT).build()?;

    for (_, file) in KEY_FILES.iter() {
        let path = dataset_dir.join(file);
        if path.exists() {
            continue;
        }
        println!("Downloading {} ...", file);
        let response = client
            .get(&format!("{}{}", URL_BASE, file))
            .send()
            .await?
            .bytes()
            .await?;
        std::fs::write(&path, &response)?;
        println!("Done");
    }
    Ok(())
}

pub fn load_mnist_images(path: &Path) -> Result<Array2<f32>> {
    let file = File::open(path)?;
    let mut decoder = GzDecoder::new(file);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;

    let data = Array2::from_shape_vec(
        (buffer[8..].len() / IMG_SIZE, IMG_SIZE),
        buffer[16..].iter().map(|&x| x as f32 / 255.0).collect(),
    )?;
    Ok(data)
}

pub fn load_mnist_labels(path: &Path) -> Result<Array2<f32>> {
    let file = File::open(path)?;
    let mut decoder = GzDecoder::new(file);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;

    let labels = Array2::from_shape_vec(
        (buffer[8..].len(), 1),
        buffer[8..].iter().map(|&x| x as f32).collect(),
    )?;
    Ok(labels)
}

pub async fn init_mnist() -> Result<()> {
    std::fs::create_dir_all(DATASET_DIR.as_path())?;

    download_mnist(&DATASET_DIR).await?;

    let train_images = load_mnist_images(&DATASET_DIR.join("train-images-idx3-ubyte.gz"))?;
    let train_labels = load_mnist_labels(&DATASET_DIR.join("train-labels-idx1-ubyte.gz"))?;

    println!("Loaded {} training images", train_images.shape()[0]);
    Ok(())
}
