#[macro_use] extern crate assert_approx_eq;

/// we allow dead code to prevent warnings saying the functions are not used
#[allow(dead_code)]
mod lib {

    use std::f32;

    /// Returns the nth percentile of the given array
    ///
    /// TODO: #7 handle float comparisons
    ///
    /// Args:
    ///
    /// * `array`: mutable reference to the array to modify
    /// * `percentile`: the expected percentile
    pub fn get_percentile(
        array: &Vec<f32>,
        percentile: u8,
    ) -> f32 {

        let mut sorted_array = array.clone();
        sorted_array.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let index: f32 = percentile as f32 / 100.0 * sorted_array.len() as f32;

        if (index.floor() - index).abs() <= f32::EPSILON {

            let index = index as usize;
            let start = index - 1;
            let end = index + 1;

            return get_average(&sorted_array[start..end]);
        }

        sorted_array[index.floor() as usize]
    }

    /// Returns the average of values into the dynamic array
    ///
    /// Args:
    ///
    /// * `array`: slice of values
    pub fn get_average(slice: &[f32]) -> f32 {

        let sum: f32 = slice.iter().sum();
        return sum / (slice.len() as f32);
    }
}

#[cfg(test)]
mod tests;
