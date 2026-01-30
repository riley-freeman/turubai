#[derive(Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum FontWeight {
    ExtraBlack  = 950,
    Black       = 900,
    ExtraBold   = 800,
    Bold        = 700,
    SemiBold    = 600,
    Medium      = 500,

    #[default]
    Normal      = 400,
    SemiLight   = 350,
    Light       = 300,
    ExtraLight  = 200,
    Thin        = 100,
}