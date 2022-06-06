use crate::component;

use std::io::Write;

pub struct Downloader {
    progress: component::Progress,
}

impl Downloader {
    pub fn new() -> Self {
        Downloader {
            progress: component::Progress::new(),
        }
    }

    pub async fn download(&mut self, url: &str, output_file_path: &str) {
        let url = url.to_string();
        let output_file_result = std::fs::File::create(output_file_path);

        let output_file =match output_file_result {
            Ok(file) => file,
            Err(_) => return,
        };

        let mut writer = std::io::BufWriter::new(output_file);

        self.progress.draw(|cell| async move {
            let mut res = reqwest::get(url).await.unwrap();
            let mut downloaded = 0u64;
            let size = res.content_length().unwrap();
            let size_mb = (size as f32 * 0.000001) as usize;
        
            let start_time = std::time::SystemTime::now();
        
            let mut mb_s = 0.0;
            let mut mb_s_tmp = 0.0;
            let mut mb_s_time = 0;

            while let Ok(chunk) = res.chunk().await {
                if let Some(bytes) = chunk {
                    writer.write_all(&bytes).unwrap();

                    let time_secs = start_time.elapsed().unwrap().as_secs();
        
                    if mb_s_time != time_secs {
                        writer.flush().unwrap();
                        mb_s_time = time_secs;
                        mb_s = mb_s_tmp;
                        mb_s_tmp = bytes.len() as f32 * 0.000001;
                    } else {
                        mb_s_tmp += bytes.len() as f32 * 0.000001;
                    }
        
                    downloaded += bytes.len() as u64;
                    let prog_f = downloaded as f64 / size as f64;
        
                    {
                        let mut prog = cell.borrow_mut();
                        if mb_s < 1.0 {
                            let speed = mb_s.to_string();
                            let speed = if speed.len() >= 5 { &speed[..5] } else { &speed };
                            (*prog).message = format!("  {}KB/s {}/{} MB", speed, (downloaded as f32 * 0.000001) as usize, size_mb);
                        } else {
                            (*prog).message = format!("  {}MB/s {}/{} MB", mb_s as usize, (downloaded as f32 * 0.000001) as usize, size_mb);
                        }
                        (*prog).progress = prog_f as f32;
                    }
        
                    tokio::task::yield_now().await;
                } else {
                    cell.borrow_mut().message = format!("  0MB/s {}/{} MB", (downloaded as f32 * 0.000001) as usize, size_mb);
                    break;
                }
            }
        }).await;
    }
}