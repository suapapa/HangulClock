union() {
    translate([33/2,0,0]) wall_w(33*5);
    translate([-33/2,0,0]) wall_w(33*5);
    translate([33/2+33,0,0]) wall_w(33*5);
    translate([-33/2-33,0,0]) wall_w(33*5);
    translate([33/2+33*2,0,0]) wall_w(170);
    translate([-33/2-33*2,0,0]) wall_w(170);
   
    translate([0,33/2,0]) wall_h(33*5);
    translate([0,-33/2,0]) wall_h(33*5);
    translate([0,33/2+33,0]) wall_h(33*5);
    translate([0,-33/2-33,0]) wall_h(33*5);
    translate([0,+33/2+33*2,0]) wall_h(170);
    translate([0,-33/2-33*2,0]) wall_h(170);
    
    base();
}

module wall_w(w) {
    translate([0,0,12/2])
        cube([1, w, 12], center=true);
}

module wall_h(h) {
    translate([0,0,12/2])
        cube([h, 1, 12], center=true);
}

module base() {
    translate([0,0,0.5/2])
        difference() {
            cube([170,170,0.5], center=true);
            cube([33*5,33*5,1], center=true);
        }
}