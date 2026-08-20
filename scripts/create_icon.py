from pathlib import Path

from PIL import Image, ImageDraw

output = Path(__file__).resolve().parents[1] / "assets" / "app.ico"
sizes = [16, 24, 32, 48, 64, 128, 256]
images = []

for size in sizes:
    image = Image.new("RGBA", (size, size), (14, 15, 17, 255))
    draw = ImageDraw.Draw(image)
    margin = max(1, size // 8)
    corner = max(2, size // 8)
    draw.rounded_rectangle((margin, margin, size - margin, size - margin), radius=corner, fill=(24, 26, 30, 255), outline=(91, 156, 255, 255), width=max(1, size // 24))
    line_left = size * 0.28
    line_right = size * 0.72
    for fraction in (0.36, 0.50, 0.64):
        y = int(size * fraction)
        draw.line((line_left, y, line_right, y), fill=(235, 237, 240, 255), width=max(1, size // 18))
    images.append(image)

images[-1].save(output, format="ICO", sizes=[(size, size) for size in sizes])
print(output)
