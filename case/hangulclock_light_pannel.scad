difference() {
    union() {
        translate([-30,50,0]) oled_supports();
        translate([30,50,0]) oled_supports_v();
        translate([-30,-35,0]) board_supports();
        translate([30,-35,0]) board_supports_2();
        cube([170, 170, 1.2], center=true);
        translate([-170/6,0,1]) cube([5,170, 2.5], center=true);
        translate([+170/6,0,1]) cube([5,170, 2.5], center=true);
        translate([0,17,1]) cube([170, 5, 2.5], center=true);
        translate([0,17+34,1]) cube([170, 5, 2.5], center=true);
        translate([0,-17,1]) cube([170, 5, 2.5], center=true);
        translate([0,-17-34,1]) cube([170, 5, 2.5], center=true);
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
    
    translate([75,82.5,0]) cube([10,6,2], center=true);
    translate([-75,82.5,0]) cube([10,6,2], center=true);
    translate([75,-82.5,0]) cube([10,6,2], center=true);
    translate([-75,-82.5,0]) cube([10,6,2], center=true);
}

module oled_supports() {
    translate([-(33.7-3)/2,-(31.8-3)/2]) {
        support(3, 5);
        translate([33.7-3,0,0]) support(3, 5);
        translate([0,31.8-3,0]) support(3, 5);
        translate([33.7-3,31.8-3,0]) support(3, 5);
    }
}

module oled_supports_v() {
    translate([-(33.7-3)/2,-(31.8-3)/2]) {
        support(3, 5);
        translate([0,33.7-3,0]) support(3, 5);
        translate([31.8-3,0,0]) support(3, 5);
        translate([31.8-3,33.7-3,0]) support(3, 5);
    }
}

// for pcb rev2 and rev3
module board_supports() {
    translate([-(41.5-2)/2,-(46.5-2)/2]) {
        support(3, 5);
        translate([38,0,0]) support(3, 5);
        translate([0,47,0]) support(3, 5);
        translate([38,47,0]) support(3, 5);
    }
}

// for pcb rev1
module board_supports_2() {
    translate([-(46.5)/2,-(46.5)/2]) {
        support(3, 5);
        translate([46.5,0,0]) support(3, 5);
        translate([0,46.5,0]) support(3, 5);
        translate([46.5,46.5,0]) support(3, 5);
    }
}

module support(in, h) {
    translate([0,0,h/2]) difference() {
        cylinder(h, in/2+1,in/2+1, center=true);
        translate([0,0,-1]) cylinder(h+2, in/2, in/2, center=true);
    }
}