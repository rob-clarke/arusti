#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CombinableElementType {
    Line,
    Radius,
    Turn,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ElementType {
    /// Main angle defines angle between forward direction and ground
    /// Auxiliary angle defines rolls
    /// Matching specifies a line matching identifier. All lines with the same identifier in a
    ///  figure should have the same length.
    Line,

    /// Main angle defines pull (+ve) or push (-ve) angle
    /// Auxiliary angle defines rolls
    /// Matching specifies a radius matching identifier. All radii with the same identifier in a
    ///  figure should have the same radius.
    /// If matching is negative, the radius cannot be inverted to make the exit attitude match
    Radius,

    /// Main angle defines turn angle
    /// Auxiliary angle defines roll, positive angle is towards inside, negative is outside
    Turn,

    /// Main angle defines yaw between entry and exit
    /// Auxiliary angle defines pitch between entry and exit
    Stall,

    /// Defines insertion point for combining elements in a figure
    /// If matching = -1 -> Takes rolls from before figure
    /// If matching = 0 -> Takes rolls from after figure
    /// If matching = [1,2] -> Takes inner rolls
    Combining {
        base_type: CombinableElementType,
        roll_selector: i32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Attitude {
    Normal,
    Inverted,
    KnifeEdge,
}

impl Attitude {
    pub fn get_inverted(attitude: Attitude) -> Attitude {
        match attitude {
            Attitude::Normal => Attitude::Inverted,
            Attitude::Inverted => Attitude::Normal,
            _ => attitude
            }
        }
    }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RollType {
    None,
    Standard,
    Flick,
    InvertedFlick,
    Spin,
    InvertedSpin,
    HesitationHalves,
    HesitationQuarters,
    HesitationEighths,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Element {
    pub elem_type: ElementType,
    pub attitude: Attitude,
    pub main_angle: i16,
    pub aux_angle: i16,
    pub roll_type: RollType,
    pub matching: i32,
}

impl Element {
    pub fn new(elem_type: ElementType) -> Element {
        Element {
            elem_type: elem_type,
            attitude: Attitude::Normal,
            main_angle: 0,
            aux_angle: 0,
            roll_type: RollType::None,
            matching: 0,
        }
    }

    pub fn line(angle: i16) -> Element {
        Element {
            main_angle: angle,
            ..Element::new(ElementType::Line)
        }
    }

    pub fn invline(angle: i16) -> Element {
        Element {
            attitude: Attitude::Inverted,
            ..Element::line(angle)
        }
    }

    pub fn keline(angle: i16) -> Element {
        Element {
            attitude: Attitude::KnifeEdge,
            ..Element::line(angle)
        }
    }

    pub fn radius(angle: i16) -> Element {
        Element {
            main_angle: angle,
            ..Element::new(ElementType::Radius)
        }
    }

    pub fn stall(yaw: i16, pitch: i16) -> Element {
        Element {
            main_angle: yaw,
            aux_angle: pitch,
            ..Element::new(ElementType::Stall)
        }
    }

    pub fn combining(roll_selector: i32, base_element: Element) -> Element {
        let base_type = match base_element.elem_type {
            ElementType::Line => CombinableElementType::Line,
            ElementType::Radius => CombinableElementType::Radius,
            ElementType::Turn => CombinableElementType::Turn,
            _ => {
                panic!("Attempt to use non-combinable base element");
            }
        };

        Element {
            elem_type: ElementType::Combining {
                base_type,
                roll_selector,
            },
            ..base_element
        }
    }
}

#[derive(Debug)]
pub struct Figure {
    pub elements: Vec<Element>,
}

impl Figure {
    pub fn new() -> Figure {
        Figure {
            elements: Vec::new(),
        }
    }

    pub fn append(self: &mut Figure, mut new_elements: Vec<Element>) {
        self.elements.append(&mut new_elements);
    }
    pub fn push(self: &mut Figure, new_element: Element) {
        self.elements.push(new_element);
    }
}

impl<'a> IntoIterator for &'a Figure {
    type Item = &'a Element;
    type IntoIter = std::slice::Iter<'a, Element>;

    fn into_iter(self) -> Self::IntoIter {
        let inner_vec = &self.elements;
        inner_vec.into_iter()
    }
}

#[derive(Debug)]
pub struct Sequence {
    pub figures: Vec<Figure>,
}

impl Sequence {
    pub fn new() -> Sequence {
        Sequence {
            figures: Vec::new(),
        }
    }

    pub fn append(self: &mut Sequence, mut new_figures: Vec<Figure>) {
        self.figures.append(&mut new_figures);
    }

    pub fn push(self: &mut Sequence, new_figure: Figure) {
        self.figures.push(new_figure);
    }
}

impl<'a> IntoIterator for &'a Sequence {
    type Item = &'a Figure;
    type IntoIter = std::slice::Iter<'a, Figure>;

    fn into_iter(self) -> Self::IntoIter {
        let inner_vec = &self.figures;
        inner_vec.into_iter()
    }
}
