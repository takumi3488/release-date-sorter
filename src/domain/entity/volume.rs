use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, sqlx::FromRow, Serialize)]
pub struct Volume {
    pub id: Uuid,
    pub title: String,
    pub publication_date: NaiveDate,
}

impl Volume {
    pub fn compare_publication_date(&self, other: &Self) -> std::cmp::Ordering {
        self.publication_date.cmp(&other.publication_date)
    }
}

#[derive(Clone)]
pub struct NewVolume {
    pub series_id: String,
    pub title: String,
    pub publication_date: NaiveDate,
}

impl NewVolume {
    pub fn new(series_id: &str, title: &str, publication_date: NaiveDate) -> Self {
        Self {
            series_id: series_id.to_string(),
            title: title.to_string(),
            publication_date,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    impl Volume {
        pub fn new(id: Uuid, title: &str, publication_date: NaiveDate) -> Self {
            Self {
                id,
                title: title.to_string(),
                publication_date,
            }
        }
    }

    #[test]
    fn test_compare_publication_date() {
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

        assert_eq!(
            volume1.compare_publication_date(&volume2),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            volume2.compare_publication_date(&volume3),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            volume3.compare_publication_date(&volume1),
            std::cmp::Ordering::Greater
        );
    }
}
