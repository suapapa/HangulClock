difference() {
    translate([0,0,40/2])
        difference() {
            cube([176,176,40], center=true);
            cube([165,165,70], center=true);
            translate([0,0,3]) cube([171,171,40], center=true);
            //translate([2.5,2.5,2.5]) cube([170,170,70]);
            
        }

    translate([-170/2,170/2-5,24])
        rotate([0,90,0]) screw_hole();
    translate([-170/2,-170/2+5,24])
        rotate([0,90,0]) screw_hole();
    translate([170/2,170/2-5,24])
        rotate([0,-90,0]) screw_hole();
    translate([170/2,-170/2+5,24])
        rotate([0,-90,0]) screw_hole();
}



module screw_hole() {
    translate([0,0,-3])
    union() {
        translate([0,0,5]) cylinder(10, 1.5+0.1, 1.5+0.1, center=true, $fn=20);
        translate([0,0,2.5/2-0.2]) cylinder(2.5+0.1,3.5+0.1,1.5, center=true, $fn=20);
    }
}