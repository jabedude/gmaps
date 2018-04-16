extern crate gmaps;

#[test]
fn test_all() {
    use gmaps::GoogleMaps;
    let gm = GoogleMaps::new("mymap.html");
    gm.draw();
}
