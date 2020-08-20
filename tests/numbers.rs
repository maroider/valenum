use valenum::valenum;

valenum! {
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) enum Region {
        Europe = 0,
        NorthAmerica = 1,
        SouthAmerica = 2,
        Asia = 3,
        Other(i32),
    }
}

valenum! {
    #[derive(Debug, PartialEq, Eq)]
    pub enum Country {
        Afghanistan = 0,
        Albania = 1,
        Algeria = 2,
        Andorra = 3,
        Angola = 4,
        Unknown { country_id: i32 },
    }
}

#[test]
fn regions() {
    let raw_region1 = 0;
    let raw_region2 = 9;
    let region1 = Region::Europe;
    let region2 = Region::Other(9);
    assert_eq!(raw_region1, region1.into());
    assert_eq!(raw_region2, region2.into());
    assert_eq!(Region::from(raw_region1), region1);
    assert_eq!(Region::from(raw_region2), region2);
}

#[test]
fn countries() {
    let raw_country1 = 1;
    let raw_country2 = 10;
    let country1 = Country::Albania;
    let country2 = Country::Unknown { country_id: 10 };
    assert_eq!(raw_country1, country1.into());
    assert_eq!(raw_country2, country2.into());
    assert_eq!(Country::from(raw_country1), country1);
    assert_eq!(Country::from(raw_country2), country2);
}
