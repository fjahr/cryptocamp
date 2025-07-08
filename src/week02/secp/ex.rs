use num_bigint::BigUint;
use std::str::FromStr;
use num_traits::{Num, Zero, One};

fn ex1_test_vectors() -> Vec<((BigUint, BigUint), (BigUint, BigUint), (BigUint, BigUint))> {
    vec![
        (
            (BigUint::from_str("67021774492365321256634043516869791044054964063002935266026048760627130221114").unwrap(), BigUint::from_str("22817883221438079958217963063610327523693969913024717835557258242342029550595").unwrap()),
            (BigUint::from_str("61124938217888369397608518626468079588341162087856379517664485009963441753645").unwrap(), BigUint::from_str("5723382937169086635766392599511664586625983027860520036338464885987365575658").unwrap()),
            (BigUint::from_str("78518484088348927894279633941273782106215956054783044881924083038087974375069").unwrap(), BigUint::from_str("18400956471605157290158330638123206056219981947313880254846397293938760781200").unwrap())
        ),
        (
            (BigUint::from_str("44797955726860071483167773525787460171685721903803276437396496681708013097206").unwrap(), BigUint::from_str("112878323467240798018200025047246733779416351939079609883282945822975931592141").unwrap()),
            (BigUint::from_str("44797955726860071483167773525787460171685721903803276437396496681708013097206").unwrap(), BigUint::from_str("2913765770075397405370959961441174073853632726560954156174638184932903079522").unwrap()),
            (BigUint::zero(), BigUint::zero())
        ),
        (
            (BigUint::from_str("95200151225387174391707134980196577229773167465894787919263504089948495725202").unwrap(), BigUint::from_str("94213123740092242124032541289267941722641721980066680728855126898974205181980").unwrap()),
            (BigUint::from_str("95200151225387174391707134980196577229773167465894787919263504089948495725202").unwrap(), BigUint::from_str("94213123740092242124032541289267941722641721980066680728855126898974205181980").unwrap()),
            (BigUint::from_str("5909177817561749019375996132097716007690336893057112295739767175467136927121").unwrap(), BigUint::from_str("32162989297956602751967132742255814558956860587998309119003795115938320862381").unwrap())
        ),
        (
            (BigUint::from_str("24050370140998638157368766089090079788245793492514664296883668741529047882113").unwrap(), BigUint::from_str("14478882322437672032054487172211630444001167135141445302555096737662467817571").unwrap()),
            (BigUint::from_str("15045863282447234231848775263852322721143017336655001075698483887751182719636").unwrap(), BigUint::from_str("14478882322437672032054487172211630444001167135141445302555096737662467817571").unwrap()),
            (BigUint::from_str("76695855813870323034353443655745505343881173836470898666875431378628604069914").unwrap(), BigUint::from_str("101313206914878523391516497836476277409268817530499118736902487270246366854092").unwrap())
        ),
        (
            (BigUint::from_str("14256779447437936128616290794341059890063336098474125854681710102809814868320").unwrap(), BigUint::from_str("90566103014364716248988921534849031279541603477816641946022463390335657035131").unwrap()),
            (BigUint::from_str("2303067510121489830312323422056091166740725427601969990117485452141659178613").unwrap(), BigUint::from_str("25225986222951479174582063473838876573728381187823922093435120617573177636532").unwrap()),
            (BigUint::from_str("95430772898311369787541983276504378677140303663720683940530878996024106515165").unwrap(), BigUint::from_str("48068184564993462938397020947826677061288691733511084479824032705110581338856").unwrap())
        )
    ]
}

fn get_p() -> BigUint {
    BigUint::from_str_radix("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap()
}

fn point_add(p_point: (BigUint, BigUint), q_point: (BigUint, BigUint)) -> (BigUint, BigUint) {
    let p = get_p();
    
    // If either one are point at infinity, return the other one
    if p_point.0 == BigUint::zero() && p_point.1 == BigUint::zero() {
        return q_point;
    }
    if q_point.0 == BigUint::zero() && q_point.1 == BigUint::zero() {
        return p_point;
    }
    
    let slope;
    
    // Same x coordinate
    if p_point.0 == q_point.0 {
        // Points are inverses
        if p_point.1 != q_point.1 {
            return (BigUint::zero(), BigUint::zero());
        }
        
        // Point doubling case
        let three = BigUint::from(3u32);
        let two = BigUint::from(2u32);
        
        let numerator = (three * &p_point.0 * &p_point.0) % &p;
        let denominator = (two * &p_point.1) % &p;
        
        let inv_denominator = mod_inverse(&denominator, &p);
        slope = (numerator * inv_denominator) % &p;
    } else {
        // Regular point addition
        let numerator = if q_point.1 >= p_point.1 {
            &q_point.1 - &p_point.1
        } else {
            &p + &q_point.1 - &p_point.1
        };
        
        let denominator = if q_point.0 >= p_point.0 {
            &q_point.0 - &p_point.0
        } else {
            &p + &q_point.0 - &p_point.0
        };
        
        let inv_denominator = mod_inverse(&denominator, &p);
        slope = (numerator * inv_denominator) % &p;
    }
    
    let slope_squared = (&slope * &slope) % &p;
    let x = ((slope_squared + &p - &p_point.0) % &p + &p - &q_point.0) % &p;
    
    let px_minus_x = if p_point.0 >= x {
        &p_point.0 - &x
    } else {
        &p + &p_point.0 - &x
    };
    let y = ((&slope * px_minus_x) % &p + &p - &p_point.1) % &p;
    
    (x, y)
}

fn mod_inverse(a: &BigUint, p: &BigUint) -> BigUint {
    let p_minus_2 = p - BigUint::from(2u32);
    // Same logic as week1 ex3
    a.modpow(&p_minus_2, p)
}

fn scalar_mult(scalar: BigUint, point: (BigUint, BigUint)) -> (BigUint, BigUint) {
    if scalar == BigUint::zero() {
        return (BigUint::zero(), BigUint::zero());
    }
    
    if point.0 == BigUint::zero() && point.1 == BigUint::zero() {
        return point;
    }
    
    let mut result = (BigUint::zero(), BigUint::zero());
    let mut temp = point.clone();
    let mut k = scalar;
    
    // Similar to week1 ex1
    while k > BigUint::zero() {
        if &k & BigUint::one() == BigUint::one() {
            result = point_add(result, temp.clone());
        }
        temp = point_add(temp.clone(), temp.clone());
        k >>= 1;
    }
    
    result
}

fn ex3_test_vectors() -> Vec<(BigUint, (BigUint, BigUint), (BigUint, BigUint))> {
    vec![
        (
            BigUint::from_str("23529072936145521956642440150769408702836782170707519110832596096096916532363").unwrap(),
            (BigUint::from_str("94777218176490725267733209794395406270863807953747235979017564313980479098344").unwrap(), BigUint::from_str("53121120406880321033414824968851949358991212541220678285657788880408683486672").unwrap()),
            (BigUint::from_str("81492582484984365721511233996054540050314813088236204730182464710703690737195").unwrap(), BigUint::from_str("84165397430175583340352582740254662715932722835371860159802475562062898918484").unwrap())
        ),
        (
            BigUint::from_str("77770687059601253501098075906318324640585620643934538062621691587089455400301").unwrap(),
            (BigUint::from_str("5187380010089560191829928600869675928625207216422014112981972591844926771008").unwrap(), BigUint::from_str("75026050083095897004323393777174635055491620440662638678606562665317466685019").unwrap()),
            (BigUint::from_str("76999255841974189685876230118581110410155956505185745130247574937430232984638").unwrap(), BigUint::from_str("87571171775685157828750403037960903210473289232782306139148947195874900187006").unwrap())
        ),
        (
            BigUint::from_str("3747619523960563074315083315669137577217731866086110333821423552891044218266").unwrap(),
            (BigUint::from_str("66371586610273545144505648512343824229224003523952192165787799288317344396675").unwrap(), BigUint::from_str("6489011411151914877089190610663845093649879070897583530615192453262848111419").unwrap()),
            (BigUint::from_str("109441138145498884726545575659592733193661671281368885246963601136369148387669").unwrap(), BigUint::from_str("83708880322787879701338478937074052809697986569225329829504559758598509123336").unwrap())
        )
    ]
}

fn get_g() -> (BigUint, BigUint) {
    (
        BigUint::from_str_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap(),
        BigUint::from_str_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap(),
    )
}

fn get_lambda() -> BigUint {
   BigUint::from_str_radix("5363ad4cc05c30e0a5261c028812645a122e22ea20816678df02967c1b23bd72", 16).unwrap()
}

fn get_n() -> BigUint {
   BigUint::from_str_radix("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap()
}

fn point_negate(point: (BigUint, BigUint)) -> (BigUint, BigUint) {
    let p = get_p();
    if point.0 == BigUint::zero() && point.1 == BigUint::zero() {
        return point;
    }
    (point.0, &p - &point.1)
}

fn point_subtract(p_point: (BigUint, BigUint), q_point: (BigUint, BigUint)) -> (BigUint, BigUint) {
    point_add(p_point, point_negate(q_point))
}

fn solve_for_q() -> (BigUint, BigUint) {
    let p = get_p();
    let n = get_n();
    let g = get_g();
    let lambda = get_lambda();
    
    // (p+2)/9
    let p_plus_2 = &p + BigUint::from(2u32);
    let nine = BigUint::from(9u32);
    let exponent = &p_plus_2 / &nine;
    
    // -6 mod p = p - 6
    let minus_six = &p - BigUint::from(6u32);
    
    // x = (-6)^((p+2)/9) mod p
    let x = minus_six.modpow(&exponent, &p);
    
    // P
    let p_point = (x.clone(), BigUint::one());
    
    // 5 * G
    let five_g = scalar_mult(BigUint::from(5u32), g);
    
    // lambda * P
    let lambda_p = scalar_mult(lambda, p_point);
    
    // 5 * G - lambda * P
    let right_side = point_subtract(five_g, lambda_p);
    
    // Divide right side by 3
    let three = BigUint::from(3u32);
    let three_inv = mod_inverse(&three, &n);
    
    // Q
    scalar_mult(three_inv, right_side)
}

fn main() {
    // Exercise 1
    for (i, v) in ex1_test_vectors().iter().enumerate() {
        let result = point_add(v.0.clone(), v.1.clone());
        assert_eq!(result, v.2, "Test {} failed", i + 1);
    }

    // Exercise 2
    // TODO
    
    // Exercise 3
    for (i, v) in ex3_test_vectors().iter().enumerate() {
        let result = scalar_mult(v.0.clone(), v.1.clone());
        assert_eq!(result, v.2, "Test {} failed", i + 1);
    }

    // Exercise 4
    let q = solve_for_q();
    println!("Q = ({}, {})", q.0, q.1);

    // Exercise 5
    // TODO

    println!("Success!");
}
