use crate::immich::models::AlbumInfo;

pub async fn get_album_info() -> Result<String, ()> {
    println!("In albums");
    let settings = crate::settings::settings();
    let api_key = settings.immich.api_key.clone();
    let base_url = settings.immich.server_url.clone();
    let album_id = settings.immich_album.album_id.clone();
    let url = format!("{}/api/albums/{}", base_url, album_id);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("x-api-key", format!("{}", api_key))
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        //println!("Raw Response {}", response.text().await.unwrap());
        let album_info: AlbumInfo = response.json().await.unwrap();
        for asset in album_info.assets.unwrap() {
            println!("{:?}", &asset);
            println!("{:?}", asset.id);
        }
        //        println!("Album info: {:?}", album_info);
        Ok("hi".to_string())
    } else {
        println!("{:?}", response.status());
        println!("Error");
        Err(())
    }
}
