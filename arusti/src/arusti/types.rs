#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub enum ElementType {
    /// Angle defines angle between forward direction and ground
    Line,
    /// Angle defines pull (+ve) or push (-ve) angle. Argument defines radius matching. -ve argument is non-invertible
    Radius,
    /// Angle defines turn angle. Argument defines roll, +ve inside, -ve outside
    Turn,
    /// Angle defines total roll angle. Argument defines hesitation divisions
    Roll,
    /// Angle defines total roll angle
    Flick,
    /// Angle defines total spin angle
    Spin,
    /// Angle defines yaw, argument defines pitch between entry and exit
    Stall,
    /// Defines insertion point for combining elements in a figure
    /// If argument = -1 -> Takes rolls from before figure
    /// If argument = 0 -> Takes rolls from after figure
    /// If argument = [1,2] -> Takes inner rolls
    Combining
    }

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub struct Element {
    pub elem_type: ElementType,
    pub inverted: bool,
    pub angle: f32,
    pub argument: f32,
    }

impl Element {
    pub fn new(elem_type: ElementType) -> Element {
        Element {
            elem_type: elem_type,
            inverted: false,
            angle: 0.0,
            argument: 0.0
            }
        }
    
    pub fn line(angle: f32) -> Element {
        Element {
            angle: angle,
            .. Element::new(ElementType::Line)
            }
        }
    
    pub fn invline(angle: f32) -> Element {
        Element {
            inverted: true,
            .. Element::line(angle)
            }
        }
    
    pub fn radius(angle: f32) -> Element {
        Element {
            angle: angle,
            .. Element::new(ElementType::Radius)
            }
        }
    
    pub fn stall(yaw: f32, pitch: f32) -> Element {
        Element {
            angle: yaw,
            argument: pitch,
            .. Element::new(ElementType::Stall)
            }
        }

    pub fn combining(argument: i8) -> Element {
        Element {
            argument: argument as f32,
            .. Element::new(ElementType::Combining)
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
            elements: Vec::new()
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
    type IntoIter = std::slice::Iter<'a,Element>;

    fn into_iter(self) -> Self::IntoIter {
        let inner_vec = &self.elements;
        inner_vec.into_iter()
        }
    }

#[derive(Debug)]
pub struct Sequence {
    pub figures: Vec<Figure>
    }

impl Sequence {
    pub fn new() -> Sequence {
        Sequence {
            figures: Vec::new()
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
    type IntoIter = std::slice::Iter<'a,Figure>;

    fn into_iter(self) -> Self::IntoIter {
        let inner_vec = &self.figures;
        inner_vec.into_iter()
        }
    }

