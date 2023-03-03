use crate::qk::{EPMACH, UFLOW};
//  use std::time::Instant;
//  use std::{thread,time};
use crate::qng_integrator_result::*;
use crate::result_state::*;
use crate::quad_integral_method::*;
use crate::quad_integrator_result::QuadIntegratorResult;

#[derive(Clone)]
pub struct Qng{}



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
///
///         On return : QagIntegratorResult :
///
///           QngIntegrationResult:
///           result : f64
///                    approximation to the integral i
///                    result is obtained by applying the 21-point
///                    gauss-kronrod rule (res21) obtained by optimal
///                    addition of abscissae to the 10-point gauss rule
///                    (res10), or by applying the 43-point rule (res43)
///                    obtained by optimal addition of abscissae to the
///                    21-point gauss-kronrod rule, or by applying the
///                    87-point rule (res87) obtained by optimal addition
///                    of abscissae to the 43-point rule.
///
///           abserr : f64
///                    estimate of the modulus of the absolute error,
///                    which should equal or exceed abs(i-result)
///
///           neval  : i32
///                    number of integrand evaluations
///
///           ResultState =
///           Success :
///                    normal and reliable termination of the routine. it is assumed that the
///                    requested accuracy has been achieved.
///           MaxIteration :
///                    the maximum number of steps has been executed. the integral is probably too
///                    difficult to be calculated by dqng.
///           Invalid :
///                     the input is invalid, because epsabs <= 0 &&
///                     epsrel < max(50 * rel.mach.acc.,0.5e-28).
///           If ResultState != Succes =>  QngIntegration.{result, abserr,neval} are set to zero.
///
///           the following are the abscissae and weights of the integration rules used.
///
///           x1 :      abscissae common to the 10, 21, 43 and 87 point rule
///           x2 :      abscissae common to the 21, 43 and 87 point rule
///           x3 :      abscissae common to the 43 and 87 point rule
///           x4 :      abscissae of the 87 point rule
///           w10 :     weights of the 10 point formula
///           w21a :    weights of the 21 point formula for abscissae x1
///           w21b :    weights of the 21 point formula for abscissae x2
///           w43a :    weights of the 43 point formula for abscissae x1, x3
///           w43b :    weights of the 43 point formula for abscissae x3
///           w87a :    weights of the 87 point formula for abscissae x1, x2, x3
///           w87b :    weights of the 87 point formula for abscissae x4
///
///
///         These coefficients were calculated with 101 decimal digit arithmetic by l. w. fullerton,
///         bell labs, nov 1981.
///
///
///


const X1: [f64;5] = [0.973906528517171720077964012084452,
                      0.865063366688984510732096688423493,
                      0.679409568299024406234327365114874,
                      0.433395394129247190799265943165784,
                      0.148874338981631210884826001129720];

const W10: [f64;5] = [0.066671344308688137593568809893332,
                       0.149451349150580593145776339657697,
                       0.219086362515982043995534934228163,
                       0.269266719309996355091226921569469,
                       0.295524224714752870173892994651338];

const X2: [f64;5]  = [0.995657163025808080735527280689003,
                       0.930157491355708226001207180059508,
                       0.780817726586416897063717578345042,
                       0.562757134668604683339000099272694,
                       0.294392862701460198131126603103866];

const W21A: [f64;5] = [0.032558162307964727478818972459390,
                        0.075039674810919952767043140916190,
                        0.109387158802297641899210590325805,
                        0.134709217311473325928054001771707,
                        0.147739104901338491374841515972068];

const W21B: [f64;6] = [0.011694638867371874278064396062192,
                        0.054755896574351996031381300244580,
                        0.093125454583697605535065465083366,
                        0.123491976262065851077958109831074,
                        0.142775938577060080797094273138717,
                        0.149445554002916905664936468389821];

const X3: [f64;11] = [0.999333360901932081394099323919911,
                       0.987433402908088869795961478381209,
                       0.954807934814266299257919200290473,
                       0.900148695748328293625099494069092,
                       0.825198314983114150847066732588520,
                       0.732148388989304982612354848755461,
                       0.622847970537725238641159120344323,
                       0.499479574071056499952214885499755,
                       0.364901661346580768043989548502644,
                       0.222254919776601296498260928066212,
                       0.074650617461383322043914435796506];

const W43A: [f64;10] = [0.016296734289666564924281974617663,
                         0.037522876120869501461613795898115,
                         0.054694902058255442147212685465005,
                         0.067355414609478086075553166302174,
                         0.073870199632393953432140695251367,
                         0.005768556059769796184184327908655,
                         0.027371890593248842081276069289151,
                         0.046560826910428830743339154433824,
                         0.061744995201442564496240336030883,
                         0.071387267268693397768559114425516];

const W43B: [f64;12] = [0.001844477640212414100389106552965,
                         0.010798689585891651740465406741293,
                         0.021895363867795428102523123075149,
                         0.032597463975345689443882222526137,
                         0.042163137935191811847627924327955,
                         0.050741939600184577780189020092084,
                         0.058379395542619248375475369330206,
                         0.064746404951445885544689259517511,
                         0.069566197912356484528633315038405,
                         0.072824441471833208150939535192842,
                         0.074507751014175118273571813842889,
                         0.074722147517403005594425168280423];

const X4: [f64;22] = [0.999902977262729234490529830591582,
                       0.997989895986678745427496322365960,
                       0.992175497860687222808523352251425,
                       0.981358163572712773571916941623894,
                       0.965057623858384619128284110607926,
                       0.943167613133670596816416634507426,
                       0.915806414685507209591826430720050,
                       0.883221657771316501372117548744163,
                       0.845710748462415666605902011504855,
                       0.803557658035230982788739474980964,
                       0.757005730685495558328942793432020,
                       0.706273209787321819824094274740840,
                       0.651589466501177922534422205016736,
                       0.593223374057961088875273770349144,
                       0.531493605970831932285268948562671,
                       0.466763623042022844871966781659270,
                       0.399424847859218804732101665817923,
                       0.329874877106188288265053371824597,
                       0.258503559202161551802280975429025,
                       0.185695396568346652015917141167606,
                       0.111842213179907468172398359241362,
                       0.037352123394619870814998165437704];

const W87A: [f64;21] = [0.008148377384149172900002878448190,
                         0.018761438201562822243935059003794,
                         0.027347451050052286161582829741283,
                         0.033677707311637930046581056957588,
                         0.036935099820427907614589586742499,
                         0.002884872430211530501334156248695,
                         0.013685946022712701888950035273128,
                         0.023280413502888311123409291030404,
                         0.030872497611713358675466394126442,
                         0.035693633639418770719351355457044,
                         0.000915283345202241360843392549948,
                         0.005399280219300471367738743391053,
                         0.010947679601118931134327826856808,
                         0.016298731696787335262665703223280,
                         0.021081568889203835112433060188190,
                         0.025370969769253827243467999831710,
                         0.029189697756475752501446154084920,
                         0.032373202467202789685788194889595,
                         0.034783098950365142750781997949596,
                         0.036412220731351787562801163687577,
                         0.037253875503047708539592001191226];

const W87B: [f64;23] = [0.000274145563762072350016527092881,
                         0.001807124155057942948341311753254,
                         0.004096869282759164864458070683480,
                         0.006758290051847378699816577897424,
                         0.009549957672201646536053581325377,
                         0.012329447652244853694626639963780,
                         0.015010447346388952376697286041943,
                         0.017548967986243191099665352925900,
                         0.019938037786440888202278192730714,
                         0.022194935961012286796332102959499,
                         0.024339147126000805470360647041454,
                         0.026374505414839207241503786552615,
                         0.028286910788771200659968002987960,
                         0.030052581128092695322521110347341,
                         0.031646751371439929404586051078883,
                         0.033050413419978503290785944862689,
                         0.034255099704226061787082821046821,
                         0.035262412660156681033782717998428,
                         0.036076989622888701185500318003895,
                         0.036698604498456094498018047441094,
                         0.037120549269832576114119958413599,
                         0.037334228751935040321235449094698,
                         0.037361073762679023410321241766599];

impl Qng{
    pub fn qintegrate(&self,f : &dyn Fn(f64)->f64, a : f64, b : f64, epsabs : f64, epsrel : f64) -> QngIntegratorResult
    {
        let mut result : f64 ;
        let mut abserr : f64;
        let mut neval :i32;


        if epsabs <= 0.0 && epsrel < 0.5e-28_f64.max(50.0 * EPMACH) {
            return QngIntegratorResult::new_error(ResultState::Invalid)
        }


        let hlgth : f64 = 0.5*(b-a);
        let dhlgth : f64 = hlgth.abs();
        let centr : f64 = 0.5 * (b+a);
        let fcentr : f64= f(centr);
        neval = 21;

//       compute the integral using the 10- and 21-point formula.

        let mut res10 : f64 = 0.0;
        let mut res21 : f64 = W21B[5] * fcentr;
        let mut resabs : f64 = W21B[5] * fcentr.abs();
        let mut savfun : Vec<f64> = vec![];
        let mut fv1 : Vec<f64> = vec![];
        let mut fv2 : Vec<f64> = vec![];
        let mut fv3 : Vec<f64> = vec![];
        let mut fv4 : Vec<f64> = vec![];

        for k in 0..5{
            let absc : f64 = hlgth * X1[k];
            let fval1 : f64 = f(centr+absc);
            let fval2 : f64 = f(centr-absc);
            let fval : f64 = fval1 + fval2;
            res10 += W10[k] * fval;
            res21 += W21A[k] * fval;
            resabs += W21A[k] * (fval1.abs() + fval2.abs());
            savfun.push(fval);
            fv1.push(fval1);
            fv2.push(fval2);

        }

        for k in 0..5{
            let absc : f64 = hlgth * X2[k];
            let fval1 : f64 = f(centr+absc);
            let fval2 : f64 = f(centr-absc);
            let fval : f64 = fval1+fval2;
            res21 += W21B[k] * fval;
            resabs += W21B[k] * (fval1.abs() + fval2.abs());
            savfun.push(fval);
            fv3.push(fval1);
            fv4.push(fval2);
        }

        result = res21 * hlgth;
        resabs = resabs * dhlgth;
        let reskh : f64 = 0.5 * res21;
        let mut resasc : f64 = W21B[5] * (fcentr-reskh).abs();
        for k in 0..5{
            resasc += W21A[k] * ( (fv1[k]-reskh).abs() + (fv2[k]-reskh).abs() )
                + W21B[k] * ( (fv3[k]-reskh).abs() + (fv4[k] - reskh).abs() );
        }

        abserr = ( (res21-res10) * hlgth).abs();
        resasc = resasc * dhlgth;

        if resasc != 0.0 && abserr != 0.0 {
            abserr = resasc * 1.0_f64.min((200.0 * abserr/resasc).powf(1.5));
        }
        if resabs >= UFLOW/(50.0 * EPMACH)  {
            abserr = abserr.max( EPMACH * 50.0 * resabs);
        }
        if abserr <= epsabs.max(epsrel * result.abs()) {
            //  let ten_millis = time::Duration::from_micros(10000);
            //  thread::sleep(ten_millis);
            return QngIntegratorResult::new(result,abserr,neval)
        }


        //         compute the integral using the 43-point formula.
        let mut res43 : f64 = W43B[11] * fcentr;
        neval = 43;

        for k in 0..10{
            res43 += savfun[k] * W43A[k]
        }

        for k in 0..11{
            let absc : f64 = hlgth * X3[k];
            let fval : f64 = f(absc + centr) + f(centr - absc);
            res43 += fval * W43B[k];
            savfun.push(fval);
        }

        result = res43 * hlgth;
        abserr = ((res43 - res21) * hlgth).abs();


        if resasc != 0.0 && abserr != 0.0 {
            abserr = resasc * 1.0_f64.min((200.0 * abserr/resasc).powf(1.5));
        }
        if resabs >= UFLOW/(50.0 * EPMACH)  {
            abserr = abserr.max( EPMACH * 50.0 * resabs);
        }

        if abserr <= epsabs.max(epsrel * result.abs()) {
            return QngIntegratorResult::new(result,abserr,neval)
        }


        //  compute the integral using the 87-point formula.

        let mut res87 : f64 = W87B[22] * fcentr;
        neval = 87;

        for k in 0..21{
            res87 += savfun[k] * W87A[k];
        }

        for k in 0..22{
            let absc : f64 = hlgth * X4[k];
            res87 += W87B[k] * (f(absc + centr) + f(centr - absc));
        }

        result = res87 * hlgth;
        abserr = ((res87-res43) * hlgth).abs();


        if resasc != 0.0 && abserr != 0.0 {
            abserr = resasc * 1.0_f64.min((200.0 * abserr/resasc).powf(1.5));
        }
        if resabs >= UFLOW/(50.0 * EPMACH)  {
            abserr = abserr.max( EPMACH * 50.0 * resabs);
        }

        if abserr <= epsabs.max(epsrel * result.abs()) {
            return QngIntegratorResult::new(result,abserr,neval)
        }



        return QngIntegratorResult::new_error(ResultState::Invalid)
    }
}

impl QuadIntegralMethod for Qng{
    fn integrate(&self,f : &dyn Fn(f64)->f64, a : f64, b : f64, epsabs : f64, epsrel : f64) -> QuadIntegratorResult{
        QuadIntegratorResult::new_qng( self.qintegrate(f,a,b,epsabs,epsrel))
    }
}







