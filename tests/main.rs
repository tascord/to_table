#[cfg(test)]
pub mod test {
    use std::cmp::Ordering;

    use to_table::ToTable;

    pub trait TableData: 'static + Clone {
        #[allow(unused_variables)]
        fn sort(&self, other: &Self, field: String) -> Ordering {
            Ordering::Equal
        }
    }

    #[allow(dead_code)]
    #[derive(ToTable, Clone)]
    struct Data {
        #[skip]
        effective_date_time_utc: String,
        effective_date_time_sydney: String,
        effective_date_sydney: String,
        #[unwrap]
        bid: Option<f32>,
        #[unwrap]
        low: Option<f32>,
        #[unwrap]
        ask: Option<f32>,
        #[unwrap]
        high: Option<f32>,
        #[unwrap]
        volume: Option<f32>,
        #[unwrap]
        last: Option<f32>,
        code: String,
        region: String,
    }

    #[test]
    pub fn it_works() {
        let data = Data {
            effective_date_time_utc: "2021-01-01T00:00:00".to_string(),
            effective_date_time_sydney: "2021-01-01T10:00:00".to_string(),
            effective_date_sydney: "2021-01-01".to_string(),
            bid: Some(1.0),
            low: Some(1.0),
            ask: Some(1.0),
            high: Some(1.0),
            volume: Some(1.0),
            last: Some(1.0),
            code: "ABC".to_string(),
            region: "Australia".to_string(),
        };

        let data2 = Data {
            effective_date_time_utc: "2021-01-01T00:00:00".to_string(),
            effective_date_time_sydney: "2021-01-01T10:00:00".to_string(),
            effective_date_sydney: "2021-01-01".to_string(),
            bid: Some(1.0),
            low: Some(1.0),
            ask: Some(1.0),
            high: Some(1.0),
            volume: Some(1.0),
            last: Some(1.0),
            code: "ABC".to_string(),
            region: "Australia".to_string(),
        };

        assert_eq!(data.sort(&data2, "code".to_string()), std::cmp::Ordering::Equal);
    }
}