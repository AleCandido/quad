use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash;
use std::sync::{Arc, Mutex};
use crate::funct_vector::FnVecGen;
use crate::qag_vec_norm_integrator_result::QagVecNormIntegratorResult;
use crate::qk61_vec_norm2::*;
use crate::result_state::*;




#[derive(Clone)]
pub struct QagVecNormPar {
    pub key : i32,
    pub limit : usize,
}

///           f      : f64
///                     function
///
///           a      : f64
///                    lower limit of integration
///
///           b      : f64
///                    upper limit of integration
///
///           epsabs : f64
///                    absolute accuracy requested
///
///           epsrel : f64
///                    relative accuracy requested
///                    if  epsabs <= 0 && epsrel <= max(50*rel.mach.acc.,0.5d-28),
///                    the fn will return with result_state = Invalid.
///
///            key   : i32
///                    key for choice of local integration rule. A gauss-kronrod pair is used with:
///                          7 - 15 points if key < 2,
///                         10 - 21 points if key = 2,
///                         15 - 31 points if key = 3,
///                         20 - 41 points if key = 4,
///                         25 - 51 points if key = 5,
///                         30 - 61 points if key > 5.
///
///            limit : i32
///                    gives an upperbound on the number of subintervals in the partition
///                    of (a,b), limit >= 1.
///
///
///
///         On return : QagIntegratorResult :
///
///           QagIntegrationResult:
///           result : f64
///                    Approximation to the integral.
///
///           abserr : f64
///                    Estimate of the modulus of the absolute error,
///                    which should equal or exceed abs(i-result).
///
///           neval  : i32
///                    Number of integrand evaluations.
///
///           alist  : Vec<f64>
///                      Vector of dimension at least limit, the elements of which are the left
///                      end points of the subintervals in the partition of the given integration
///                      range (a,b).
///
///           blist  : Vec<f64>
///                      Vector of dimension at least limit, the elements of which are the right
///                      end points of the subintervals in the partition of the given integration
///                      range (a,b).
///
///           rlist  : Vec<f64>
///                      Vector of dimension at least limit, the elements of which are the integral
///                      approximations on the subintervals.
///
///            rlist  : Vec<f64>
///                      Vector of dimension at least limit, the elements of which are the moduli
///                      of the absolute error estimates on the subintervals.
///
///            iord   : Vec<usize>
///                      Vector of dimension at least limit, the elements of which are pointers to
///                      the error estimates over the subintervals, such that
///                      elist(iord(1)), ...,elist(iord(k)) form a decreasing sequence,
///                      with k = last if last <= (limit/2+2), and
///                      k = limit+1-last otherwise.
///
///            last    : usize
///                      number of subintervals actually produced in the
///                      subdivision process
///
///
///
///
///           ResultState =
///           Success :
///                    Normal and reliable termination of the routine. it is assumed that the
///                    requested accuracy has been achieved.
///           MaxIteration :
///                    The maximum number of steps has been executed. the integral is probably too
///                    difficult to be calculated by dqng.
///           Invalid :
///                     The input is invalid, because epsabs <= 0 &&
///                     epsrel < max(50 * rel.mach.acc.,0.5e-28).
///           BadTolerance :
///                     The occurrence of roundoff error is detected, which prevents the requested
///                     tolerance from being achieved.
///           BadFunction :
///                     Extremely bad integrand behaviour occurs at some points of the integration
///                     interval.
///
///
///           If ResultState != Succes =>    It is assumed that the requested accuracy has not
///           been achieved.
///
///
///





impl QagVecNormPar {
    pub fn qintegrate<const n :usize>(&self, fun : FnVecGen<n>, a : f64, b : f64, epsabs : f64, epsrel : f64)
                                      ->  QagVecNormIntegratorResult<n> {

        if epsabs <= 0.0 && epsrel < 0.5e-28_f64.max(50.0 * EPMACH) {
            return QagVecNormIntegratorResult::new_error(ResultState::Invalid)
        }


        //            first approximation to the integral

        let mut neval = 0;
        let mut last= 1 ;
        let mut result = Arc::new(Mutex::new([0.0;n]));
        let mut abserr = Arc::new(Mutex::new(0.0));
        let mut rounderr  = Arc::new(Mutex::new(0.0));
        let f = fun.components;

        let qk61 = Qk61VecNorm2 {};

        let mut keyf = self.key;
        if self.key <= 0 { keyf = 1; }
        if self.key >= 7 { keyf = 6; }
        match keyf {
            6 => (*result.lock().unwrap(), *abserr.lock().unwrap(), *rounderr.lock().unwrap()) = qk61.integrate(&*f, a, b),
            _ => (),
        }

        //           test on accuracy.

        let mut errbnd = epsabs.max(epsrel * norm_vec(&*result.lock().unwrap()));

        let mut interval_cache = Arc::new(Mutex::new(HashMap::from([((Myf64{x:a},Myf64{x:b}),result.lock().unwrap().clone())])));
        let mut heap = Arc::new(Mutex::new(BinaryHeap::new()));
        heap.lock().unwrap().push(HeapItem::new((a,b),*abserr.lock().unwrap()));

        //  DA VEDERE !!!!!!!!!!!!!
        //if abserr <= 50.0 * EPMACH * defabs[k] && abserr[k] > errbnd[k] {
        //    return QagVecIntegratorResult::new_error(ResultState::BadTolerance)
        //}


        if *abserr.lock().unwrap() <= errbnd{
            if keyf != 1 { neval = (10 * keyf + 1) * (2 * neval + 1); }
            if keyf == 1 { neval = 30 * neval + 15; }
            println!("first iter is enough ");
            return QagVecNormIntegratorResult::new(*result.lock().unwrap(),*abserr.lock().unwrap(),neval,last)
        }

        if self.limit == 1 {
            return QagVecNormIntegratorResult::new_error(ResultState::MaxIteration)
        }

        //          initialization

        //          main do-loop
        //           bisect the subinterval with the largest error estimate.




        while last < self.limit{
            let mut to_process = vec![];
            let mut err_sum = 0.0;

            {
                let mut heap = heap.lock().unwrap();
                let mut interval_cache = interval_cache.lock().unwrap();
                let mut abserr = abserr.lock().unwrap();

                while to_process.len() < 128 && heap.len() != 0 {
                    let old_interval = heap.pop().unwrap();
                    let ((x, y), old_err) = (old_interval.interval, old_interval.err);
                    let old_res = interval_cache.remove(&(Myf64 { x }, Myf64 { x: y })).unwrap();
                    err_sum += old_err;
                    to_process.push((x, y, old_err, old_res));
                    if err_sum > *abserr - errbnd / 8.0 { break }
                }
            }

            rayon::scope(|s| {




                for comp in to_process {

                    let mut result = result.clone();
                    let mut abserr = abserr.clone();
                    let mut heap = heap.clone();
                    let mut rounderr = rounderr.clone();
                    let mut interval_cache = interval_cache.clone();
                    let f = f.clone();

                    s.spawn(move |_| {
                        last += 1;

                        //let f = f.components;

                        let mut result1 = [0.0; n];
                        let mut abserr1 = 0.0;
                        let mut rounderr1 = 0.0;

                        let mut result2 = [0.0; n];
                        let mut abserr2 = 0.0;
                        let mut rounderr2 = 0.0;

                        let a1 = comp.0;
                        let b1 = 0.5 * (comp.0 + comp.1);
                        let a2 = b1;
                        let b2 = comp.1;

                        let qk61 = Qk61VecNorm2 {};

                        match keyf {
                            6 => {
                                (result1, abserr1, rounderr1) = qk61.integrate(&*f, a1, b1);
                                (result2, abserr2, rounderr2) = qk61.integrate(&*f, a2, b2);
                            },
                            _ => (),
                        }

                        let mut result = result.lock().unwrap();
                        res_update(&mut *result, &result1, &result2, &comp.3);
                        drop(result);

                        let mut interval_cache = interval_cache.lock().unwrap();
                        interval_cache.insert((Myf64 { x: a1 }, Myf64 { x: b1 }), result1);
                        interval_cache.insert((Myf64 { x: a2 }, Myf64 { x: b2 }), result2);
                        drop(interval_cache);

                        let mut heap = heap.lock().unwrap();
                        heap.push(HeapItem::new((a1, b1), abserr1));
                        heap.push(HeapItem::new((a2, b2), abserr2));
                        drop(heap);


                        *abserr.lock().unwrap() += -comp.2 + abserr1 + abserr2;
                        *rounderr.lock().unwrap() += rounderr1 + rounderr2;
                    })
                }



            });
            let result = result.lock().unwrap();
            let abserr = abserr.lock().unwrap();
            let rounderr = rounderr.lock().unwrap();

            errbnd = epsabs.max(epsrel * norm_vec(&*result));


            if *abserr <= errbnd / 8.0{ break;}
            if *abserr < *rounderr {
                return QagVecNormIntegratorResult::new_error(ResultState::BadTolerance)
            }
        }


        let result = result.lock().unwrap().clone();
        let abserr = abserr.lock().unwrap().clone();
        return QagVecNormIntegratorResult::new(result,abserr,neval,last)

    }
}


pub fn norm_vec(v : &[f64]) -> f64{
    let mut norm = 0.0;
    for comp in v{
        norm += comp.powi(2);
    }
    norm = norm.sqrt();
    norm
}

pub fn res_update(v : &mut[f64], w: &[f64], z : &[f64], y : &[f64]){
    for k  in 0..v.len(){
        v[k] += w[k] + z[k] - y[k];
    }
}

#[derive(Debug)]
pub struct HeapItem {
    interval : (f64,f64),
    err : f64,
}

impl HeapItem {
    pub fn new( interval : (f64,f64) , err : f64) -> Self{
        Self{ interval,err}
    }
}

impl Eq for HeapItem{}

impl PartialEq for HeapItem{
    fn eq(&self, other: &Self) -> bool {
        self.err == other.err
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.err).partial_cmp(&other.err).unwrap()
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



#[derive(Debug)]
pub struct Myf64{
    x : f64,
}
impl Myf64 {
    fn key(&self) -> u64 {
        self.x.to_bits()
    }
}

impl hash::Hash for Myf64 {
    fn hash<H>(&self, state: &mut H)
        where
            H: hash::Hasher,
    {
        self.key().hash(state)
    }
}

impl PartialEq for Myf64 {
    fn eq(&self, other: &Myf64) -> bool {
        self.key() == other.key()
    }
}

impl Eq for Myf64{}


