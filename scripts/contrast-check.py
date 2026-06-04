#!/usr/bin/env python3
"""
Contrast-check script for Brewski themes.
Exits with code 1 if any hard failures are found.
Hard failures:
  - --color-text-primary vs --color-bg-base contrast < 4.5
  - --color-accent vs white contrast < 4.5
Warnings printed for secondary/muted/border but do not fail CI.

Usage: python3 scripts/contrast-check.py
"""
import glob, re, sys

pat = re.compile(r'--([a-z0-9-]+):\s*#([0-9a-fA-F]{3,6})')

def hex_to_rgb(h):
    h=h.strip().lstrip('#')
    if len(h)==3:
        h=''.join(c*2 for c in h)
    return tuple(int(h[i:i+2],16) for i in (0,2,4))

def lin(c):
    c=c/255.0
    return c/12.92 if c<=0.03928 else ((c+0.055)/1.055)**2.4

def lum(rgb):
    r,g,b=rgb
    return 0.2126*lin(r)+0.7152*lin(g)+0.0722*lin(b)

def contrast(a,b):
    La=lum(a); Lb=lum(b)
    L1=max(La,Lb); L2=min(La,Lb)
    return (L1+0.05)/(L2+0.05)

files = sorted(glob.glob('src/themes/*.css'))
if not files:
    print('No theme files found in src/themes')
    sys.exit(0)

hard_fail = False

for f in files:
    name = f.split('/')[-1].replace('.css','')
    vals = {}
    with open(f,'r') as fh:
        for line in fh:
            for m in pat.finditer(line):
                try:
                    vals[m.group(1)] = hex_to_rgb(m.group(2))
                except Exception:
                    pass
    bg = vals.get('color-bg-base')
    textp = vals.get('color-text-primary')
    texts = vals.get('color-text-secondary')
    textm = vals.get('color-text-muted')
    accent = vals.get('color-accent')
    border = vals.get('color-border')

    def r(name, a, b):
        if a and b:
            return contrast(a,b)
        return None

    c_primary = r('primary_vs_bg', textp, bg)
    c_accent = r('accent_vs_white', accent, (255,255,255))
    c_secondary = r('secondary_vs_bg', texts, bg)
    c_muted = r('muted_vs_bg', textm, bg)
    c_border = r('border_vs_surface', border, vals.get('color-bg-surface'))

    print(f"\nTheme: {name}")
    if c_primary is not None:
        print(f"  Primary vs bg: {c_primary:.2f}")
        if c_primary < 4.5:
            print("    HARD FAIL: primary text contrast < 4.5")
            hard_fail = True
    else:
        print("  Primary vs bg: n/a")

    if c_accent is not None:
        print(f"  Accent vs white: {c_accent:.2f}")
        if c_accent < 4.5:
            print("    HARD FAIL: accent color too light for white text (button labels)")
            hard_fail = True
    else:
        print("  Accent vs white: n/a")

    if c_secondary is not None:
        print(f"  Secondary vs bg: {c_secondary:.2f}")
        if c_secondary < 3.0:
            print("    WARN: secondary text contrast < 3.0 (consider darkening)")
    if c_muted is not None:
        print(f"  Muted vs bg: {c_muted:.2f}")
        if c_muted < 3.0:
            print("    WARN: muted text contrast < 3.0 (consider darkening)")
    if c_border is not None:
        print(f"  Border vs surface: {c_border:.2f}")

if hard_fail:
    print('\nOne or more hard failures detected. See above.')
    sys.exit(1)
else:
    print('\nNo hard failures detected.')
    sys.exit(0)
