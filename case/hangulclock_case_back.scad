difference() {
    cube([170,170,10], center=true);
    translate([0,0,3]) cube([165,150,10], center=true);
    translate([0,0,3]) cube([130,165,10], center=true);
    translate([0,0,0]) cube([130,150,15], center=true);
    translate([0,170/6,4]) cube([171,10,5], center=true);
    translate([0,-170/6,4]) cube([171,10,5], center=true);
    translate([170/6,0,4]) cube([10,171,5], center=true);
    translate([-170/6,0,4]) cube([10,171,5], center=true);
}