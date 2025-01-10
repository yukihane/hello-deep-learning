use anyhow::Result;
use flate2::read::GzDecoder;
use ndarray::{Array1, Array2, Array4};
use reqwest;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

const URL_BASE: &str = "https://ossci-datasets.s3.amazonaws.com/mnist/";
const TRAIN_IMAGES: &str = "train-images-idx3-ubyte.gz";
const TRAIN_LABELS: &str = "train-labels-idx1-ubyte.gz";
const TEST_IMAGES: &str = "t10k-images-idx3-ubyte.gz";
const TEST_LABELS: &str = "t10k-labels-idx1-ubyte.gz";

const TRAIN_NUM: usize = 60000;
const TEST_NUM: usize = 10000;
const IMG_SIZE: usize = 784;
const IMG_DIMS: (usize, usize) = (28, 28);

fn get_cache_dir() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("data");
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

async fn download_file(filename: &str) -> Result<()> {
    let path = get_cache_dir().join(filename);
    if path.exists() {
        return Ok(());
    }

    println!("Downloading {}...", filename);
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}{}", URL_BASE, filename))
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?
        .bytes()
        .await?;

    let mut file = File::create(&path)?;
    file.write_all(&response)?;
    println!("Done");
    Ok(())
}

async fn download_mnist() -> Result<()> {
    for file in &[TRAIN_IMAGES, TRAIN_LABELS, TEST_IMAGES, TEST_LABELS] {
        download_file(file).await?;
    }
    Ok(())
}

fn load_images(filename: &str) -> Result<Array2<f32>> {
    println!("Converting {} to Array...", filename);
    let path = get_cache_dir().join(filename);
    let file = File::open(path)?;
    let mut gz = GzDecoder::new(file);
    let mut contents = Vec::new();
    gz.read_to_end(&mut contents)?;

    // Skip header (16 bytes)
    let data = &contents[16..];
    let mut array = Array2::zeros((
        if filename.contains("train") {
            TRAIN_NUM
        } else {
            TEST_NUM
        },
        IMG_SIZE,
    ));

    for (i, chunk) in data.chunks(IMG_SIZE).enumerate() {
        for (j, &byte) in chunk.iter().enumerate() {
            array[[i, j]] = byte as f32;
        }
    }

    println!("Done");
    Ok(array)
}

fn load_labels(filename: &str) -> Result<Array1<u8>> {
    println!("Converting {} to Array...", filename);
    let path = get_cache_dir().join(filename);
    let file = File::open(path)?;
    let mut gz = GzDecoder::new(file);
    let mut contents = Vec::new();
    gz.read_to_end(&mut contents)?;

    // Skip header (8 bytes)
    let data = &contents[8..];
    let array = Array1::from_vec(data.to_vec());

    println!("Done");
    Ok(array)
}

pub struct MnistDataset {
    train_images: Array2<f32>,
    train_images_not_flatten: Array4<f32>,
    train_labels: Array1<u8>,
    test_images: Array2<f32>,
    test_images_not_flatten: Array4<f32>,
    test_labels: Array1<u8>,
}

impl MnistDataset {
    pub async fn new(normalize: bool, flatten: bool) -> Result<Self> {
        download_mnist().await?;

        let mut train_images = load_images(TRAIN_IMAGES)?;
        let mut test_images = load_images(TEST_IMAGES)?;
        let train_labels = load_labels(TRAIN_LABELS)?;
        let test_labels = load_labels(TEST_LABELS)?;

        if normalize {
            train_images.mapv_inplace(|x| x / 255.0);
            test_images.mapv_inplace(|x| x / 255.0);
        }

        let train_images_not_flatten = train_images
            .clone()
            .into_shape((TRAIN_NUM, 1, IMG_DIMS.0, IMG_DIMS.1))?;
        let test_images_not_flatten = test_images
            .clone()
            .into_shape((TEST_NUM, 1, IMG_DIMS.0, IMG_DIMS.1))?;

        Ok(Self {
            train_images,
            train_images_not_flatten,
            train_labels,
            test_images,
            test_images_not_flatten,
            test_labels,
        })
    }

    pub fn to_one_hot(&self) -> (Array2<f32>, Array2<f32>) {
        let train_one_hot = self.labels_to_one_hot(&self.train_labels);
        let test_one_hot = self.labels_to_one_hot(&self.test_labels);
        (train_one_hot, test_one_hot)
    }

    fn labels_to_one_hot(&self, labels: &Array1<u8>) -> Array2<f32> {
        let mut one_hot = Array2::zeros((labels.len(), 10));
        for (i, &label) in labels.iter().enumerate() {
            one_hot[[i, label as usize]] = 1.0;
        }
        one_hot
    }
}
