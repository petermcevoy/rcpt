A ray-tracer that performs spectral rendering.

It performs spectral rendering by using a fixed number of samples of the visible spectrum instead of RGB primaries.
The renderer also has support for *re-readiance* matrices for diffuse color to allow simulating fluorescent behaviour.

It was developed during my Master's thesis to render images using measurments of fluorescent materials.

Structure is inspired by and based on Peter Shirley's book [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).
It has been extended to support spectral rendering and materials that do not neccessaraly reflect the same wavelength as it was illuminated with (so called .*bi-spectral* materials).

```
cargo run --release --features=use_sampled_spectrum
```

![Image shows renders of conrell-box under different types of illumination (D65, D50 and Halogen)](https://user-images.githubusercontent.com/116268/122679529-87c0a080-d1eb-11eb-9b5e-449f8a8de164.png "Demo renders")

