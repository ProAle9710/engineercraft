"""Render a workshop E icon to PNG + ICO."""
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageFont

ROOT = Path(__file__).resolve().parent
PNG = ROOT / "icon.png"
ICO = ROOT / "icon.ico"
SIZE = 1024


def font(size: int) -> ImageFont.FreeTypeFont | ImageFont.ImageFont:
    for path in (
        r"C:\Windows\Fonts\georgia.ttf",
        r"C:\Windows\Fonts\georgiab.ttf",
        r"C:\Windows\Fonts\times.ttf",
        r"C:\Windows\Fonts\timesbd.ttf",
        r"C:\Windows\Fonts\palab.ttf",
        r"C:\Windows\Fonts\constan.ttf",
    ):
        p = Path(path)
        if p.exists():
            try:
                return ImageFont.truetype(str(p), size)
            except OSError:
                continue
    return ImageFont.load_default()


def rounded_mask(size: int, radius: int) -> Image.Image:
    m = Image.new("L", (size, size), 0)
    d = ImageDraw.Draw(m)
    d.rounded_rectangle([0, 0, size - 1, size - 1], radius=radius, fill=255)
    return m


def main() -> None:
    s = SIZE
    img = Image.new("RGBA", (s, s), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)

    pad = int(s * 0.035)
    radius = int(s * 0.22)
    panel = [pad, pad, s - pad - 1, s - pad - 1]
    d.rounded_rectangle(panel, radius=radius, fill=(16, 15, 13, 255))
    d.rounded_rectangle(panel, radius=radius, outline=(200, 164, 90, 255), width=max(4, s // 90))

    gold = (212, 176, 110, 255)
    faint = (212, 176, 110, 70)
    g = ImageDraw.Draw(img)
    inset = int(s * 0.23)
    step = int((s - inset * 2) / 3)
    for i in range(4):
        x = inset + i * step
        g.line([(x, inset), (x, s - inset)], fill=faint, width=2)
        g.line([(inset, x), (s - inset, x)], fill=faint, width=2)
    for ix in (1, 2):
        for iy in (1, 2):
            cx = inset + ix * step
            cy = inset + iy * step
            g.ellipse([cx - 4, cy - 4, cx + 4, cy + 4], fill=(200, 164, 90, 120))

    fnt = font(int(s * 0.58))
    letter = "E"
    bbox = d.textbbox((0, 0), letter, font=fnt)
    tw, th = bbox[2] - bbox[0], bbox[3] - bbox[1]
    tx = (s - tw) / 2 - bbox[0]
    ty = (s - th) / 2 - bbox[1] - int(s * 0.04)
    d.text((tx, ty), letter, font=fnt, fill=gold)

    bar_y = int(s * 0.78)
    bar_h = max(10, s // 48)
    x0, x1 = int(s * 0.33), int(s * 0.67)
    mid = bar_y + bar_h // 2
    d.polygon(
        [
            (x0, bar_y),
            (x1, bar_y),
            (x1 + bar_h, mid),
            (x1, bar_y + bar_h),
            (x0, bar_y + bar_h),
            (x0 - bar_h, mid),
        ],
        fill=gold,
    )

    mask = rounded_mask(s, radius + pad)
    out = Image.new("RGBA", (s, s), (0, 0, 0, 0))
    out.paste(img, mask=mask)
    out = out.filter(ImageFilter.UnsharpMask(radius=1.2, percent=80, threshold=2))
    png = out.resize((512, 512), Image.Resampling.LANCZOS)
    png.save(PNG, "PNG")
    sizes = [16, 24, 32, 48, 64, 128, 256]
    icos = [out.resize((n, n), Image.Resampling.LANCZOS) for n in sizes]
    icos[0].save(ICO, format="ICO", sizes=[(n, n) for n in sizes], append_images=icos[1:])
    print(f"wrote {PNG} {ICO}")


if __name__ == "__main__":
    main()
