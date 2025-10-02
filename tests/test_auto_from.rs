#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use fake::{Dummy, Fake, Faker};
    use rstest::*;
    use struct_auto_from::auto_from;

    #[derive(PartialEq, Eq, Debug, Clone, Dummy)]
    #[auto_from(Model2)]
    struct Model1 {
        id: i32,
        name: String,
        attrs: Vec<String>,
    }

    #[auto_from(Model1)]
    #[derive(PartialEq, Eq, Debug, Clone, Dummy)]
    struct Model2 {
        id: i32,
        name: String,
        attrs: Vec<String>,
    }

    #[auto_from(Model1)]
    #[derive(PartialEq, Eq, Debug, Clone, Dummy)]
    struct Model3 {
        #[auto_from_attr(default_value = 0)]
        id: i32,
        name: String,
        attrs: Vec<String>,
        #[auto_from_attr(default_value = Default::default())]
        metadata: HashMap<String, String>,
    }

    // In the target struct (the one with the #auto_from annotation) we
    // can also use types that can auto convert to the respective type
    // in the reference struct.
    // In the following example Model4 has the custom type 'MyString'
    // for the field 'name'. This works because MyString implements From<String>.
    #[derive(PartialEq, Eq, Debug, Clone, Dummy)]
    struct MyString {
        value: String,
        other: i32,
    }

    impl From<String> for MyString {
        fn from(value: String) -> Self {
            MyString { value, other: 42 }
        }
    }

    #[auto_from(Model1)]
    #[derive(PartialEq, Eq, Debug, Clone, Dummy)]
    struct Model4 {
        id: i64,
        name: MyString,
        attrs: Vec<String>,
    }

    #[rstest]
    fn test_auto_from() {
        // given
        let m1: Model1 = Faker.fake();
        let m2: Model2 = Faker.fake();

        // when
        let m1_2: Model2 = m1.clone().into();
        let m2_1: Model1 = m2.clone().into();

        // then
        assert!(m1.id == m1_2.id);
        assert!(m1.name == m1_2.name);
        assert!(m1.attrs == m1_2.attrs);

        assert!(m2.id == m2_1.id);
        assert!(m2.name == m2_1.name);
        assert!(m2.attrs == m2_1.attrs);
    }

    #[rstest]
    fn test_auto_from_transitive() {
        // given
        let m1: Model1 = Faker.fake();
        let m2: Model2 = Faker.fake();

        // when
        let m1_2: Model2 = m1.clone().into();
        let m2_1: Model1 = m2.clone().into();
        let m1_2_1: Model1 = m1_2.into();
        let m2_1_2: Model2 = m2_1.into();

        // then
        assert!(m1 == m1_2_1);
        assert!(m2 == m2_1_2);
    }

    #[rstest]
    fn test_auto_from_default_value() {
        // given
        let mut m1: Model1 = Faker.fake();
        m1.id = 1;

        // when
        let m1_3: Model3 = m1.clone().into();

        // then
        assert!(m1_3.id == 0);
        assert!(m1_3.name == m1.name);
        assert!(m1_3.attrs == m1.attrs);
        assert!(m1_3.metadata == Default::default());
    }
    
    #[rstest]
    fn test_auto_from_into_value() {
        // given
        let mut m1: Model1 = Faker.fake();
        m1.id = 1;

        // when
        let m1_4: Model4 = m1.clone().into();

        // then
        assert!(m1_4.id == 0);
        assert!(m1_4.name.value == m1.name);
        assert!(m1_4.attrs == m1.attrs);
    }

}
