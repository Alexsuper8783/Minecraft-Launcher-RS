use super::types::version_manifest::{VersionManifest, Version, VersionType};
use serde_json::{from_str, Value};
use cached::proc_macro::{once, cached};

#[once]
pub async fn fetch_versions() -> Vec<Version> {
    let mut versions: Vec<Version> = Vec::new();
    let json: Value = from_str(
        &*reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json").await.expect("Request failed").text().await.expect("Get text failed")
    ).expect("Parse as json failed");
    
    for json_version in json.get("versions").expect("Failed to get field versions").as_array().expect("Failed to cast") {
        versions.push(Version::new(
            json_version.get("id").expect("Failed to get field id").as_str().expect("Failed to cast").to_string(),
            VersionType::new(json_version.get("type").expect("Failed to get field type").as_str().expect("Failed to cast").to_string()),
            json_version.get("url").expect("Failed to get field url").as_str().expect("Failed to cast").to_string(),
            json_version.get("releaseTime").expect("Failed to get field release_time").as_str().expect("Failed to cast").to_string()
        ));
    }
    
    versions
}

#[cached]
pub async fn find_version(id: String) -> Option<Version> {
    let versions = fetch_versions().await;
    
    for version in versions {
        if version.id == id {
            return Some(version)
        }
    }
    None
}

#[once]
pub async fn fetch_version_manifest() -> VersionManifest {
    let versions = fetch_versions().await;
    let json: Value = from_str(
        &*reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json").await.expect("Request failed").text().await.expect("Get text failed")
    ).expect("Parse as json failed");
    
    let latest_release = find_version(
        json.get("latest").expect("Failed to get field latest").get("release").expect("Failed to get field release").as_str().expect("Failed to cast").to_string()
    ).await.expect("Failed to get version");
    
    let latest_snapshot = find_version(
        json.get("latest").expect("Failed to get field latest").get("snapshot").expect("Failed to get field snapshot").as_str().expect("Failed to cast").to_string()
    ).await.expect("Failed to get version");
    
    return VersionManifest::new(latest_release, latest_snapshot, versions);
}