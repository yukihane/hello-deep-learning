use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// const URL_BASE: &str = "http://yann.lecun.com/exdb/mnist/";
const URL_BASE: &str = "https://ossci-datasets.s3.amazonaws.com/mnist/";

lazy_static! {
    static ref KEY_FILE: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("train_img", "train-images-idx3-ubyte.gz");
        m.insert("train_label", "train-labels-idx1-ubyte.gz");
        m.insert("test_img", "t10k-images-idx3-ubyte.gz");
        m.insert("test_label", "t10k-labels-idx1-ubyte.gz");
        m
    };
}

#[derive(Debug)]
pub struct MnistData {
    pub images: Vec<Vec<f32>>,
    pub labels: Vec<u8>,
}

use anyhow::Result;
use reqwest::Client;

const USER_AGENT: &str =
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:47.0) Gecko/20100101 Firefox/47.0";

pub async fn download(file_name: &str) -> Result<()> {
    let dataset_dir = std::env::current_dir()?.join("dataset");
    let file_path = dataset_dir.join(file_name);

    // Skip if file exists
    if file_path.exists() {
        return Ok(());
    }

    println!("Downloading {} ...", file_name);

    // Create client with custom headers
    let client = Client::builder().user_agent(USER_AGENT).build()?;

    // Download file
    let url = format!("{}{}", URL_BASE, file_name);
    let response = client.get(&url).send().await?.bytes().await?;

    // Ensure directory exists
    fs::create_dir_all(dataset_dir)?;

    // Write file
    fs::write(&file_path, &response)?;

    println!("Done");
    Ok(())
}

pub async fn download_mnist() -> Result<MnistData, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    for (_, filename) in KEY_FILE.iter() {
        let url = format!("{}{}", URL_BASE, filename);
        let path = Path::new("data").join(filename);

        if !path.exists() {
            let bytes = client.get(&url).send().await?.bytes().await?;
            fs::create_dir_all("data")?;
            fs::write(&path, &bytes)?;
        }
    }

    // Load data implementation here
    Ok(MnistData {
        images: Vec::new(),
        labels: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_download_new_file() -> Result<()> {
        // Setup mock server
        let mock_server = MockServer::start().await;

        // Setup mock response
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(vec![1, 2, 3], "application/octet-stream"),
            )
            .mount(&mock_server)
            .await;

        // Test file download
        let test_file = "test_mnist.gz";
        download(test_file).await?;

        // Verify file exists
        let file_path = Path::new("dataset").join(test_file);
        assert!(file_path.exists());

        // Cleanup
        fs::remove_file(file_path)?;
        fs::remove_dir("dataset")?;

        Ok(())
    }

    #[tokio::test]
    async fn test_download_existing_file() -> Result<()> {
        // Create test directory and file
        fs::create_dir_all("dataset")?;
        let test_file = "test_existing.gz";
        let file_path = Path::new("dataset").join(test_file);
        fs::write(&file_path, "test data")?;

        // Test download (should skip)
        download(test_file).await?;

        // Verify file still exists with original content
        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content, "test data");

        // Cleanup
        fs::remove_file(file_path)?;
        fs::remove_dir("dataset")?;

        Ok(())
    }
}
