use pyo3::prelude::*;
use pyo3::types::PyType;

fn tama<'py>(py: Python<'py>) {
    static ID3_MODULE: pyo3::sync::PyOnceLock<Py<PyType>> = pyo3::sync::PyOnceLock::new();
    let id3_object = ID3_MODULE.import(py, "mutagen.easyid3", "EasyID3").unwrap();
    static ID3_KLUDGE: std::sync::Once = std::sync::Once::new();
    ID3_KLUDGE.call_once(|| {
        id3_object
            .call_method1("RegisterTXXXKey", ("albumartist", "QuodLibet::albumartist"))
            .unwrap();
    });
}

fn main() {
    Python::initialize();
    Python::attach(|py| {
        tama(py);
    });
    println!("Hello, world!");
}
