mod rs {

    /// Inserts one value into the given dynamic array
    ///
    /// NOTE: this method is useless,
    /// we could directly use `push()` of `Vec<T>`.
    ///
    /// Args:
    ///
    /// * `array`: mutable reference to the array to modify
    /// * `value`: the value to insert into the array
    pub fn add(
        array: &mut Vec<f32>,
        value: f32,
    ) {
        array.push(value);
    }

    /// Returns the nth percentile of the given array
    ///
    /// Args:
    ///
    /// * `array`: mutable reference to the array to modify
    /// * `percentile`: the expected percentile
    pub fn get_percentile(
        array: &Vec<f32>,
        percentile: f32,
    ) -> f32 {

        let mut sorted_array = array.clone();
        sorted_array.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let index: f32 = percentile / 100.0 * (sorted_array.len() as f32);
        return sorted_array[index.round() as usize];
    }
}

#[cfg(test)]
mod tests {

    use rs;

    #[test]
    fn test_add_values() {

        let mut array = Vec::new();

        rs::add(&mut array, 3.4);

        assert_eq!(
            array,
            [3.4],
            "incorrect one item array"
        );

        rs::add(&mut array, 5.7);

        assert_eq!(
            array,
            [3.4, 5.7],
            "incorrect two items array"
        );
    }

    #[test]
    fn test_get_percentile() {

        let mut first_array = Vec::new();
        let first_values: [f32; 9] = [
            9.0,
            3.0,
            3.0,
            4.0,
            5.0,
            4.9,
            8.0,
            3.3,
            2.0
        ];

        for value in first_values.iter() {
            rs::add(
                &mut first_array,
                *value
            );
        }

        assert_eq!(
            rs::get_percentile(
                &first_array,
                70.0,
            ),
            5.0
        );
    }
}
