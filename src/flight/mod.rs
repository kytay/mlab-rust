#[derive(Debug)]
pub struct Flight {
    pub airline_iata: String,
    pub airline_icao: String,
    pub flight_no: u64,
    pub origin_iata: String,
    pub origin_icao: String,
    pub destination_iata: String,
    pub destination_icao: String
}