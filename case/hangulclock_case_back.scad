difference() {
    cube([170,170,10], center=true);
    translate([0,0,1.6]) cube([170-1.6*2,150,10], center=true);
    translate([0,0,1.6]) cube([130,170-1.6*2,10], center=true);
    translate([0,0,0]) cube([130,150,15], center=true);
    
    translate([0,17,4]) cube([171,10,5], center=true);
    translate([0,17+34,4]) cube([171,10,5], center=true);
    translate([0,-17,4]) cube([171,10,5], center=true);
    translate([0,-17-34,4]) cube([171,10,5], center=true);
    translate([170/6,0,4]) cube([10,171,5], center=true);
    translate([-170/6,0,4]) cube([10,171,5], center=true);
    
    translate([-170/2,170/2-5,0])
        rotate([0,90,0]) cylinder(h=20,r=1.5,center=true);
    translate([-170/2,-170/2+5,0])
        rotate([0,90,0]) cylinder(h=20,r=1.5,center=true);
    translate([170/2,170/2-5,0])
        rotate([0,90,0]) cylinder(h=20,r=1.5,center=true);
    translate([170/2,-170/2+5,0])
        rotate([0,90,0]) cylinder(h=20,r=1.5,center=true);
}