# Maus Ray Tracing in One Weekend

I made this project in rust following the [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) Book. 

## Usage
```
Usage: maus_raytracing_in_one_weekend [OPTIONS] --filename <FILE>

Options:
  -s, --scene-number <NUM>  [default: 0]
  -f, --filename <FILE>
  -h, --help                Print help
  -V, --version             Print version
```

## Running
```sh
cargo run -qr -- -f test
```

## Book 2 Final Result

[imgur album](https://imgur.com/a/sYA0Ppr) of progress

![final_image](https://user-images.githubusercontent.com/22963960/232484463-7e18cd98-d017-473f-b541-839efdde3dae.jpg)

Render Time: 11:06:30

## Book 1 Final Result

[imgur album](https://imgur.com/a/2y72Qfw) of progress

### Render 1
![image](https://user-images.githubusercontent.com/22963960/213843837-10c3e9ac-0f75-432e-a8ee-a5859ffe80f5.jpg)

Render Time: 01:05:52

### Render 2
![image2](https://user-images.githubusercontent.com/22963960/230586425-9e2613e6-2b09-4a97-b9f2-9911e8e9212e.jpg)

Render Time: 01:05:25

Rendered after implementing ProgressBar & BufWriter
