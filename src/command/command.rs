#[derive(strum::EnumString, Debug, PartialEq, Clone)]
pub enum Command {
    #[strum(ascii_case_insensitive)]
    CREATE,
    #[strum(ascii_case_insensitive)]
    SELECT,
    #[strum(ascii_case_insensitive)]
    SHOW,
    #[strum(ascii_case_insensitive)]
    ADD,
    #[strum(ascii_case_insensitive)]
    DELETE,
}
