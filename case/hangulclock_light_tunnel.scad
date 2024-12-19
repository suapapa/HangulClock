union() {
    tunnel(0,0);
    translate([33/2,0,0]) cube([2,170-1,15], center=true);
    translate([-33/2,0,0]) cube([2,170-1,15], center=true);
    translate([33/2+33,0,0]) cube([2,170-1,15], center=true);
    translate([-33/2-33,0,0]) cube([2,170-1,15], center=true);
    translate([0,33/2,0]) cube([170-1,2,15], center=true);
    translate([0,-33/2,0]) cube([170-1,2,15], center=true);
    translate([0,33/2+33,0]) cube([170-1,2,15], center=true);
    translate([0,-33/2-33,0]) cube([170-1,2,15], center=true);
}

module tunnel(x, y) {
    translate([x,y,0]) difference() {
        cube([33*5+1,33*5+1,15], center=true);
        translate([0,0,-1])cube([33*5-1-2,33*5-1-2,25], center=true);
    }
}