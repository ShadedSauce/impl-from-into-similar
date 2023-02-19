#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use impl_from_into_similar::{impl_into_similar};

    #[test]
    fn test_impl_into() {
        #[derive(Default, Clone, Serialize, Deserialize)]
        struct StructA {
            f1: i32,
            f2: i32
        }

        #[derive(Default, Serialize, Deserialize)]
        struct StructB {
            f2: i32,
            f1: i32,

            #[serde(default)]
            f3: String
        }

        impl_into_similar!(StructB, StructA);

        let a = StructA {
            f1: 1,
            f2: 2
        };

        let b: StructB = a.clone().try_into().unwrap();

        assert_eq!(b.f1, a.f1);
        assert_eq!(b.f2, a.f2);
        assert_eq!(b.f3, String::default());
    }
}