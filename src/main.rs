use fastly::geo::geo_lookup;
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    if let Some(geo) = req.get_client_ip_addr().and_then(geo_lookup) {
        req.set_header("client-geo-city", geo.city());
        req.set_header("client-geo-continent", format!("{:?}", geo.continent()));
        req.set_header("client-geo-country", geo.country_code());
        req.set_header("client-geo-latitude", geo.latitude().to_string());
        req.set_header("client-geo-longitude", geo.longitude().to_string());
    }

    req.set_pass(true);
    Ok(req.send("origin_0")?)
}