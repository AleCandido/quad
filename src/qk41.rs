use crate::qk::*;

pub struct Qk41{}
///     Parameters:
///
///     On entry:
///         f   :   f64
///                 function
///
///         a   :   f64
///                 lower limit of integration
///
///         b   :   f64
///                 upper limit of integration
///
///     On return:
///         result  :   f64
///                     approximation to the integral i result is computed by applying
///                     the 41-point kronrod rule (resk) obtained by optimal addition
///                     of abscissae to the 20-point gauss rule(resg).
///
///         abserr  :   f64
///                     estimate of the modulus of the absolute error, which should not
///                     exceed abs(i-result)
///
///         resabs  :   f64
///                     approximation to the integral j
///
///         resasc  :   f64
///                     approximation to the integral of abs(f-i/(b-a)) over (a,b)
///
///     The abscissae and weights are given for the interval (-1,1).
///     Because of symmetry only the positive abscissae and their
///     corresponding weights are given.
///
///         xgk     :   abscissae of the 41-point kronrod rule
///                     xgk(2), xgk(4), ...  abscissae of the 20-point
///                     gauss rule
///                     xgk(1), xgk(3), ...  abscissae which are optimally
///                     added to the 20-point gauss rule
///
///         wgk     :   weights of the 41-point kronrod rule
///
///         wg      :   weights of the 20-point gauss rule
///
///
///     Gauss quadrature weights and kronrod quadrature abscissae and weights
///     as evaluated with 80 decimal digit arithmetic by l. w. fullerton,
///     bell labs, nov. 1981.
///
///
///

const XGK : [f64;21] = [0.998859031588277663838315576545863, 0.993128599185094924786122388471320,
                        0.981507877450250259193342994720217, 0.963971927277913791267666131197277,
                        0.940822633831754753519982722212443, 0.912234428251325905867752441203298,
                        0.878276811252281976077442995113078, 0.839116971822218823394529061701521,
                        0.795041428837551198350638833272788, 0.746331906460150792614305070355642,
                        0.693237656334751384805490711845932, 0.636053680726515025452836696226286,
                        0.575140446819710315342946036586425, 0.510867001950827098004364050955251,
                        0.443593175238725103199992213492640, 0.373706088715419560672548177024927,
                        0.301627868114913004320555356858592, 0.227785851141645078080496195368575,
                        0.152605465240922675505220241022678, 0.076526521133497333754640409398838,
                        0.000000000000000000000000000000000];

const WGK : [f64;21] = [0.003073583718520531501218293246031, 0.008600269855642942198661787950102,
                        0.014626169256971252983787960308868, 0.020388373461266523598010231432755,
                        0.025882133604951158834505067096153, 0.031287306777032798958543119323801,
                        0.036600169758200798030557240707211, 0.041668873327973686263788305936895,
                        0.046434821867497674720231880926108, 0.050944573923728691932707670050345,
                        0.055195105348285994744832372419777, 0.059111400880639572374967220648594,
                        0.062653237554781168025870122174255, 0.065834597133618422111563556969398,
                        0.068648672928521619345623411885368, 0.071054423553444068305790361723210,
                        0.073030690332786667495189417658913, 0.074582875400499188986581418362488,
                        0.075704497684556674659542775376617, 0.076377867672080736705502835038061,
                        0.076600711917999656445049901530102];

const WG : [f64;10] = [0.017614007139152118311861962351853, 0.040601429800386941331039952274932,
                       0.062672048334109063569506535187042, 0.083276741576704748724758143222046,
                       0.101930119817240435036750135480350, 0.118194531961518417312377377711382,
                       0.131688638449176626898494499748163, 0.142096109318382051329298325067165,
                       0.149172986472603746787828737001969, 0.152753387130725850698084331955098];

impl Qk for Qk41 {
    fn integrate(&self, f: &dyn Fn(f64) -> f64, a: f64, b: f64, ) -> (f64, f64, f64, f64) {
        let hlgth: f64 = 0.5 * (b - a);
        let dhlgth: f64 = hlgth.abs();
        let centr: f64 = 0.5 * (b + a);

        let mut fv1: Vec<f64> = vec![0.0; 20];
        let mut fv2: Vec<f64> = vec![0.0; 20];

        //compute the 41-point kronrod approximation to
        //the integral, and estimate the absolute error.

        let mut resg = 0.0;
        let fc : f64 = f(centr);
        let mut resk = WGK[20] * fc;
        let mut resabs = resk.abs();

        for j in 1..11 {
            let jtw = 2 * j;
            let absc = hlgth * XGK[jtw - 1];
            let fval1 : f64 = f(centr - absc);
            let fval2 : f64 = f(centr + absc);
            fv1[jtw - 1] = fval1;
            fv2[jtw - 1] = fval2;
            let fsum = fval1 + fval2;
            resg += WG[j - 1] * fsum;
            resk += WGK[jtw - 1] * fsum;
            resabs += WGK[jtw - 1] * (fval1.abs() + fval2.abs());
        }

        for j in 1..11 {
            let jtwm1 = 2 * j - 1;
            let absc = hlgth * XGK[jtwm1 - 1];
            let fval1 : f64 = f(centr - absc);
            let fval2 : f64 = f(centr + absc);
            fv1[jtwm1 - 1] = fval1;
            fv2[jtwm1 - 1] = fval2;
            let fsum = fval1 + fval2;
            resk += WGK[jtwm1 - 1] * fsum;
            resabs += WGK[jtwm1 - 1] * (fval1.abs() + fval2.abs());
        }

        let reskh = resk * 0.5;
        let mut resasc = WGK[20] * (fc - reskh).abs();

        for j in 1..21 {
            resasc += WGK[j - 1] * ((fv1[j - 1] - reskh).abs() + (fv2[j - 1] - reskh).abs());
        }

        let result = resk * hlgth;
        resabs = resabs * dhlgth;
        resasc = resasc * dhlgth;
        let mut abserr = ((resk - resg) * hlgth).abs();
        if resasc != 0.0 && abserr != 0.0 {
            abserr = resasc * 1.0_f64.min((200.0 * abserr / resasc).powf(1.5));
        }
        if resabs > UFLOW / (50.0 * EPMACH) {
            abserr = abserr.max((EPMACH * 50.0) * resabs);
        }

        (result, abserr, resabs, resasc)
    }
}
