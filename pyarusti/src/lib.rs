use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use pyo3::types::{PyDict, PyList};

use arusti;
use arusti::{Attitude, ElementType, RollType};

struct ElementWrapper(arusti::Element);

impl IntoPy<PyObject> for ElementWrapper {
    fn into_py(self, py: Python) -> PyObject {
        let pyelement = PyDict::new(py);

        pyelement.set_item(
            "type",
            match self.0.elem_type {
                ElementType::Line => "Line",
                ElementType::Radius => "Radius",
                ElementType::Turn => "Turn",
                ElementType::Stall => "Stall",
                _ => unreachable!(),
            },
        );

        pyelement.set_item(
            "attitude",
            match self.0.attitude {
                Attitude::Normal => "Normal",
                Attitude::Inverted => "Inverted",
                Attitude::KnifeEdge => "KnifeEdge",
            },
        );

        pyelement.set_item("main_angle", self.0.main_angle);
        pyelement.set_item("aux_angle", self.0.aux_angle);
        if self.0.roll_type != RollType::None {
            pyelement.set_item(
                "roll_type",
                match self.0.roll_type {
                    RollType::Standard => "Standard",
                    RollType::Flick => "Flick",
                    RollType::InvertedFlick => "InvertedFlick",
                    RollType::Spin => "Spin",
                    RollType::InvertedSpin => "InvertedSpin",
                    RollType::HesitationHalves => "HesitationHalves",
                    RollType::HesitationQuarters => "HesitationQuarters",
                    RollType::HesitationEighths => "HesitationEighths",
                    _ => unreachable!(),
                },
            );
        }

        pyelement.set_item("matching", self.0.matching);
        pyelement.into_py(py)
    }
}

struct FigureWrapper(arusti::Figure);

impl IntoPy<PyObject> for FigureWrapper {
    fn into_py(self, py: Python) -> PyObject {
        let pyelements = PyList::empty(py);

        for element in self.0.elements {
            pyelements.append(ElementWrapper(element).into_py(py));
        }
        pyelements.into_py(py)
    }
}

struct SequenceWrapper(arusti::Sequence);

impl IntoPy<PyObject> for SequenceWrapper {
    fn into_py(self, py: Python) -> PyObject {
        let pyfigures = PyList::empty(py);

        for figure in self.0.figures {
            pyfigures.append(FigureWrapper(figure).into_py(py));
        }
        pyfigures.into_py(py)
    }
}

#[pymodule]
fn pyarusti(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "parse")]
    fn parse(py: Python, sequence_string: String) -> PyResult<SequenceWrapper> {
        let sequence = arusti::olan::parse_sequence(sequence_string);
        Ok(SequenceWrapper(sequence))
    }
    Ok(())
}
