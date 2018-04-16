extern crate gmaps;

use gmaps::GoogleMaps;

#[test]
fn test_all() {
    let gm = GoogleMaps::new();
    gm.draw(&"/vagrant/test_map.html");
}

#[test]
fn test_marker() {
    let mut gm = GoogleMaps::new();
    let location = (37.770776, -122.461689);
    gm.new_marker(location);
    gm.draw(&"/vagrant/test_map.html");
    println!("{}", gm);
    // TODO
    // assert_eq!(gm.markers[0], (37.770776, -122.461689));
}
