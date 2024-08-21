
pub mod math {
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }

    #[cfg(test)]
    mod test {
        use super::*;

        /// check if the add function returns the correct value when input parameters are 2 and 1
        ///
        /// # test steps
        ///
        /// * call add with 2 and 1
        /// * check result
        /// * ...
        ///
        /// # Requirement
        ///
        /// * [req_math_add_ID](https://example.com/link_to_req)
        #[test]
        fn add_unit_2_1() {
            let result = add(2, 1);
            assert_eq!(result, 3);
        }

        /// check if the add function returns the correct value when input parameters are 2 and 3
        ///
        /// # test steps
        ///
        /// * call add with 2 and 3
        /// * check result
        /// * ...
        ///
        /// # Requirement
        ///
        /// * [req_math_add_ID](https://example.com/link_to_req)
        #[test]
        fn add_unit_2_3() {
            let result = add(2, 3);
            assert_eq!(result, 5);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// check if the add function returns the correct value when both input parameters are 2
    ///
    /// # test steps
    ///
    /// * call add with 2 and 2
    /// * check result
    /// * ...
    ///
    /// # requirement
    ///
    /// * [req_math_add_ID](https://example.com/link_to_req)
    #[test]
    fn add_check_2_2() {
        let result = math::add(2, 2);
        assert_eq!(result, 4);
    }
    
    /// check if the add function returns the correct value when both input parameters are 3
    ///
    /// # test steps
    ///
    /// * call add with 3 and 3
    /// * check result
    /// * ...
    ///
    /// # requirement
    ///
    /// * [req_math_add_ID](https://example.com/link_to_req)
    #[test]
    fn add_check_3_3() {
        let result = math::add(3, 3);
        assert_eq!(result, 6);
    }
}
