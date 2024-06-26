// use error_chain::error_chain;
use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};
use log::{info, trace};
use regex::Regex;
use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;
use std::str::FromStr;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         Reqwest(reqwest::Error);
//         Header(reqwest::header::ToStrError);
//     }
// }

struct PartialRangeIter {
    start: usize,
    end: usize,
    buffer_size: usize,
}

impl PartialRangeIter {
    pub fn new(start: usize, end: usize, buffer_size: usize) -> Result<Self> {
        if buffer_size == 0 {
            Err(eyre!("Invalid buffer size, give a value greater than 0"))?;
        }
        Ok(PartialRangeIter {
            start,
            end,
            buffer_size,
        })
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            let prev_start = self.start;
            self.start += std::cmp::min(self.buffer_size as usize, self.end - self.start + 1);
            Some(
                HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1))
                    .wrap_err_with(|| eyre!("Failed to create header value"))
                    .unwrap(),
            )
        }
    }
}

pub async fn retrieve_file(
    list: &crate::repo::Wordlist,
    decompress: bool,
    base_dir: &str,
    user_agent: &str,
    worker_count: usize,
) -> Result<()> {
    assert!(
        !std::path::Path::new(&format!("/{}/{}", base_dir, list.get_name()))
            .try_exists()
            .expect("File already exists")
    );
    log::debug!("{}", list.get_url());
    let chunk_size = crate::units::convert_size(
        list.get_size().ceil() as u64,
        crate::units::get_unit(list.get_unit()),
    )
    .await
        / worker_count as u64;
    dbg!(chunk_size);

    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .gzip(decompress)
        .brotli(decompress)
        .deflate(decompress)
        .build()
        .wrap_err_with(|| eyre!("Failed to create reqwest client"))?;
    let response = client.head(list.get_url()).send().await?;

    let length = response.headers().get(CONTENT_LENGTH);

    if !std::path::Path::new(&base_dir).is_dir() {
        std::fs::create_dir_all(&base_dir)?;
    }

    let mut output_file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("/{}/{}", base_dir, list.get_name()))
        .wrap_err_with(|| eyre!("Failed to create file"))?;
    info!("Starting download of {}", list.get_name());
    // fix types for length
    let length = if let Some(length) = length {
        u64::from_str(length.to_str()?)?
    } else {
        crate::units::convert_size(
            list.get_size().ceil() as u64,
            crate::units::get_unit(list.get_unit()),
        )
        .await
    };

    for range in PartialRangeIter::new(0, (length - 1).try_into().unwrap(), chunk_size as usize)? {
        trace!("Downloading range: {:?}", range);
        let res = client
            .get(list.get_url())
            .header(RANGE, range)
            .send()
            .await?;

        let status = res.status();
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            return Err(eyre!(
                "Failed to download file-- server respnse: {}",
                status
            ));
        }
        let bytes = res.bytes().await?;
        std::io::copy(&mut bytes.as_ref(), &mut output_file)?;
    }

    if !decompress {
        info!("Finished with success");
        return Ok(());
    }

    let re = Regex::new(r"\.tar\.gz$").unwrap();
    let gz_re = Regex::new(r"\.gz$").unwrap();

    if re.is_match(list.get_url()) {
        info!("Decompressing file");
        let tar_gz = std::fs::File::open(format!("/{}/{}", base_dir, list.get_name()))?;
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);
        archive.unpack(format!("/{}", base_dir))?;
        std::fs::remove_file(format!("/{}/{}", base_dir, list.get_name()))?;
    } else if gz_re.is_match(list.get_url()) {
        info!("Decompressing file");
        let gz = std::io::BufReader::new(std::fs::File::open(format!(
            "/{}/{}",
            base_dir,
            list.get_name()
        ))?);

        let mut decoder = flate2::bufread::GzDecoder::new(gz);
        let mut buffer = Vec::new();
        std::io::copy(&mut decoder, &mut buffer)?;
        std::fs::write(format!("/{}/{}", base_dir, list.get_name()), &buffer)?;
        std::fs::remove_file(format!("/{}/{}", base_dir, list.get_name()))?;
    }

    // std::fs::remove_file(format!("/{}/{}", base_dir, list.get_name()))?;

    let _content = response.text().await?;

    info!("Finished with success");
    Ok(())
}
