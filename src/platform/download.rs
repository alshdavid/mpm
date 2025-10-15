use std::fs::File;
use std::io::Write;

use futures_util::StreamExt;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::unbounded_channel;

pub type DownloadProgressReceiver = UnboundedReceiver<anyhow::Result<DownloadProgress>>;

#[allow(dead_code)]
pub struct DownloadProgress {
    pub total: usize,
    pub downloaded: usize,
    pub percent: f64,
}

pub fn download_file(
    url: &str,
    path: &str,
) -> DownloadProgressReceiver {
    let (tx, rx) = unbounded_channel();

    tokio::spawn({
        let url = url.to_string();
        let path = path.to_string();

        async move {
            let _ = 'block: {
                let client = reqwest::Client::new();
                let response = match client.get(url).send().await {
                    Ok(response) => response,
                    Err(err) => break 'block tx.send(Err(anyhow::anyhow!(err))),
                };

                let total_size = response.content_length();

                if let Some(size) = total_size {
                    tx.send(Ok(DownloadProgress {
                        total: size as usize,
                        downloaded: (size as f64 / 1_048_576.0) as usize,
                        percent: 0.0,
                    }))
                    .ok();
                } else {
                    tx.send(Ok(DownloadProgress {
                        total: 0,
                        downloaded: 0,
                        percent: 0.0,
                    }))
                    .ok();
                }

                let mut file = match File::create(path) {
                    Ok(file) => file,
                    Err(err) => break 'block tx.send(Err(anyhow::anyhow!(err))),
                };

                let mut downloaded: u64 = 0;
                let mut stream = response.bytes_stream();

                while let Some(item) = stream.next().await {
                    let chunk = match item {
                        Ok(chunk) => chunk,
                        Err(err) => break 'block tx.send(Err(anyhow::anyhow!(err))),
                    };
                    match file.write_all(&chunk) {
                        Ok(_) => {}
                        Err(err) => break 'block tx.send(Err(anyhow::anyhow!(err))),
                    };

                    downloaded += chunk.len() as u64;

                    if let Some(total) = total_size {
                        let percent = (downloaded as f64 / total as f64) * 100.0;
                        tx.send(Ok(DownloadProgress {
                            total: total as usize,
                            downloaded: downloaded as usize,
                            percent,
                        }))
                        .ok();
                    } else {
                        tx.send(Ok(DownloadProgress {
                            total: 0,
                            downloaded: downloaded as usize,
                            percent: 0.0,
                        }))
                        .ok();
                    }
                }

                Ok(())
            };
        }
    });

    rx
}
