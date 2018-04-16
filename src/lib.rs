pub struct GoogleMaps {
    center: (f32, f32),
    markers: Vec<(f32, f32)>,
}

impl GoogleMaps {
    pub fn new() -> GoogleMaps {
        GoogleMaps {
            center: (0_f32, 0_f32),
            markers: Vec::new(),
        }
    }

    pub fn new_marker(&mut self, location: (f32, f32)) {
        unimplemented!();
    }

    pub fn draw(&self, filename: &str) {
        use std::fs::File;
        use std::io::Write;

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
}
