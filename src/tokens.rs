//! Tokens used in the nomnoml syntax language


/// Structural link definitions
pub enum Association {
    Simple,
    Linked,
    BiLinked,
    Dependency,
    BiDependency,
    Generalisation,
    RevGeneralisation,
    Implementation,
    RevImplementation,
    Composition,
    LinkComposition,
    Aggregation,
    LinkAggregation,
    Note,
    Hidden,
}

/// Structural node definitions
pub enum Classifiers {
    None,
    Abstract,
    Instance,
    Reference,
    Note,
    Package,
    Frame,
    Database,
    Start,
    End,
    State,
    Choice,
    Input,
    Sender,
    Receiver,
    Transceiver,
    Actor,
    Usecase,
    Label,
    Hidden
}

/// Optional rendering directives
pub enum Directives {
    ArrowSize(u32),
    BendSize(f32),
    Direction(Direction),
    Gutter(u32),
    EdgeMargin(u32),
    Edges(Edges),
    Fill(Color),
    FillArrows(bool),
    Font(String),
    FontSize(u32),
    Leading(f32),
    LineWidth(u32),
    Padding(u32),
    Spacing(u32),
    Stroke(Color),
    Title(String),
    Zoom(usize)
}

/// Directional layout modifiers
pub enum Direction {
    Down,
    Right
}

/// Edge display modifiers
pub enum Edges {
    Hard,
    Rounded
}

/// Generic display modifiers
pub enum Modifiers {
    Center,
    Bold,
    Underline,
    Italic,
    Dashed,
    Empty
}

// TODO: Maybe make this into a custom parsed struct?
pub type Color = String;

