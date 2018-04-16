pub struct GoogleMaps {
    coordinate: u64,
}

impl GoogleMaps {
    pub fn new(filename: &str) -> GoogleMaps {
        GoogleMaps { coordinate: 0 }
    }

    pub fn draw(&self) {
        use std::fs::File;
        use std::io::Write;


        let mut f = File::create("/vagrant/mymap.html").expect("Unable to create file");
        f.write_all("<html>\n".as_bytes());
        f.write_all("<head>\n".as_bytes());
        f.write_all(
            "<meta name=\"viewport\" content=\"initial-scale=1.0, user-scalable=no\" />\n".as_bytes());
        f.write_all(
            "<meta http-equiv=\"content-type\" content=\"text/html; charset=UTF-8\"/>\n".as_bytes());
        f.write_all("<title>Google Maps - gmplot </title>\n".as_bytes());
        f.write_all("<script type=\"text/javascript\" src=\"https://maps.googleapis.com/maps/api/js?libraries=visualization&sensor=true_or_false\"></script>\n".as_bytes());
        f.write_all("<script type=\"text/javascript\">\n".as_bytes());
        f.write_all("\tfunction initialize() {\n".as_bytes());
        f.write("\t\tvar centerlatlng = new google.maps.LatLng(0.000000, 0.000000);\n".as_bytes());
        f.write("\t\tvar myOptions = {\n".as_bytes());
        f.write("\t\t\tzoom: 5,\n".as_bytes());
        f.write("\t\t\tcenter: centerlatlng,\n".as_bytes());
        f.write("\t\t\tmapTypeId: google.maps.MapTypeId.ROADMAP\n".as_bytes());
        f.write("\t\t};\n".as_bytes());
        f.write("\t\tvar map = new google.maps.Map(document.getElementById(\"map_canvas\"), myOptions);\n".as_bytes());
        f.write("\n".as_bytes());
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
