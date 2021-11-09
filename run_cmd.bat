echo "Building..."
cargo build --release
echo "Done building"

echo "Starting render"
.\target\release\raytracer.exe --spp 150 -w 1080 > image_windows.ppm

echo "Done render, converting ppm > png"
C:\"Program Files"\ImageMagick-7.1.0-Q16-HDRI\magick.exe convert .\image_windows.ppm .\image_windows.png

image_windows.png