extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io::Write;

use reqwest::header::Origin;

pub struct GoogleMaps {
    center: (f64, f64),
    markers: Vec<(f64, f64)>,
}

pub fn json_req(url: &str) -> Option<(f64, f64)> {
    let client = reqwest::Client::new();
    // TODO: Origin header probably non necessary
    let res: serde_json::Value = client.get(url)
                        .header(Origin::new("https", "wikipedia.org", Some(443)))
                        .send().unwrap().json().unwrap();

    //println!("{:?}", res);
    //println!("{:?}", res["results"][0]["geometry"]["location"]["lat"].as_f64().unwrap());
    let lat = res["results"][0]["geometry"]["location"]["lat"].as_f64().unwrap();
    let lng = res["results"][0]["geometry"]["location"]["lng"].as_f64().unwrap();
    println!("Latitude: {}\nLongitude: {}", lat, lng);
    Some((lat, lng))
}

impl GoogleMaps {
    pub fn new(lat: f64, long: f64) -> GoogleMaps {
        GoogleMaps {
            center: (lat, long),
            markers: Vec::new(),
        }
    }

    pub fn from_geocode(geocode: &str) -> Option<GoogleMaps> {
        let endpoint = format!("http://maps.googleapis.com/maps/api/geocode/json?address=\"{}\"", geocode);
        let (lat, long) = json_req(&endpoint).unwrap();
        Some(GoogleMaps::new(lat, long))
    }

    pub fn new_marker(&mut self, location: (f64, f64)) {
        self.markers.push(location);
    }

    pub fn draw(&self, filename: &str) {
        // TODO: validate path
        let mut f = File::create(filename).expect("Unable to create file");
        // TODO: maybe use BufferedWriter
        f.write_all("<html>\n".as_bytes());
        f.write_all("<head>\n".as_bytes());
        f.write_all(
            "<meta name=\"viewport\" content=\"initial-scale=1.0, user-scalable=no\" />\n".as_bytes());
        f.write_all(
            "<meta http-equiv=\"content-type\" content=\"text/html; charset=UTF-8\"/>\n".as_bytes());
        f.write_all("<title>Google Maps - gmplot </title>\n".as_bytes());
        f.write_all(
            "<script type=\"text/javascript\" src=\"https://maps.googleapis.com/maps/api/js?libraries=visualization&sensor=true_or_false\"></script>\n".as_bytes());
        f.write_all("<script type=\"text/javascript\">\n".as_bytes());
        f.write_all("\tfunction initialize() {\n".as_bytes());
        f.write_all(format!("\t\tvar centerlatlng = new google.maps.LatLng({}, {});\n", self.center.0, self.center.1).as_bytes());
        f.write_all("\t\tvar myOptions = {\n".as_bytes());
        f.write_all("\t\t\tzoom: 5,\n".as_bytes());
        f.write_all("\t\t\tcenter: centerlatlng,\n".as_bytes());
        f.write_all("\t\t\tmapTypeId: google.maps.MapTypeId.ROADMAP\n".as_bytes());
        f.write_all("\t\t};\n".as_bytes());
        f.write_all("\t\tvar map = new google.maps.Map(document.getElementById(\"map_canvas\"), myOptions);\n".as_bytes());
        self.draw_markers(&mut f);
        f.write_all("\n".as_bytes());
        f.write_all("\t}\n".as_bytes());
        f.write_all("</script>\n".as_bytes());
        f.write_all("</head>\n".as_bytes());
        f.write_all("<body style=\"margin:0px; padding:0px;\" onload=\"initialize()\">\n".as_bytes());
        f.write_all(
            "\t<div id=\"map_canvas\" style=\"width: 100%; height: 100%;\"></div>\n".as_bytes());
        f.write_all("</body>\n".as_bytes());
        f.write_all("</html>\n".as_bytes());
    }

    fn draw_markers(&self, mut f: &File) {
        let image = "\t\tvar image = 'https://developers.google.com/maps/documentation/javascript/examples/full/images/beachflag.png';";
        f.write_all(image.as_bytes());
        for (index, marker) in self.markers.iter().enumerate() {
            // let loc = "var loc = new google.maps.LatLng(marker.0, marker.1);\n";
            let icon =
                format!("\n\t\tvar marker{} = new google.maps.Marker({{\n\t\t\tposition: {{lat: {}, lng: {}}},\n\t\t\tmap: map,\n\t\t\ticon: image\n\t\t}});", index, marker.0, marker.1);
            f.write_all(icon.as_bytes());
            f.write_all(format!("\n\t\tmarker{}.setMap(map);\n", index).as_bytes());
        }
    }
}

impl std::fmt::Display for GoogleMaps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(center: {:?}, markers: {:?})", self.center, self.markers)
    }
}
