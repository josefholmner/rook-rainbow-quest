# Original procedural artwork matching the game's smiling cream rook.
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$assetRoot = Join-Path (Split-Path -Parent $PSScriptRoot) 'assets'
$bitmap = [Drawing.Bitmap]::new(1024, 1024)
$graphics = [Drawing.Graphics]::FromImage($bitmap)
$graphics.SmoothingMode = 'AntiAlias'
$graphics.Clear([Drawing.Color]::Transparent)
$graphics.ScaleTransform(4, 4)
function Brush([string]$Color) { [Drawing.SolidBrush]::new([Drawing.ColorTranslator]::FromHtml($Color)) }
function Rounded([single]$X,[single]$Y,[single]$Width,[single]$Height,[single]$Radius,$Fill) {
    $path = [Drawing.Drawing2D.GraphicsPath]::new()
    $d = $Radius * 2
    $path.AddArc($X,$Y,$d,$d,180,90)
    $path.AddArc($X+$Width-$d,$Y,$d,$d,270,90)
    $path.AddArc($X+$Width-$d,$Y+$Height-$d,$d,$d,0,90)
    $path.AddArc($X,$Y+$Height-$d,$d,$d,90,90)
    $path.CloseFigure()
    $graphics.FillPath($Fill,$path)
    $path.Dispose()
}
$purple = Brush '#7D57C2'; $edge = Brush '#382C55'; $cream = Brush '#FFF6DA'
$gold = Brush '#FFC641'; $mint = Brush '#A5D7B1'; $coral = Brush '#FF7B80'
Rounded 8 12 240 240 54 $edge
Rounded 8 4 240 240 54 $purple
$graphics.FillEllipse($gold,28,23,200,200)
# Broad rook silhouette remains legible at small desktop icon sizes.
Rounded 67 72 122 48 8 $edge
Rounded 72 65 112 47 6 $cream
foreach ($x in @(72,116,160)) { Rounded $x 43 24 43 5 $edge; Rounded ($x+4) 43 16 35 3 $cream }
Rounded 83 102 90 82 10 $edge
Rounded 89 103 78 72 7 $cream
Rounded 67 173 122 31 12 $edge
Rounded 72 174 112 22 8 $cream
Rounded 78 182 100 8 4 $mint
$graphics.FillEllipse($edge,105,124,9,13); $graphics.FillEllipse($edge,143,124,9,13)
$graphics.FillEllipse($coral,96,142,13,7); $graphics.FillEllipse($coral,147,142,13,7)
$smile = [Drawing.Pen]::new([Drawing.ColorTranslator]::FromHtml('#382C55'),4)
$smile.StartCap='Round'; $smile.EndCap='Round'
$graphics.DrawArc($smile,117,139,22,16,15,150)
$smile.Dispose(); $graphics.Dispose()
$images = @()
foreach ($size in @(16,32,48,64,128,256)) {
    $scaled = [Drawing.Bitmap]::new($size,$size)
    $g = [Drawing.Graphics]::FromImage($scaled)
    $g.InterpolationMode='HighQualityBicubic'; $g.PixelOffsetMode='HighQuality'
    $g.DrawImage($bitmap,0,0,$size,$size); $g.Dispose()
    $memory = [IO.MemoryStream]::new()
    $scaled.Save($memory,[Drawing.Imaging.ImageFormat]::Png)
    $images += ,@($size,$memory.ToArray())
    if ($size -eq 256) { [IO.File]::WriteAllBytes((Join-Path $assetRoot 'rook-rainbow-quest-icon.png'),$memory.ToArray()) }
    if ($size -in @(16,32,64)) {
        $rgba = [Collections.Generic.List[byte]]::new()
        for ($y=0;$y -lt $size;$y++) { for ($x=0;$x -lt $size;$x++) {
            $p=$scaled.GetPixel($x,$y)
            $rgba.AddRange([byte[]]@($p.R,$p.G,$p.B,$p.A))
        } }
        [IO.File]::WriteAllBytes((Join-Path $assetRoot "icon-$size.rgba"),$rgba.ToArray())
    }
    $memory.Dispose(); $scaled.Dispose()
}
$icon = [IO.MemoryStream]::new(); $writer = [IO.BinaryWriter]::new($icon)
$writer.Write([uint16]0); $writer.Write([uint16]1); $writer.Write([uint16]$images.Count)
$offset = 6 + 16 * $images.Count
foreach ($image in $images) {
    $size=$image[0]; $bytes=$image[1]
    $dimension=if ($size -eq 256) { 0 } else { $size }
    $writer.Write([byte]$dimension); $writer.Write([byte]$dimension)
    $writer.Write([uint16]0); $writer.Write([uint16]1); $writer.Write([uint16]32)
    $writer.Write([uint32]$bytes.Length); $writer.Write([uint32]$offset)
    $offset += $bytes.Length
}
foreach ($image in $images) { $writer.Write([byte[]]$image[1]) }
[IO.File]::WriteAllBytes((Join-Path $assetRoot 'rook-rainbow-quest.ico'),$icon.ToArray())
$writer.Dispose(); $icon.Dispose(); $bitmap.Dispose()
foreach ($brush in @($purple,$edge,$cream,$gold,$mint,$coral)) { $brush.Dispose() }
Write-Host 'Generated the original rook icon: 16–256px ICO, PNG preview and runtime RGBA sizes.'
