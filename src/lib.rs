use country_geocoder;
use geo::Point;
use lazy_static::lazy_static;
use pyo3::prelude::*;

lazy_static! {
    static ref CODER: country_geocoder::CountryGeocoder = country_geocoder::CountryGeocoder::new();
}

#[pyclass]
#[derive(Debug)]
pub struct Info {
    #[pyo3(get)]
    pub iso_a2: String,
    #[pyo3(get)]
    pub name_sort: String,
    #[pyo3(get)]
    pub left_handed: bool,
}

#[pyfunction]
fn get_info(_py: Python, lng: f64, lat: f64) -> PyResult<Info> {
    let res = CODER.lookup(Point::new(lng, lat)).unwrap();
    return Ok(Info {
        iso_a2: res.iso_a2.to_owned(),
        name_sort: res.name_sort.to_owned(),
        left_handed: res.left_handed.to_owned(),
    });
}


#[pymodule]
fn geocoderpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_info, m)?)?;
    Ok(())
}
