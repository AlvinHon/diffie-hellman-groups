use std::fmt::Debug;

use lazy_static::lazy_static;
use num_bigint::BigUint;

/// Trait of the Modular Exponential (MODP) Groups for the Internet Key Exchange (IKE) protocol.
pub trait MODPGroup: Debug {
    /// prime modulus
    fn prime_modulus() -> BigUint;

    /// The Sophie Germain Prime, i.e. q
    fn sophie_garmain_prime() -> BigUint;

    /// generator of the subgroup of order 2q
    fn generator() -> BigUint;

    /// modular addition, compute a + b mod p
    fn add(a: &BigUint, b: &BigUint) -> BigUint {
        (a + b) % Self::prime_modulus()
    }

    /// modular subtraction, compute a - b mod p
    fn sub(a: &BigUint, b: &BigUint) -> BigUint {
        (a + Self::prime_modulus() - b) % Self::prime_modulus()
    }

    /// modular multiplication, compute a * b mod p
    fn mul(a: &BigUint, b: &BigUint) -> BigUint {
        (a * b) % Self::prime_modulus()
    }

    /// modular exponentiation, compute a^e mod p
    fn pow(a: &BigUint, e: &BigUint) -> BigUint;

    /// compute the element of the group from the exponent, compute g^e mod p
    fn element(exponent: &BigUint) -> BigUint {
        Self::pow(&Self::generator(), exponent)
    }
}

/// Implementations of the MODPGroup trait for the MODP groups defined in RFC 3526.
///
/// The 1536 bit MODP group has been used for the implementations for
/// quite a long time, but was not defined in RFC 2409 (IKE).
/// Implementations have been using group 5 to designate this group, we
/// standardize that practice here.
///
/// The prime is: 2^1536 - 2^1472 - 1 + 2^64 * { \[2^1406 pi\] + 741804 }
#[derive(Debug)]
pub struct MODPGroup5;

impl MODPGroup for MODPGroup5 {
    fn prime_modulus() -> BigUint {
        PRIME_GROUP_5.clone()
    }

    fn sophie_garmain_prime() -> BigUint {
        Q_GROUP_5.clone()
    }

    fn generator() -> BigUint {
        BigUint::from(2u32)
    }

    fn pow(a: &BigUint, e: &BigUint) -> BigUint {
        a.modpow(e, &PRIME_GROUP_5)
    }
}

/// Implementations of the MODPGroup trait for the MODP groups defined in RFC 3526.
///
/// 2048-bit MODP Group
/// This group is assigned id 14.
///
/// This prime is: 2^2048 - 2^1984 - 1 + 2^64 * { \[2^1918 pi\] + 124476 }
#[derive(Debug)]
pub struct MODPGroup14;

impl MODPGroup for MODPGroup14 {
    fn prime_modulus() -> BigUint {
        PRIME_GROUP_14.clone()
    }

    fn sophie_garmain_prime() -> BigUint {
        Q_GROUP_14.clone()
    }

    fn generator() -> BigUint {
        BigUint::from(2u32)
    }

    fn pow(a: &BigUint, e: &BigUint) -> BigUint {
        a.modpow(e, &PRIME_GROUP_14)
    }
}

/// Implementations of the MODPGroup trait for the MODP groups defined in RFC 3526.
///
/// 3072-bit MODP Group
/// This group is assigned id 15.
///
/// This prime is: 2^3072 - 2^3008 - 1 + 2^64 * { \[2^2942 pi\] + 1690314 }
#[derive(Debug)]
pub struct MODPGroup15;

impl MODPGroup for MODPGroup15 {
    fn prime_modulus() -> BigUint {
        PRIME_GROUP_15.clone()
    }

    fn sophie_garmain_prime() -> BigUint {
        Q_GROUP_15.clone()
    }

    fn generator() -> BigUint {
        BigUint::from(2u32)
    }

    fn pow(a: &BigUint, e: &BigUint) -> BigUint {
        a.modpow(e, &PRIME_GROUP_15)
    }
}

/// Implementations of the MODPGroup trait for the MODP groups defined in RFC 3526.
///
/// 4096-bit MODP Group
/// This group is assigned id 16.
///
/// This prime is: 2^4096 - 2^4032 - 1 + 2^64 * { \[2^3966 pi\] + 240904 }
#[derive(Debug)]
pub struct MODPGroup16;

impl MODPGroup for MODPGroup16 {
    fn prime_modulus() -> BigUint {
        PRIME_GROUP_16.clone()
    }

    fn sophie_garmain_prime() -> BigUint {
        Q_GROUP_16.clone()
    }

    fn generator() -> BigUint {
        BigUint::from(2u32)
    }

    fn pow(a: &BigUint, e: &BigUint) -> BigUint {
        a.modpow(e, &PRIME_GROUP_16)
    }
}

/// Implementations of the MODPGroup trait for the MODP groups defined in RFC 3526.
///
/// 6144-bit MODP Group
/// This group is assigned id 17.
///
/// This prime is: 2^6144 - 2^6080 - 1 + 2^64 * { \[2^6014 pi\] + 929484 }
#[derive(Debug)]
pub struct MODPGroup17;

impl MODPGroup for MODPGroup17 {
    fn prime_modulus() -> BigUint {
        PRIME_GROUP_17.clone()
    }

    fn sophie_garmain_prime() -> BigUint {
        Q_GROUP_17.clone()
    }

    fn generator() -> BigUint {
        BigUint::from(2u32)
    }

    fn pow(a: &BigUint, e: &BigUint) -> BigUint {
        a.modpow(e, &PRIME_GROUP_17)
    }
}

/// Implementations of the MODPGroup trait for the MODP groups defined in RFC 3526.
///
/// 8192-bit MODP Group
/// This group is assigned id 18.
///
/// This prime is: 2^8192 - 2^8128 - 1 + 2^64 * { \[2^8062 pi\] + 4743158 }
#[derive(Debug)]
pub struct MODPGroup18;

impl MODPGroup for MODPGroup18 {
    fn prime_modulus() -> BigUint {
        PRIME_GROUP_18.clone()
    }

    fn sophie_garmain_prime() -> BigUint {
        Q_GROUP_18.clone()
    }

    fn generator() -> BigUint {
        BigUint::from(2u32)
    }

    fn pow(a: &BigUint, e: &BigUint) -> BigUint {
        a.modpow(e, &PRIME_GROUP_18)
    }
}

lazy_static! {
    static ref PRIME_GROUP_5: BigUint = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD1\
        29024E088A67CC74020BBEA63B139B22514A08798E3404DD\
        EF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245\
        E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7ED\
        EE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3D\
        C2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F\
        83655D23DCA3AD961C62F356208552BB9ED529077096966D\
        670C354E4ABC9804F1746C08CA237327FFFFFFFFFFFFFFFF",
        16,
    )
    .unwrap();
    static ref Q_GROUP_5: BigUint = BigUint::parse_bytes(
        b"7FFFFFFFFFFFFFFFE487ED5110B4611A62633145C06E0E68\
        948127044533E63A0105DF531D89CD9128A5043CC71A026E\
        F7CA8CD9E69D218D98158536F92F8A1BA7F09AB6B6A8E122\
        F242DABB312F3F637A262174D31BF6B585FFAE5B7A035BF6\
        F71C35FDAD44CFD2D74F9208BE258FF324943328F6722D9E\
        E1003E5C50B1DF82CC6D241B0E2AE9CD348B1FD47E9267AF\
        C1B2AE91EE51D6CB0E3179AB1042A95DCF6A9483B84B4B36\
        B3861AA7255E4C0278BA36046511B993FFFFFFFFFFFFFFFF",
        16
    )
    .unwrap();
    static ref PRIME_GROUP_14: BigUint = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD1\
        29024E088A67CC74020BBEA63B139B22514A08798E3404DD\
        EF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245\
        E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7ED\
        EE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3D\
        C2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F\
        83655D23DCA3AD961C62F356208552BB9ED529077096966D\
        670C354E4ABC9804F1746C08CA18217C32905E462E36CE3B\
        E39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9\
        DE2BCBF6955817183995497CEA956AE515D2261898FA0510\
        15728E5A8AACAA68FFFFFFFFFFFFFFFF",
        16,
    )
    .unwrap();
    static ref Q_GROUP_14: BigUint = BigUint::parse_bytes(
        b"7FFFFFFFFFFFFFFFE487ED5110B4611A62633145C06E0E68\
        948127044533E63A0105DF531D89CD9128A5043CC71A026E\
        F7CA8CD9E69D218D98158536F92F8A1BA7F09AB6B6A8E122\
        F242DABB312F3F637A262174D31BF6B585FFAE5B7A035BF6\
        F71C35FDAD44CFD2D74F9208BE258FF324943328F6722D9E\
        E1003E5C50B1DF82CC6D241B0E2AE9CD348B1FD47E9267AF\
        C1B2AE91EE51D6CB0E3179AB1042A95DCF6A9483B84B4B36\
        B3861AA7255E4C0278BA3604650C10BE19482F23171B671D\
        F1CF3B960C074301CD93C1D17603D147DAE2AEF837A62964\
        EF15E5FB4AAC0B8C1CCAA4BE754AB5728AE9130C4C7D0288\
        0AB9472D455655347FFFFFFFFFFFFFFF",
        16
    )
    .unwrap();
    static ref PRIME_GROUP_15: BigUint = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD1\
        29024E088A67CC74020BBEA63B139B22514A08798E3404DD\
        EF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245\
        E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7ED\
        EE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3D\
        C2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F\
        83655D23DCA3AD961C62F356208552BB9ED529077096966D\
        670C354E4ABC9804F1746C08CA18217C32905E462E36CE3B\
        E39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9\
        DE2BCBF6955817183995497CEA956AE515D2261898FA0510\
        15728E5A8AAAC42DAD33170D04507A33A85521ABDF1CBA64\
        ECFB850458DBEF0A8AEA71575D060C7DB3970F85A6E1E4C7\
        ABF5AE8CDB0933D71E8C94E04A25619DCEE3D2261AD2EE6B\
        F12FFA06D98A0864D87602733EC86A64521F2B18177B200C\
        BBE117577A615D6C770988C0BAD946E208E24FA074E5AB31\
        43DB5BFCE0FD108E4B82D120A93AD2CAFFFFFFFFFFFFFFFF",
        16,
    )
    .unwrap();
    static ref PRIME_GROUP_16: BigUint = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD1\
        29024E088A67CC74020BBEA63B139B22514A08798E3404DD\
        EF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245\
        E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7ED\
        EE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3D\
        C2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F\
        83655D23DCA3AD961C62F356208552BB9ED529077096966D\
        670C354E4ABC9804F1746C08CA18217C32905E462E36CE3B\
        E39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9\
        DE2BCBF6955817183995497CEA956AE515D2261898FA0510\
        15728E5A8AAAC42DAD33170D04507A33A85521ABDF1CBA64\
        ECFB850458DBEF0A8AEA71575D060C7DB3970F85A6E1E4C7\
        ABF5AE8CDB0933D71E8C94E04A25619DCEE3D2261AD2EE6B\
        F12FFA06D98A0864D87602733EC86A64521F2B18177B200C\
        BBE117577A615D6C770988C0BAD946E208E24FA074E5AB31\
        43DB5BFCE0FD108E4B82D120A92108011A723C12A787E6D7\
        88719A10BDBA5B2699C327186AF4E23C1A946834B6150BDA\
        2583E9CA2AD44CE8DBBBC2DB04DE8EF92E8EFC141FBECAA6\
        287C59474E6BC05D99B2964FA090C3A2233BA186515BE7ED\
        1F612970CEE2D7AFB81BDD762170481CD0069127D5B05AA9\
        93B4EA988D8FDDC186FFB7DC90A6C08F4DF435C934063199\
        FFFFFFFFFFFFFFFF",
        16,
    )
    .unwrap();
    static ref Q_GROUP_15: BigUint = BigUint::parse_bytes(
        b"7FFFFFFFFFFFFFFFE487ED5110B4611A62633145C06E0E68\
        948127044533E63A0105DF531D89CD9128A5043CC71A026E\
        F7CA8CD9E69D218D98158536F92F8A1BA7F09AB6B6A8E122\
        F242DABB312F3F637A262174D31BF6B585FFAE5B7A035BF6\
        F71C35FDAD44CFD2D74F9208BE258FF324943328F6722D9E\
        E1003E5C50B1DF82CC6D241B0E2AE9CD348B1FD47E9267AF\
        C1B2AE91EE51D6CB0E3179AB1042A95DCF6A9483B84B4B36\
        B3861AA7255E4C0278BA3604650C10BE19482F23171B671D\
        F1CF3B960C074301CD93C1D17603D147DAE2AEF837A62964\
        EF15E5FB4AAC0B8C1CCAA4BE754AB5728AE9130C4C7D0288\
        0AB9472D45556216D6998B8682283D19D42A90D5EF8E5D32\
        767DC2822C6DF785457538ABAE83063ED9CB87C2D370F263\
        D5FAD7466D8499EB8F464A702512B0CEE771E9130D697735\
        F897FD036CC504326C3B01399F643532290F958C0BBD9006\
        5DF08BABBD30AEB63B84C4605D6CA371047127D03A72D598\
        A1EDADFE707E884725C16890549D69657FFFFFFFFFFFFFFF",
        16
    )
    .unwrap();
    static ref Q_GROUP_16: BigUint = BigUint::parse_bytes(
        b"7FFFFFFFFFFFFFFFE487ED5110B4611A62633145C06E0E68\
        948127044533E63A0105DF531D89CD9128A5043CC71A026E\
        F7CA8CD9E69D218D98158536F92F8A1BA7F09AB6B6A8E122\
        F242DABB312F3F637A262174D31BF6B585FFAE5B7A035BF6\
        F71C35FDAD44CFD2D74F9208BE258FF324943328F6722D9E\
        E1003E5C50B1DF82CC6D241B0E2AE9CD348B1FD47E9267AF\
        C1B2AE91EE51D6CB0E3179AB1042A95DCF6A9483B84B4B36\
        B3861AA7255E4C0278BA3604650C10BE19482F23171B671D\
        F1CF3B960C074301CD93C1D17603D147DAE2AEF837A62964\
        EF15E5FB4AAC0B8C1CCAA4BE754AB5728AE9130C4C7D0288\
        0AB9472D45556216D6998B8682283D19D42A90D5EF8E5D32\
        767DC2822C6DF785457538ABAE83063ED9CB87C2D370F263\
        D5FAD7466D8499EB8F464A702512B0CEE771E9130D697735\
        F897FD036CC504326C3B01399F643532290F958C0BBD9006\
        5DF08BABBD30AEB63B84C4605D6CA371047127D03A72D598\
        A1EDADFE707E884725C16890549084008D391E0953C3F36B\
        C438CD085EDD2D934CE1938C357A711E0D4A341A5B0A85ED\
        12C1F4E5156A26746DDDE16D826F477C97477E0A0FDF6553\
        143E2CA3A735E02ECCD94B27D04861D1119DD0C328ADF3F6\
        8FB094B867716BD7DC0DEEBB10B8240E68034893EAD82D54\
        C9DA754C46C7EEE0C37FDBEE48536047A6FA1AE49A0318CC\
        FFFFFFFFFFFFFFFF",
        16
    )
    .unwrap();
    static ref PRIME_GROUP_17: BigUint = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E08\
        8A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B\
        302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9\
        A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE6\
        49286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8\
        FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D\
        670C354E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772C\
        180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF695581718\
        3995497CEA956AE515D2261898FA051015728E5A8AAAC42DAD33170D\
        04507A33A85521ABDF1CBA64ECFB850458DBEF0A8AEA71575D060C7D\
        B3970F85A6E1E4C7ABF5AE8CDB0933D71E8C94E04A25619DCEE3D226\
        1AD2EE6BF12FFA06D98A0864D87602733EC86A64521F2B18177B200C\
        BBE117577A615D6C770988C0BAD946E208E24FA074E5AB3143DB5BFC\
        E0FD108E4B82D120A92108011A723C12A787E6D788719A10BDBA5B26\
        99C327186AF4E23C1A946834B6150BDA2583E9CA2AD44CE8DBBBC2DB\
        04DE8EF92E8EFC141FBECAA6287C59474E6BC05D99B2964FA090C3A2\
        233BA186515BE7ED1F612970CEE2D7AFB81BDD762170481CD0069127\
        D5B05AA993B4EA988D8FDDC186FFB7DC90A6C08F4DF435C934028492\
        36C3FAB4D27C7026C1D4DCB2602646DEC9751E763DBA37BDF8FF9406\
        AD9E530EE5DB382F413001AEB06A53ED9027D831179727B0865A8918\
        DA3EDBEBCF9B14ED44CE6CBACED4BB1BDB7F1447E6CC254B33205151\
        2BD7AF426FB8F401378CD2BF5983CA01C64B92ECF032EA15D1721D03\
        F482D7CE6E74FEF6D55E702F46980C82B5A84031900B1C9E59E7C97F\
        BEC7E8F323A97A7E36CC88BE0F1D45B7FF585AC54BD407B22B4154AA\
        CC8F6D7EBF48E1D814CC5ED20F8037E0A79715EEF29BE32806A1D58B\
        B7C5DA76F550AA3D8A1FBFF0EB19CCB1A313D55CDA56C9EC2EF29632\
        387FE8D76E3C0468043E8F663F4860EE12BF2D5B0B7474D6E694F91E\
        6DCC4024FFFFFFFFFFFFFFFF",
        16,
    )
    .unwrap();
    static ref Q_GROUP_17: BigUint = BigUint::parse_bytes(
        b"7FFFFFFFFFFFFFFFE487ED5110B4611A62633145C06E0E68\
        948127044533E63A0105DF531D89CD9128A5043CC71A026E\
        F7CA8CD9E69D218D98158536F92F8A1BA7F09AB6B6A8E122\
        F242DABB312F3F637A262174D31BF6B585FFAE5B7A035BF6\
        F71C35FDAD44CFD2D74F9208BE258FF324943328F6722D9E\
        E1003E5C50B1DF82CC6D241B0E2AE9CD348B1FD47E9267AF\
        C1B2AE91EE51D6CB0E3179AB1042A95DCF6A9483B84B4B36\
        B3861AA7255E4C0278BA3604650C10BE19482F23171B671D\
        F1CF3B960C074301CD93C1D17603D147DAE2AEF837A62964\
        EF15E5FB4AAC0B8C1CCAA4BE754AB5728AE9130C4C7D0288\
        0AB9472D45556216D6998B8682283D19D42A90D5EF8E5D32\
        767DC2822C6DF785457538ABAE83063ED9CB87C2D370F263\
        D5FAD7466D8499EB8F464A702512B0CEE771E9130D697735\
        F897FD036CC504326C3B01399F643532290F958C0BBD9006\
        5DF08BABBD30AEB63B84C4605D6CA371047127D03A72D598\
        A1EDADFE707E884725C16890549084008D391E0953C3F36B\
        C438CD085EDD2D934CE1938C357A711E0D4A341A5B0A85ED\
        12C1F4E5156A26746DDDE16D826F477C97477E0A0FDF6553\
        143E2CA3A735E02ECCD94B27D04861D1119DD0C328ADF3F6\
        8FB094B867716BD7DC0DEEBB10B8240E68034893EAD82D54\
        C9DA754C46C7EEE0C37FDBEE48536047A6FA1AE49A014249\
        1B61FD5A693E381360EA6E593013236F64BA8F3B1EDD1BDE\
        FC7FCA0356CF298772ED9C17A09800D7583529F6C813EC18\
        8BCB93D8432D448C6D1F6DF5E7CD8A76A267365D676A5D8D\
        EDBF8A23F36612A5999028A895EBD7A137DC7A009BC6695F\
        ACC1E500E325C9767819750AE8B90E81FA416BE7373A7F7B\
        6AAF3817A34C06415AD42018C8058E4F2CF3E4BFDF63F479\
        91D4BD3F1B66445F078EA2DBFFAC2D62A5EA03D915A0AA55\
        6647B6BF5FA470EC0A662F6907C01BF053CB8AF7794DF194\
        0350EAC5DBE2ED3B7AA8551EC50FDFF8758CE658D189EAAE\
        6D2B64F617794B191C3FF46BB71E0234021F47B31FA43077\
        095F96AD85BA3A6B734A7C8F36E620127FFFFFFFFFFFFFFF",
        16
    )
    .unwrap();
    static ref PRIME_GROUP_18: BigUint = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD1\
        29024E088A67CC74020BBEA63B139B22514A08798E3404DD\
        EF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245\
        E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7ED\
        EE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3D\
        C2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F\
        83655D23DCA3AD961C62F356208552BB9ED529077096966D\
        670C354E4ABC9804F1746C08CA18217C32905E462E36CE3B\
        E39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9\
        DE2BCBF6955817183995497CEA956AE515D2261898FA0510\
        15728E5A8AAAC42DAD33170D04507A33A85521ABDF1CBA64\
        ECFB850458DBEF0A8AEA71575D060C7DB3970F85A6E1E4C7\
        ABF5AE8CDB0933D71E8C94E04A25619DCEE3D2261AD2EE6B\
        F12FFA06D98A0864D87602733EC86A64521F2B18177B200C\
        BBE117577A615D6C770988C0BAD946E208E24FA074E5AB31\
        43DB5BFCE0FD108E4B82D120A92108011A723C12A787E6D7\
        88719A10BDBA5B2699C327186AF4E23C1A946834B6150BDA\
        2583E9CA2AD44CE8DBBBC2DB04DE8EF92E8EFC141FBECAA6\
        287C59474E6BC05D99B2964FA090C3A2233BA186515BE7ED\
        1F612970CEE2D7AFB81BDD762170481CD0069127D5B05AA9\
        93B4EA988D8FDDC186FFB7DC90A6C08F4DF435C934028492\
        36C3FAB4D27C7026C1D4DCB2602646DEC9751E763DBA37BD\
        F8FF9406AD9E530EE5DB382F413001AEB06A53ED9027D831\
        179727B0865A8918DA3EDBEBCF9B14ED44CE6CBACED4BB1B\
        DB7F1447E6CC254B332051512BD7AF426FB8F401378CD2BF\
        5983CA01C64B92ECF032EA15D1721D03F482D7CE6E74FEF6\
        D55E702F46980C82B5A84031900B1C9E59E7C97FBEC7E8F3\
        23A97A7E36CC88BE0F1D45B7FF585AC54BD407B22B4154AA\
        CC8F6D7EBF48E1D814CC5ED20F8037E0A79715EEF29BE328\
        06A1D58BB7C5DA76F550AA3D8A1FBFF0EB19CCB1A313D55C\
        DA56C9EC2EF29632387FE8D76E3C0468043E8F663F4860EE\
        12BF2D5B0B7474D6E694F91E6DBE115974A3926F12FEE5E4\
        38777CB6A932DF8CD8BEC4D073B931BA3BC832B68D9DD300\
        741FA7BF8AFC47ED2576F6936BA424663AAB639C5AE4F568\
        3423B4742BF1C978238F16CBE39D652DE3FDB8BEFC848AD9\
        22222E04A4037C0713EB57A81A23F0C73473FC646CEA306B\
        4BCBC8862F8385DDFA9D4B7FA2C087E879683303ED5BDD3A\
        062B3CF5B3A278A66D2A13F83F44F82DDF310EE074AB6A36\
        4597E899A0255DC164F31CC50846851DF9AB48195DED7EA1\
        B1D510BD7EE74D73FAF36BC31ECFA268359046F4EB879F92\
        4009438B481C6CD7889A002ED5EE382BC9190DA6FC026E47\
        9558E4475677E9AA9E3050E2765694DFC81F56E880B96E71\
        60C980DD98EDD3DFFFFFFFFFFFFFFFFF",
        16,
    )
    .unwrap();
    static ref Q_GROUP_18: BigUint = BigUint::parse_bytes(
        b"7FFFFFFFFFFFFFFFE487ED5110B4611A62633145C06E0E68\
        948127044533E63A0105DF531D89CD9128A5043CC71A026E\
        F7CA8CD9E69D218D98158536F92F8A1BA7F09AB6B6A8E122\
        F242DABB312F3F637A262174D31BF6B585FFAE5B7A035BF6\
        F71C35FDAD44CFD2D74F9208BE258FF324943328F6722D9E\
        E1003E5C50B1DF82CC6D241B0E2AE9CD348B1FD47E9267AF\
        C1B2AE91EE51D6CB0E3179AB1042A95DCF6A9483B84B4B36\
        B3861AA7255E4C0278BA3604650C10BE19482F23171B671D\
        F1CF3B960C074301CD93C1D17603D147DAE2AEF837A62964\
        EF15E5FB4AAC0B8C1CCAA4BE754AB5728AE9130C4C7D0288\
        0AB9472D45556216D6998B8682283D19D42A90D5EF8E5D32\
        767DC2822C6DF785457538ABAE83063ED9CB87C2D370F263\
        D5FAD7466D8499EB8F464A702512B0CEE771E9130D697735\
        F897FD036CC504326C3B01399F643532290F958C0BBD9006\
        5DF08BABBD30AEB63B84C4605D6CA371047127D03A72D598\
        A1EDADFE707E884725C16890549084008D391E0953C3F36B\
        C438CD085EDD2D934CE1938C357A711E0D4A341A5B0A85ED\
        12C1F4E5156A26746DDDE16D826F477C97477E0A0FDF6553\
        143E2CA3A735E02ECCD94B27D04861D1119DD0C328ADF3F6\
        8FB094B867716BD7DC0DEEBB10B8240E68034893EAD82D54\
        C9DA754C46C7EEE0C37FDBEE48536047A6FA1AE49A014249\
        1B61FD5A693E381360EA6E593013236F64BA8F3B1EDD1BDE\
        FC7FCA0356CF298772ED9C17A09800D7583529F6C813EC18\
        8BCB93D8432D448C6D1F6DF5E7CD8A76A267365D676A5D8D\
        EDBF8A23F36612A5999028A895EBD7A137DC7A009BC6695F\
        ACC1E500E325C9767819750AE8B90E81FA416BE7373A7F7B\
        6AAF3817A34C06415AD42018C8058E4F2CF3E4BFDF63F479\
        91D4BD3F1B66445F078EA2DBFFAC2D62A5EA03D915A0AA55\
        6647B6BF5FA470EC0A662F6907C01BF053CB8AF7794DF194\
        0350EAC5DBE2ED3B7AA8551EC50FDFF8758CE658D189EAAE\
        6D2B64F617794B191C3FF46BB71E0234021F47B31FA43077\
        095F96AD85BA3A6B734A7C8F36DF08ACBA51C937897F72F2\
        1C3BBE5B54996FC66C5F626839DC98DD1DE4195B46CEE980\
        3A0FD3DFC57E23F692BB7B49B5D212331D55B1CE2D727AB4\
        1A11DA3A15F8E4BC11C78B65F1CEB296F1FEDC5F7E42456C\
        911117025201BE0389F5ABD40D11F8639A39FE3236751835\
        A5E5E44317C1C2EEFD4EA5BFD16043F43CB41981F6ADEE9D\
        03159E7AD9D13C53369509FC1FA27C16EF9887703A55B51B\
        22CBF44CD012AEE0B2798E628423428EFCD5A40CAEF6BF50\
        D8EA885EBF73A6B9FD79B5E18F67D1341AC8237A75C3CFC9\
        2004A1C5A40E366BC44D00176AF71C15E48C86D37E013723\
        CAAC7223AB3BF4D54F1828713B2B4A6FE40FAB74405CB738\
        B064C06ECC76E9EFFFFFFFFFFFFFFFFF",
        16,
    )
    .unwrap();
}

#[cfg(test)]
mod test {
    use num_bigint::BigUint;

    use super::*;

    fn test_order<G: MODPGroup>() {
        let lhs = G::sophie_garmain_prime();
        let rhs = (G::prime_modulus() - BigUint::from(1u32)) / BigUint::from(2u32);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_orders() {
        test_order::<MODPGroup5>();
        test_order::<MODPGroup14>();
        test_order::<MODPGroup15>();
        test_order::<MODPGroup16>();
        test_order::<MODPGroup17>();
        test_order::<MODPGroup18>();
    }
}
