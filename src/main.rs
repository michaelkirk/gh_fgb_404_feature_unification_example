use flatgeobuf::{FeatureProperties, HttpFgbReader};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let reader = HttpFgbReader::open("https://raw.githubusercontent.com/flatgeobuf/flatgeobuf/master/test/data/countries.fgb").await.unwrap();
    let mut features = reader.select_all().await.unwrap();

    let mut longest_name = String::new();
    while let Some(feature) = features.next().await.unwrap() {
        if let Some(name) = feature.properties().unwrap().get("name") {
            if name.len() > longest_name.len() {
                longest_name = name.clone();
            }
        }
    }

    println!("longest name: {longest_name}");
    assert_eq!(longest_name, "French Southern and Antarctic Lands");
}
