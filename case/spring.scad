difference() {
    union() {
        springLeaf(5, 9, 3);
        translate([0,-3.25,0]) cube([9,1,5], center=true);
        //translate([0,3.25,0]) cube([9,1,5], center=true);
    }
}

module springLeaf(width=3,len=10,height=5){
 scale([len/10,height/10,1]) difference() {
     cylinder(h=width, r=10, center=true);
     #cylinder(h=width+2, r=7.8, center=true);
  }
}