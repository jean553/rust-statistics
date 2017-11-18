#[cfg(test)]
mod tests {

    use lib;

    /// Returns the array to work with for tests purposes
    ///
    /// Returns:
    ///
    /// tests purpose array
    fn get_array() -> Vec<f32> {

        return vec![
            9.0,
            3.0,
            3.0,
            4.0,
            5.0,
            4.9,
            8.0,
            3.3,
            2.0,
            0.1,
        ];
    }

    #[test]
    fn test_get_percentile() {

        let array = get_array();

        assert_approx_eq!(
            lib::get_percentile(
                &array,
                70,
            ),
            4.95
        );

        assert_approx_eq!(
            lib::get_percentile(
                &array,
                50,
            ),
            3.65
        );

        assert_approx_eq!(
            lib::get_percentile(
                &array,
                1,
            ),
            0.1
        );
    }

    #[test]
    fn test_get_average() {

        let array = get_array();

        assert_approx_eq!(
            lib::get_average(&array),
            4.23
        );
    }
}
