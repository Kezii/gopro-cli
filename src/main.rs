use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info, warn};
use openapi::apis::configuration::Configuration;
use reqwest::Client;
use reqwest::Error;
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::timeout;

use clap::Parser;
use std::path::PathBuf;

mod bluetooth;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// folder to download files to
    #[arg(short, long, value_name = "PATH")]
    download_path: PathBuf,

    /// gopro host
    #[arg(
        short,
        long,
        value_name = "HOST",
        default_value = "http://10.5.5.9:8080"
    )]
    gopro_host: String,

    /// dry run (do not download files)
    #[arg(long)]
    dry_run: bool,

    /// starts the camera's AP with bluetooth
    #[arg(short, long)]
    bluetooth: bool,
}

async fn check_if_camera_online(configuration: &Configuration) -> bool{
    let ka = openapi::apis::default_api::keep_alive(configuration);

    (timeout(Duration::from_secs(1), ka).await).is_ok()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("gopro_cli=trace"))
        .format_timestamp(None)
        .init();

    let cli = Cli::parse();

    if !cli.download_path.exists() {
        panic!("Download path does not exist");
    }

    if !cli.download_path.is_dir() {
        panic!("Download path is not a directory");
    }

    let configuration = Configuration {
        base_path: cli.gopro_host.clone(),
        ..Default::default()
    };


    if cli.bluetooth {
        if check_if_camera_online(&configuration).await {
            warn!("Camera is already online, skipping bluetooth setup");
        } else {
            bluetooth::start_ap().await.unwrap();
        }
    }



    for i in 0..10 {

        if ! check_if_camera_online(&configuration).await {
            warn!("Can't connect to the camera, retrying in 5 seconds");
            tokio::time::sleep(Duration::from_secs(5)).await;
        } else {
            break;
        }

        if i == 9 {
            panic!("Can't connect to the camera, check that is on and that you are connected to its wifi network");
        }
    }

    print_camera_status(&configuration).await;

    let response = openapi::apis::default_api::get_media_list(&configuration)
        .await
        .unwrap();

    let media_list = response.media.unwrap();

    let mut total_size: usize = 0;
    let mut file_paths = Vec::new();
    for directory in &media_list {
        info!("Directory: {:?}", directory.d);
        info!("Files: ");
        for file in &directory.fs {
            let path = format!("{}/videos/DCIM/{}/{}", &cli.gopro_host, directory.d, file.n);
            let size = i32::from_str(file.s.as_str()).unwrap();
            info!("└ {} - {:4} MB", file.n, size / 1024 / 1024,);
            total_size += size as usize;
            file_paths.push((path, size, file.n.clone()));
        }
    }

    info!("Total size: {} GB", total_size / 1024 / 1024 / 1024);

    let client = Client::new();

    let mut counter = 0;
    let mut downloaded_size: usize = 0;
    for file in &file_paths {
        counter += 1;
        downloaded_size += file.1 as usize;
        let path = &file.0;

        let path_in_disk = cli.download_path.join(&file.2);

        if std::path::Path::new(&path_in_disk).exists() {
            let metadata = std::fs::metadata(&path_in_disk).unwrap();
            if metadata.len() == file.1 as u64 {
                warn!("File already exists: {}", file.0);
                continue;
            }
        }

        if cli.dry_run {
            warn!("Dry run: {}, skipping", path);
            continue;
        }

        let success = download_file(
            &client,
            path,
            &path_in_disk,
            &format!("{}/{}", counter, file_paths.len()),
            &format!("{:5} MB", downloaded_size / 1024 / 1024),
        )
        .await;
        if let Err(e) = success {
            error!("Failed to download file: {}", path);
            error!("Error: {}", e);
        }
    }

    Ok(())
}

async fn print_camera_status(configuration: &Configuration) {
    let status = openapi::apis::default_api::get_camera_state(configuration)
        .await
        .unwrap();

    info!("battery present: {}", status.status.param_1);
    info!("battery level: {}%", status.status.param_70);
}

pub async fn download_file(
    client: &Client,
    url: &str,
    path: &PathBuf,
    message: &str,
    end_message: &str,
) -> Result<(), String> {
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(message);

    // download chunks
    let mut file = File::create(path).or(Err(format!("Failed to create file '{:?}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file".to_string()))?;
        file.write_all(&chunk)
            .or(Err("Error while writing to file".to_string()))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(end_message);
    Ok(())
}
