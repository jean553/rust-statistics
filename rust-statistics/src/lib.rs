/// we allow dead code to prevent warnings saying the functions are not used
#[allow(dead_code)]
mod lib {

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

        if index.floor() == index {
            return (
                sorted_array[(index - 1.0) as usize] +
                sorted_array[index as usize]
            ) / 2.0;
        }

        sorted_array[index.ceil() as usize]
    }

    /// Returns the average of values into the dynamic array
    ///
    /// Args:
    ///
    /// * `array`: reference to the array
    pub fn get_average(array: &Vec<f32>) -> f32 {

        let mut sum = 0.0;
        for value in array {
            sum += *value;
        }

        return sum / (array.len() as f32);
    }
}

#[cfg(test)]
mod tests;
