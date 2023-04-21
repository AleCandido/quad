use std::simd::{f64x4, Simd, SimdFloat};
use crate::funct_vector::FnVec4;
use crate::qk::*;
use crate::qk61_simd::*;

pub struct Qk61Vec4Simd {}

impl Qk61Vec4Simd {
    pub(crate) fn integrate(&self, fun: &FnVec4, a: f64, b: f64, ) -> (Simd<f64, 4>, Simd<f64, 4>, Simd<f64, 4>, Simd<f64, 4>, ) {
        let hlgth = f64x4::splat(0.5 * (b - a));
        let dhlgth = hlgth.abs();
        let centr: f64 = 0.5 * (b + a);

        let mut resk = f64x4::splat(0.0);
        let mut resabs = f64x4::splat(0.0);
        let mut resg =  f64x4::splat(0.0);
        let mut resasc =  f64x4::splat(0.0);


        for k in 0..4 {
            let fv = fvec_simd(&fun.components[k], centr, hlgth[k]);
            resk[k] = (fv * WGK).reduce_sum();
            let reskh = resk[k] * 0.5;
            let reskhs = Simd::from_array([reskh; 64]);
            resabs[k] = (fv.abs() * WGK).reduce_sum();
            resg[k] = (fv * WG).reduce_sum();
            resasc[k] = (WGK * (fv - reskhs).abs()).reduce_sum();
        }

        let result = resk * hlgth;
        resabs = resabs * dhlgth;
        resasc = resasc * dhlgth;
        let mut abserr = ((resk - resg) * hlgth).abs();
        for k in 0..4 {
            if (resasc[k], abserr[k]) != (0.0, 0.0) {
                abserr[k] = resasc[k] * 1.0_f64.min((200.0 * abserr[k] / resasc[k]).powf(1.5));
            }

            if resabs[k] > UFLOW / (50.0 * EPMACH) {
                abserr[k] = abserr[k].max((EPMACH * 50.0) * resabs[k]);
            }
        }

        (result, abserr, resabs, resasc)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::funct_vector::FnVec4;
    use crate::qk61::Qk61;
    use crate::qk61_1dvec3::Qk611DVec3;
    use crate::qk61_simd::Qk61Simd;
    use crate::qk61_4vec_simd::Qk61Vec4Simd;
    use crate::qk::Qk;

    #[test]
    fn test(){
        let f = |x:f64| x.cos();

        let a = 0.0;
        let b = 1.0;
        let qks = Qk61Vec4Simd{};
        let qk = Qk61Simd {};
        let fun = FnVec4{ components : [Box::new(f),Box::new(f),Box::new(f),Box::new(f)]};

        for k in 0..100 {
            let start = Instant::now();
            let res1 = qk.integrate(&f, a, b);
            let res2 = qk.integrate(&f,a,b);
            let res3 = qk.integrate(&f,a,b);
            let res4 = qk.integrate(&f,a,b);
            println!("normal {:?}", start.elapsed());
            let start = Instant::now();
            let res_simd = qks.integrate(&fun, a, b);
            println!("simd {:?}", start.elapsed());
        }
        //println!("{:?}",res1);
        //println!("{:?}",res1);
    }
}

/*

pub(crate) fn integrate(&self, fun: &FnVec4, a: f64, b: f64, ) -> (Simd<f64, 4>, Simd<f64, 4>, Simd<f64, 4>, Simd<f64, 4>, ) {
    let qk61_1d = Qk61Simd2 {};
    /*
    let (mut result_vec,mut abserr_vec,mut resabs_vec,mut resasc_vec) =
        ([0.0;4],[0.0;4],[0.0;4],[0.0;4]);
    for k in 0..4 {
        (result_vec[k], abserr_vec[k], resabs_vec[k], resasc_vec[k]) = qk61_1d.integrate(&fun.components[k],a,b);
    }
    (Simd::from_array(result_vec),Simd::from_array(abserr_vec),
     Simd::from_array(resabs_vec),Simd::from_array(resasc_vec))
      */

    let (mut result_vec, mut abserr_vec, mut resabs_vec, mut resasc_vec) =
        (f64x4::splat(0.0), f64x4::splat(0.0), f64x4::splat(0.0), f64x4::splat(0.0));
    (result_vec[0], abserr_vec[0], resabs_vec[0], resasc_vec[0]) =
        qk61_1d.integrate(&fun.components[0], a, b);
    (result_vec[1], abserr_vec[1], resabs_vec[1], resasc_vec[1]) =
        qk61_1d.integrate(&fun.components[1], a, b);
    (result_vec[2], abserr_vec[2], resabs_vec[2], resasc_vec[2]) =
        qk61_1d.integrate(&fun.components[2], a, b);
    (result_vec[3], abserr_vec[3], resabs_vec[3], resasc_vec[3]) =
        qk61_1d.integrate(&fun.components[3], a, b);
    (result_vec, abserr_vec, resabs_vec, resasc_vec)
}

 */
