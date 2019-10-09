use valenum::valenum;

valenum! {
    pub(crate) enum Regions {
        Europe = 0,
        NorthAmerica = 1,
        SouthAmerica = 2,
        Asia = 3,
        Other(i32),
    }
}

valenum! {
    pub enum Countries {
        Afghanistan = 0,
        Albania = 1,
        Algeria = 2,
        Andorra = 3,
        Angola = 4,
        Unknown { country_id: i32 },
    }
}
