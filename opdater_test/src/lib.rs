#[cfg(test)]
mod tests {
    use opdater::Opdater;

    #[test]
    fn simple_test() {
        let mut a: Option<i32> = None;
        Opdater::update(&mut a, Some(5));
        assert_eq!(a, Some(5));
    }

    #[test]
    fn simple_struct() {
        #[derive(Debug, PartialEq, Opdater)]
        struct Bla {
            a: Option<i32>,
            b: Option<f32>,
        }

        let mut bla = Bla { a: None, b: None };
        let bla_op = Bla {
            a: Some(10),
            b: Some(13.37),
        };

        bla.update(bla_op);

        assert_eq!(
            bla,
            Bla {
                a: Some(10),
                b: Some(13.37)
            }
        );

        let bla_op2 = Bla {
            a: Some(5),
            b: None,
        };

        bla.update(bla_op2);

        assert_eq!(
            bla,
            Bla {
                a: Some(5),
                b: Some(13.37)
            }
        );
    }

    #[test]
    fn option_option() {
        #[derive(Debug, PartialEq, Opdater)]
        struct Bla {
            a: Option<Option<i32>>,
            b: Option<f32>,
        }

        let mut bla = Bla { a: None, b: None };
        let bla_op = Bla {
            a: Some(Some(10)),
            b: Some(13.37),
        };

        bla.update(bla_op);

        assert_eq!(
            bla,
            Bla {
                a: Some(Some(10)),
                b: Some(13.37)
            }
        );

        let bla_op2 = Bla {
            a: Some(None),
            b: None,
        };

        bla.update(bla_op2);

        assert_eq!(
            bla,
            Bla {
                a: Some(None),
                b: Some(13.37)
            }
        );
    }
}
