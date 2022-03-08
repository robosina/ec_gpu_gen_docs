use std::io::Write;
use ec_gpu::GpuEngine;
use ec_gpu_gen::Limb;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
// Instead of having a very large OpenCL program written for a specific curve, with a lot of
// rudandant codes (As OpenCL doesn't have generic types or templates), this module will dynamically
// generate CUDA/OpenCL codes given different PrimeFields and curves.

static FFT_SRC: &str = include_str!("fft/fft.cl");
static MULTIEXP_SRC: &str = include_str!("multiexp/multiexp.cl");

fn fft(field: &str) -> String {
    String::from(FFT_SRC).replace("FIELD", field)
}

fn multiexp(point: &str, exp: &str) -> String {
    String::from(MULTIEXP_SRC)
        .replace("POINT", point)
        .replace("EXPONENT", exp)
}

// WARNING: This function works only with Short Weierstrass Jacobian curves with Fq2 extension field.
pub fn kernel<E, L>() -> String
    where
        E: GpuEngine,
        L: Limb,
{
    let common = ec_gpu_gen::common();
    let mut file = fs::File::create("common.cu").unwrap();
    file.write_all(common.as_bytes()).unwrap();

    let gen_ec_source = ec_gpu_gen::gen_ec_source::<E, L>();
    let mut file = fs::File::create("ec_gpu_gen.cu").unwrap();
    file.write_all(gen_ec_source.as_bytes()).unwrap();

    let fft_source = fft("Fr");
    let mut file = fs::File::create("fft.cu").unwrap();
    file.write_all(fft_source.as_bytes()).unwrap();

    let multiexp_source1 = multiexp("G1", "Fr");
    let mut file = fs::File::create("multiexp1.cu").unwrap();
    file.write_all(multiexp_source1.as_bytes()).unwrap();

    let multiexp_source2 = multiexp("G2", "Fr");
    let mut file = fs::File::create("multiexp2.cu").unwrap();
    file.write_all(multiexp_source2.as_bytes()).unwrap();

    [
        common,
        gen_ec_source,
        fft_source,
        multiexp_source1,
        multiexp_source2,
    ].join("\n\n")
}

fn main() {


    use blstrs::Bls12;
    use ec_gpu_gen::Limb32;
    use sha2::{Digest, Sha256};

    let kernel_source = kernel::<Bls12, Limb32>();

    //write kernel source to file
    let mut file = fs::File::create("code.cu").unwrap();
    file.write_all(kernel_source.as_bytes()).unwrap();

}