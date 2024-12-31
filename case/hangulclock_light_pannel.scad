difference() {
    union() {
        translate([-30,-30,0]) board_supports();
        translate([0,50,0]) oled_supports();
        cube([170, 170, 2], center=true);
        translate([0,170/6,1]) cube([170, 5, 3], center=true);
        translate([0,-170/6,1]) cube([170, 5, 3], center=true);
        translate([-170/6,0,1]) cube([5,170, 3], center=true);
        translate([+170/6,0,1]) cube([5,170, 3], center=true);
    }
    translate([-170/2,0,-1]) cube([12,12,15], center=true);
    translate([170/2,0,-1]) cube([12,12,15], center=true);
    translate([-170/2,33,-1]) cube([12,12,15], center=true);
    translate([170/2,33,-1]) cube([12,12,15], center=true);
    translate([-170/2,-33,-1]) cube([12,12,15], center=true);
    translate([170/2,-33,-1]) cube([12,12,15], center=true);    
    translate([-170/2,33*2,-1]) cube([12,12,15], center=true);
    translate([170/2,33*2,-1]) cube([12,12,15], center=true);
    translate([-170/2,-33*2,-1]) cube([12,12,15], center=true);
    translate([170/2,-33*2,-1]) cube([12,12,15], center=true);
}

module oled_supports() {
    translate([-(38.5-3)/2,-(35.5-3)/2]) {
        support(3, 5);
        translate([38.5-3,0,0]) support(3, 5);
        translate([0,35.5-3,0]) support(3, 5);
        translate([38.5-3,35.5-3,0]) support(3, 5);
    }
}

module board_supports() {
    translate([-(41.5-3)/2,-(46.5-3)/2]) {
        support(2, 5);
        translate([41.5-2,0,0]) support(2, 5);
        translate([0,46.5-2,0]) support(2, 5);
        translate([41.5-2,46.5-2,0]) support(2, 5);
    }
}

module support(in, h) {
    translate([0,0,h/2]) difference() {
        cylinder(h, in/2+1,in/2+1, center=true);
        translate([0,0,-1]) cylinder(h+2, in/2, in/2, center=true);
    }
}