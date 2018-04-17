extern crate gmaps;

use gmaps::GoogleMaps;

#[test]
fn test_create() {
    let gm = GoogleMaps::new(0.0, 0.0);
}

#[test]
fn test_marker() {
    let mut gm = GoogleMaps::new(37.766956, -122.438481);
    let location = (37.770776, -122.461689);
    gm.new_marker(location);
    gm.draw(&"/vagrant/test_map.html");
    println!("{}", gm);
}

#[test]
fn test_multiple_markers() {
    let mut gm = GoogleMaps::new(37.766956, -122.438481);
    let location1 = (37.770776, -122.461689);
    let location2 = (37.770786, -122.461679);
    gm.new_marker(location1);
    gm.new_marker(location2);
    gm.draw(&"/vagrant/test_map.html");
    println!("{}", gm);
}
