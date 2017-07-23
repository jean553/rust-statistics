/// TODO: #4 we use `assert_eq!` to compare float,
/// we should check if there is a better way for float comparison

#[cfg(test)]
mod tests {

    use lib;

    /// Returns the array to work with for tests purposes
    ///
    /// TODO: #3 check if the array can be declared as a constant global
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
            0.1
        ];
    }

    #[test]
    fn test_get_percentile() {

        let array = get_array();

        /* FIXME: two floats comparison here */
        assert_approx_eq!(
            lib::get_percentile(
                &array,
                70,
            ),
            4.95
        );

        /* FIXME: two floats comparison here */
        assert_approx_eq!(
            lib::get_percentile(
                &array,
                50,
            ),
            3.65
        );

        /* FIXME: two floats comparison here */
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

        /* FIXME: two floats comparison here */
        assert_approx_eq!(
            lib::get_average(&array),
            4.23
        );
    }
}
