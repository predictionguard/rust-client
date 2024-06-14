//! Utility module used to download and base64 encode an image.
use base64;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;

/// Downloads and base64 encodes the image specified by the URL
///
/// ## Arguments
///
/// * `url` - The url of the image to download.
pub async fn encode(url: String) -> crate::Result<String> {
    let img = reqwest::get(url).await?.bytes().await?;

    let mut img_str = String::new();
    BASE64_STANDARD.encode_string(img, &mut img_str);
    Ok(img_str)
}

#[cfg(test)]
mod tests {
    use crate::image;

    #[test]
    fn image_encode() {
        tokio_test::block_on(async {
            let url = "https://farm4.staticflickr.com/3300/3497460990_11dfb95dd1_z.jpg](https://farm4.staticflickr.com/3300/3497460990_11dfb95dd1_z.jpg";

            let encoded_str = image::encode(url.to_string()).await.unwrap();

            assert!(!encoded_str.is_empty());
            println!("Image-> \n{:?}", encoded_str.clone());
        });
    }
}
