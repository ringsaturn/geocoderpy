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
    pub code: String,
    #[pyo3(get)]
    pub name: String,
}

#[pyfunction]
fn get_info(_py: Python, lng: f64, lat: f64) -> PyResult<Info> {
    let res = CODER.lookup(Point::new(lng, lat));
    if res.is_none() {
        return Ok(Info {
            code: "".to_string(),
            name: "".to_string(),
        });
    }
    let v = res.unwrap();
    return Ok(Info {
        code: v.code.to_owned(),
        name: v.name.to_owned(),
    });
}

#[pymodule]
fn geocoderpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_info, m)?)?;
    Ok(())
}
