use serde::Serialize;

use super::volume::Volume;

#[derive(Clone, Serialize)]
pub struct Series {
    pub id: String,
    pub url: String,
    pub title: String,
    pub volumes: Vec<Volume>,
}

impl Series {
    pub fn new(id: &str, url: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            url: url.to_string(),
            title: title.to_string(),
            volumes: vec![],
        }
    }

    pub fn with_volumes(&self, volumes: Vec<Volume>) -> Self {
        Self {
            id: self.id.clone(),
            url: self.url.clone(),
            title: self.title.clone(),
            volumes,
        }
    }

    pub fn sort_volumes_by_publication_date(&mut self) {
        self.volumes.sort_by(|a, b| a.compare_publication_date(b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use uuid::Uuid;

    #[test]
    fn test_sort_volumes_by_publication_date() {
        let volume1 = Volume::new(
            Uuid::new_v4(),
            "Volume 1",
            NaiveDate::from_num_days_from_ce_opt(0).unwrap(),
        );
        let volume2 = Volume::new(
            Uuid::new_v4(),
            "Volume 2",
            NaiveDate::from_num_days_from_ce_opt(1).unwrap(),
        );
        let volume3 = Volume::new(
            Uuid::new_v4(),
            "Volume 3",
            NaiveDate::from_num_days_from_ce_opt(2).unwrap(),
        );

        let mut series =
            Series::new("", "", "Series").with_volumes(vec![volume3, volume1, volume2]);
        series.sort_volumes_by_publication_date();

        assert_eq!(series.volumes[0].title, "Volume 1");
        assert_eq!(series.volumes[1].title, "Volume 2");
        assert_eq!(series.volumes[2].title, "Volume 3");
    }
}
