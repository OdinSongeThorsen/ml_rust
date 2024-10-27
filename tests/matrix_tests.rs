
#[cfg(test)]
mod tests {
    use rust_ml::matrix::Matrix;
    use std::panic::AssertUnwindSafe;

    mod new {
        use super::*;

        #[test]
        fn creates_matrix_with_correct_dimensions_3x5() {
            let matrix = create_matrix_3x5();
            assert_eq!(matrix.rows, 3);
            assert_eq!(matrix.cols, 5);
            assert_eq!(matrix.data.len(), 15);
        }

        #[test]
        fn creates_matrix_with_correct_dimensions_7x2() {
            let matrix = create_matrix_7x2();
            assert_eq!(matrix.rows, 7);
            assert_eq!(matrix.cols, 2);
            assert_eq!(matrix.data.len(), 14);
        }
    }

    mod get {
        use super::*;

        #[test]
        fn returns_correct_value_3x5() {
            let mut matrix = create_matrix_3x5();
            matrix.set(1, 1, 5.0);
            assert_eq!(matrix.get(1, 1), 5.0);
        }

        #[test]
        fn returns_correct_value_7x2() {
            let mut matrix = create_matrix_7x2();
            matrix.set(1, 1, 5.0);
            assert_eq!(matrix.get(1, 1), 5.0);
        }

        #[test]
        fn out_of_bounds_panics_3x5() {
            let matrix = create_matrix_3x5();
            let result = std::panic::catch_unwind(|| matrix.get(3, 5));
            assert!(result.is_err());
        }

        #[test]
        fn out_of_bounds_panics_7x2() {
            let matrix = create_matrix_7x2();
            let result = std::panic::catch_unwind(|| matrix.get(7, 2));
            assert!(result.is_err());
        }
    }

    mod set {
        use super::*;

        #[test]
        fn updates_value_correctly_3x5() {
            let mut matrix = create_matrix_3x5();
            matrix.set(2, 4, 10.0);
            assert_eq!(matrix.get(2, 4), 10.0);
        }

        #[test]
        fn updates_value_correctly_7x2() {
            let mut matrix = create_matrix_7x2();
            matrix.set(6, 1, 10.0);
            assert_eq!(matrix.get(6, 1), 10.0);
        }

        #[test]
        fn out_of_bounds_panics_3x5() {
            let mut matrix = create_matrix_3x5();
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                matrix.set(3, 5, 1.0);
            }));
            assert!(result.is_err());
        }

        #[test]
        fn out_of_bounds_panics_7x2() {
            let mut matrix = create_matrix_7x2();
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                matrix.set(7, 2, 1.0);
            }));
            assert!(result.is_err());
        }
    }

    mod add {
        use super::*;

        #[test]
        fn correct_result() {
            let matrix1 = create_matrix_2x3();
            let matrix2 = create_matrix_3x2();
            let result = matrix1.add(&matrix2);

            assert_eq!(result.rows, 2);
            assert_eq!(result.cols, 2);
            assert_eq!(result.get(0, 0), 58.0);
            assert_eq!(result.get(0, 1), 64.0);
            assert_eq!(result.get(1, 0), 139.0);
            assert_eq!(result.get(1, 1), 154.0);
        }

        #[test]
        fn dimension_mismatch_panics() {
            let matrix1 = create_matrix_2x3();
            let matrix2 = create_matrix_2x3();
            let result = std::panic::catch_unwind(|| matrix1.add(&matrix2));
            assert!(result.is_err());
        }
    }

    mod hadamard_add {
        use super::*;

        #[test]
        fn correct_result() {
            let matrix1 = create_matrix_2x3();
            let matrix2 = create_matrix_2x3();
            let result = matrix1.hadamard_add(&matrix2);

            assert_eq!(result.rows, 2);
            assert_eq!(result.cols, 3);
            assert_eq!(result.get(0, 0), 2.0);
            assert_eq!(result.get(0, 1), 4.0);
            assert_eq!(result.get(0, 2), 6.0);
            assert_eq!(result.get(1, 0), 8.0);
            assert_eq!(result.get(1, 1), 10.0);
            assert_eq!(result.get(1, 2), 12.0);
        }

        #[test]
        fn dimension_mismatch_panics() {
            let matrix1 = create_matrix_2x3();
            let matrix2 = create_matrix_3x2();
            let result = std::panic::catch_unwind(|| matrix1.hadamard_add(&matrix2));
            assert!(result.is_err());
        }
    }

    fn create_matrix_2x3() -> Matrix {
        let mut matrix = Matrix::new(2, 3);
        matrix.set(0, 0, 1.0);
        matrix.set(0, 1, 2.0);
        matrix.set(0, 2, 3.0);
        matrix.set(1, 0, 4.0);
        matrix.set(1, 1, 5.0);
        matrix.set(1, 2, 6.0);
        matrix
    }

    fn create_matrix_3x2() -> Matrix {
        let mut matrix = Matrix::new(3, 2);
        matrix.set(0, 0, 7.0);
        matrix.set(0, 1, 8.0);
        matrix.set(1, 0, 9.0);
        matrix.set(1, 1, 10.0);
        matrix.set(2, 0, 11.0);
        matrix.set(2, 1, 12.0);
        matrix
    }

    fn create_matrix_3x5() -> Matrix {
        Matrix::new(3, 5)
    }

    fn create_matrix_7x2() -> Matrix {
        Matrix::new(7, 2)
    }
}