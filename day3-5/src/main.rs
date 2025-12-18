fn main() {
    let data: &str = "Bitcoin, 80_000, USD";
    let csv_data = parse_csv(data);
    println!("{:?}", csv_data);
}

fn parse_csv(row: &str) -> Vec<&str> {
    let result = row.split(',').map(|s| s.trim()).collect();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv() {
        let data: &str = "Bitcoin, 80_000, USD";

        let csv_data = parse_csv(data);

        assert_eq!(csv_data[0], "Bitcoin");
        assert_eq!(csv_data[1], "80_000");
        assert_eq!(csv_data[2], "USD");
    }
}
