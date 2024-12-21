difference() {
    cube([170,170,10], center=true);
    translate([0,0,3]) cube([165,150,10], center=true);
    translate([0,0,3]) cube([150,165,10], center=true);
    translate([0,0,0]) cube([150,150,15], center=true);
}