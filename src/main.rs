use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::cyclic_group::IsGroup;

const SECRET_KEY :u64 = 0x6C616D6264617370;

fn main() {

    // https://github.com/lambdaclass/lambdaworks/blob/main/math/src/elliptic_curve/README.md#defining-points-and-operating-with-the-curves
    // let g = BLS12381Curve::generator();
    // let g2 = g.operate_with_self(2_u64);

    let g = BLS12381Curve::generator();  
    let pub_key = g.operate_with_self(SECRET_KEY);
    println!("Public Key {:#?}", pub_key);

}