mod flight;
use crate::flight::Flight;

fn main() {
    println!("Hello, world!");
    let mut _flight = Flight{
        airline_iata: String::from("SQ"),
        airline_icao: String::from("SIA"),
        flight_no: 123,
        origin_iata: String::from("SIN"),
        origin_icao: String::from("WSSS"),
        destination_iata: String::from("BKK"),
        destination_icao: String::from("VTBS")
    };

    println!("{:?}", _flight);

    _flight.flight_no = 124;
    println!("{:?}", _flight);
}
