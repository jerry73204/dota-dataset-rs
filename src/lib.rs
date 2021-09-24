mod common;

use crate::common::*;

pub fn load_annotation_file(path: impl AsRef<Path>) -> Result<Vec<Annotation>> {
    let annotations: Vec<_> = io::BufReader::new(fs::File::open(path)?)
        .lines()
        .map(|line| -> Result<_> {
            let line = line?;
            let ann: Annotation = serde_scan::from_str(&line)?;
            Ok(ann)
        })
        .try_collect()?;

    Ok(annotations)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Annotation {
    pub x1: R64,
    pub y1: R64,
    pub x2: R64,
    pub y2: R64,
    pub x3: R64,
    pub y3: R64,
    pub x4: R64,
    pub y4: R64,
    pub category: Category,
    #[serde(with = "serde_zero_one_bool")]
    pub difficult: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    // added in v1.0
    #[serde(rename = "plane")]
    Plane,
    #[serde(rename = "ship")]
    Ship,
    #[serde(rename = "storage-tank")]
    StorageTank,
    #[serde(rename = "baseball-diamond")]
    BaseballDiamond,
    #[serde(rename = "tennis-court")]
    TennisCourt,
    #[serde(rename = "basketball-court")]
    BasketballCourt,
    #[serde(rename = "ground-track-field")]
    GroundTrackField,
    #[serde(rename = "harbor")]
    Harbor,
    #[serde(rename = "bridge")]
    Bridge,
    #[serde(rename = "large-vehicle")]
    LargeVehicle,
    #[serde(rename = "small-vehicle")]
    SmallVehicle,
    #[serde(rename = "helicopter")]
    Helicopter,
    #[serde(rename = "roundabout")]
    Roundabout,
    #[serde(rename = "soccer-ball-field")]
    SoccerBallField,
    #[serde(rename = "swimming-pool")]
    SwimmingPool,
    // added in v1.5
    #[serde(rename = "container-crane")]
    ContainerCrane,
    // added in v2.0
    #[serde(rename = "airport")]
    Airport,
    #[serde(rename = "helipad")]
    Helipad,
}

mod serde_zero_one_bool {
    use super::*;

    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if *value { "1" } else { "0" }.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        let value = match &*text {
            "0" => false,
            "1" => true,
            text => {
                return Err(D::Error::custom(format!(
                    r#"expect "0" or "1", but get "{}""#,
                    text
                )))
            }
        };
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let dir = "/mnt/bbf98471-db16-4e2e-99df-ccd245253072/dota-dataset/train/labelTxt-v1.0/Train_Task2_gt/trainset_reclabelTxt";

        let annotations: Vec<_> = glob::glob(&format!("{}/*.txt", dir))?
            .map(|path| -> Result<_> {
                let path = path?;
                let annotations = load_annotation_file(path)?;
                Ok(annotations)
            })
            .try_collect()?;

        let counts: counter::Counter<_> = annotations
            .into_iter()
            .flatten()
            .map(|ann| ann.category)
            .collect();

        dbg!(&counts);
        let total_count: usize = counts.values().cloned().sum();

        counts.iter().for_each(|(cat, &count)| {
            eprintln!(
                "{:?}\t{:3.2}%",
                cat,
                count as f64 / total_count as f64 * 100.0
            );
        });

        Ok(())
    }
}
