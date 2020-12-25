use crate::game::coords;
trait Gradient{

  
  fn getRandomGradientAt (point: Vector, scale: Fraction) -> Vector;
  
}



const vecs: [[u16;2]; 16]= [
  [1000, 0],
  [923, 382],
  [707, 707],
  [382, 923],
  [0, 1000],
  [-383, 923],
  [-708, 707],
  [-924, 382],
  [-1000, 0],
  [-924, -383],
  [-708, -708],
  [-383, -924],
  [-1, -1000],
  [382, -924],
  [707, -708],
  [923, -383],
];

impl Gradient for Coords{
  fn getRandomGradientAt (point: Vector, scale: Fraction) -> Vector {

    let index = MimcState::sponge(&[point.xi, point.yi], U512(4)).remainder(16);
    vecs[index]
  }

}
  
  
  
  
 
  

  

struct IntegerVector {
  x: u32,
  y: u32,
  trackLCM: bool, 
  lcm: U512
}

impl IntegerVector{
  fn perlin(&mut self, p: IntegerVector, floor:bool,) -> u32 {
    const MAX_PERLIN_VALUE:u8 = 32;

    let fractionalP = Vector{ x: Fraction(self.x), y: Fraction{xself.y} };

    // we want 4096, 8192. 16384
    let mut ret = (12..15).fold(Fraction{x:0, y:1},|ret,i| {
      ret.add(fractionalP.valueAt(Fraction{x:2.pow(i),y:1}))
    }).div(3);
  
  
    if (self.trackLCM) {
      self.lcm =     U512.lcm(self.lcm, U512(ret.d));
    }

    ret = ret.mul(MAX_PERLIN_VALUE / 2);

    if floor{
      ret = ret.floor();
    }

    ret = ret.add(MAX_PERLIN_VALUE / 2);
  
    ret = ret.valueOf();
    Math.floor(ret * 100) / 100
  }
  
}



struct Vector {
  x: Fraction,
  y: Fraction,
}

impl Vector{


fn valueAt (&self, scale:Fraction)-> Fraction{


  let bottomLeftCoords = Coords{
    x: self.x.sub(realMod(self.x, scale)),
    y: self.y.sub(realMod(self.y, scale)),
  };

  let bottomRightCoords = Coords{
    x: bottomLeftCoords.x.add(scale),
    y: bottomLeftCoords.y,
  };

  let topLeftCoords = Coords{
    x: bottomLeftCoords.x,
    y: bottomLeftCoords.y.add(scale),
  };
  let topRightCoords = Coords{
    x: bottomLeftCoords.x.add(scale),
    y: bottomLeftCoords.y.add(scale),
  };

  let bottomLeftGrad =GradientAtPoint {
    coords: bottomLeftCoords,
    gradient: getRandomGradientAt(bottomLeftCoords, scale),
  };
  let bottomRightGrad = GradientAtPoint{
    coords: bottomRightCoords,
    gradient: getRandomGradientAt(bottomRightCoords, scale),
  };
  let topLeftGrad = GradientAtPoint{
    coords: topLeftCoords,
    gradient: getRandomGradientAt(topLeftCoords, scale),
  };
  let topRightGrad = GradientAtPoint{
    coords: topRightCoords,
    gradient: getRandomGradientAt(topRightCoords, scale),
  };

   self.perlinValue(&[bottomLeftGrad, bottomRightGrad, topLeftGrad, topRightGrad],scale)

}



fn minus (&self, b: Vector) ->Vector {
  Vector{
    x: self.x.sub(b.x),
    y: self.y.sub(b.y),
  }
}

fn dot (&self, b: Vector) -> Fraction {
  self.x.mul(b.x).add(self.y.mul(b.y))
}

fn smoothStep (&self) -> Fraction {
  // return 6 * x ** 5 - 15 * x ** 4 + 10 * x ** 3;
  self
}

fn scalarMultiply (&self, s: Fraction) ->Vector {
  Vector{
    x: self.x.mul(s),
    y: self.y.mul(s),
  }
}

fn getWeight (&self, corner: Vector) -> Fraction {
  let basb = self.y.sub(corner.y).abs();
  let dabf = Fraction(1).sub(basb);
  let grep = dabf.smoothStep();
  let afaf = Fraction(1).sub(self.x.sub(corner.x).abs());

  afaf.smoothStep().mul(grep)
}

// p is in a scale x scale square. we scale down to a 1x1 square
fn perlinValue (&self,
  corners: &[GradientAtPoint;4],
  scale: Fraction,
) -> Fraction  {

  corners.fold(Fraction{},|ret, corners|{
    let distVec = minus(p, corner.coords);
    let g = scalarMultiply(scale.inverse(), distVec);
    let h = scalarMultiply(scale.inverse(), corner.coords);
    let v = scalarMultiply(scale.inverse(), p);
    let dot = dot(g, corner.gradient);
    let blah = v.getWeight(h).mul(dot);

    ret.add( blah )
  })

}
}






struct GradientAtPoint {
  coords: Vector,
  gradient: Vector,
}
