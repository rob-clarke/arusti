use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use pyo3::types::{PyDict,PyList};

use arusti;

struct ElementWrapper(arusti::Element);

impl IntoPy<PyObject> for ElementWrapper {
    fn into_py(self, py: Python) -> PyObject {
        let pyelement = PyDict::new(py);

        match self.0.elem_type {
            arusti::ElementType::Line => pyelement.set_item("type","line"),
            arusti::ElementType::Radius => pyelement.set_item("type","radius"),
            arusti::ElementType::Turn => pyelement.set_item("type","turn"),
            arusti::ElementType::Roll => pyelement.set_item("type","roll"),
            arusti::ElementType::Flick => pyelement.set_item("type","flick"),
            arusti::ElementType::Spin => pyelement.set_item("type","spin"),
            arusti::ElementType::Stall => pyelement.set_item("type","stall"),
            _ => unreachable!()
            };

        pyelement.set_item("inverted",self.0.inverted);
        pyelement.set_item("angle",self.0.angle);
        pyelement.set_item("argument",self.0.argument);
        
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
    
    #[pyfn(m,"parse")]
    fn parse(py: Python, sequence_string: String) -> PyResult<SequenceWrapper> {
        let sequence = arusti::olan::parse_sequence(sequence_string);
        Ok(SequenceWrapper(sequence))
    }
    
    Ok(())
}
