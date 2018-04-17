extern crate gmaps;

use gmaps::GoogleMaps;

#[test]
fn test_create() {
    let gm = GoogleMaps::new();
}

#[test]
fn test_marker() {
    let mut gm = GoogleMaps::new();
    let location = (37.770776, -122.461689);
    gm.new_marker(location);
    gm.draw(&"/vagrant/test_map.html");
    println!("{}", gm);
}
