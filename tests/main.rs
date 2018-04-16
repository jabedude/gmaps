extern crate gmaps;

#[test]
fn test_all() {
    use gmaps::GoogleMaps;
    let gm = GoogleMaps::new();
    gm.draw(&"/vagrant/test_map.html");
}
