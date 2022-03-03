use strum_macros::{Display, FromRepr};

#[derive(Clone, Copy, Debug, Display, FromRepr)]
pub enum IsaacVersion {
    Rebirth,
    Afterbirth,
    AfterbirthPlus,
    AfterbirthPlusBP5,
    Repentance,
}

#[derive(Clone, Copy, FromRepr)]
pub enum Activity {
    Backup,
    Install,
}
