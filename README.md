# ec_gpu_gen_docs

CUDA/OpenCL code generator for finite-field arithmetic over prime fields and `elliptic curve==>ec` arithmetic constructed with Rust.

# How does code generation works?

First of all it is necessary to add the generator dependency to your project. For example in your
`cargo.toml` file you should add the below code:

```
[dependencies]
ec-gpu = { version = "0.1.0", optional = true }
ec-gpu-gen = { version = "0.1.0", optional = true }
```
Actual code generation is done by the `ec-gpu-gen` crate. in there we have
four `.cl` file which is responsible for the generation of the final code. these files are:

- ec-gpu-gen/src/cl/`common.cl`
- ec-gpu-gen/src/cl/`ec.cl`
- ec-gpu-gen/src/cl/`field.cl`
- ec-gpu-gen/src/cl/`field2.cl`
- ec-gpu-gen/src/cl/`test.cl` (this file is not required for generation)

The application of these files is related to the pure mathematical operations, which is not the case of this readme.

In bellperson project we can see that the following code is used in the `build.rs` file:

```rust
use blstrs::Bls12;
use ec_gpu_gen::Limb32;
use sha2::{Digest, Sha256};

#[path = "src/gpu/sources.rs"]
mod sources;

let kernel_source = sources::kernel::<Bls12, Limb32>();
```

In the above code we can see that the kernel is responsible for the generation of the code. Kernel will take two
things as it's arguments:

```
pub fn kernel<E, L>() -> String     
where
        E: GpuEngine,
        L: Limb,
{
...
}
```

So in here:
- `GpuEngine` is `Bls12`
- `Limb` is `Limb32`

After that the kernel function will use the mentioned files to generate the code.