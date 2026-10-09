#[repr(u16)]
pub enum EncounterStartContext {
    Normal,
    Chance,
    Pinch,
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum TimeOfDay {
    EarlyMorning,
    Morning,
    LunchTime,
    Afternoon,
    AfterSchool,
    Night,
    LateNight,
}