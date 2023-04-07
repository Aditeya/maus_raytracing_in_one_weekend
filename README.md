# Maus Ray Tracing in One Weekend

I made this project in rust following the [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) Book. 

## Usage
```
Usage: maus_raytracing_in_one_weekend --filename <FILE>

Options:
  -f, --filename <FILE>
  -h, --help             Print help
  -V, --version          Print version
```

## Running
```sh
cargo run -qr -- -f test
```

## Final Result

### Render 1
![image](https://user-images.githubusercontent.com/22963960/213843837-10c3e9ac-0f75-432e-a8ee-a5859ffe80f5.jpg)
Render Time: 01:05:52

### Render 2
![image2](https://user-images.githubusercontent.com/22963960/230586425-9e2613e6-2b09-4a97-b9f2-9911e8e9212e.jpg)
Render Time: 01:05:25

Rendered after implementing ProgressBar & BufWriter
