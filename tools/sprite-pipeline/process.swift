// Turn upscaled, video-compressed pixel-art frames on a magenta background into
// native-resolution transparent sprites. Nothing is repainted: each output pixel is
// the median colour sampled from the centre of its grid cell; magenta cells become
// transparent; cells that only differ from frame 0 by compression noise keep frame 0's
// colour (stops static areas flickering).
//
// process <framesDir> <outDir> <period> <phaseX> <phaseY> <fps> <cols>
import AppKit
import ImageIO
import UniformTypeIdentifiers

typealias RGB = (Int, Int, Int)
let a = CommandLine.arguments
let framesDir = a[1], outDir = a[2]
let period = Double(a[3])!, phX = Double(a[4])!, phY = Double(a[5])!
let fps = Double(a[6])!, cols = Int(a[7])!
let name = a.count > 8 ? a[8] : "books"

func load(_ path: String) -> (Int, Int, [UInt8]) {
    let img = NSImage(contentsOfFile: path)!
    var rect = CGRect(origin: .zero, size: img.size)
    let cg = img.cgImage(forProposedRect: &rect, context: nil, hints: nil)!
    let w = cg.width, h = cg.height
    var buf = [UInt8](repeating: 0, count: w * h * 4)
    let ctx = CGContext(data: &buf, width: w, height: h, bitsPerComponent: 8, bytesPerRow: w * 4,
                        space: CGColorSpaceCreateDeviceRGB(), bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)!
    ctx.draw(cg, in: CGRect(x: 0, y: 0, width: w, height: h))
    return (w, h, buf)
}

func isMagenta(_ c: RGB) -> Bool {
    let (r, g, b) = c
    // flat background, darker floor shadow, and outline/background fringe
    return r > 90 && b > 90 && g < 100 && abs(r - b) < 80 && g * 2 < min(r, b)
}
func dist(_ p: RGB, _ q: RGB) -> Int { abs(p.0 - q.0) + abs(p.1 - q.1) + abs(p.2 - q.2) }

let files = try! FileManager.default.contentsOfDirectory(atPath: framesDir).filter { $0.hasSuffix(".png") }.sorted()
let (W, H, _) = load("\(framesDir)/\(files[0])")
let nx = Int((Double(W) - phX) / period), ny = Int((Double(H) - phY) / period)

// sample every frame onto the grid; nil = transparent
var grids: [[RGB?]] = []
for f in files {
    let (_, _, buf) = load("\(framesDir)/\(f)")
    var g = [RGB?](repeating: nil, count: nx * ny)
    for cy in 0..<ny {
        for cx in 0..<nx {
            let ccx = phX + (Double(cx) + 0.5) * period, ccy = phY + (Double(cy) + 0.5) * period
            var rs: [Int] = [], gs: [Int] = [], bs: [Int] = []
            for oy in [-1.3, 0.0, 1.3] {
                for ox in [-1.3, 0.0, 1.3] {
                    let x = min(W - 1, max(0, Int(ccx + ox))), y = min(H - 1, max(0, Int(ccy + oy)))
                    let i = (y * W + x) * 4
                    rs.append(Int(buf[i])); gs.append(Int(buf[i + 1])); bs.append(Int(buf[i + 2]))
                }
            }
            rs.sort(); gs.sort(); bs.sort()
            let c: RGB = (rs[4], gs[4], bs[4])
            g[cy * nx + cx] = isMagenta(c) ? nil : c
        }
    }
    grids.append(g)
}

// temporal stabilisation against frame 0
let base = grids[0]
for k in 1..<grids.count {
    for i in 0..<(nx * ny) {
        if let c = grids[k][i], let b0 = base[i], dist(c, b0) < 48 { grids[k][i] = b0 }
    }
}

// edge fringe: dark outline cells that compression blended with the magenta keep their
// place but take the colour of the nearest clean opaque neighbour (darkest wins)
func tinted(_ c: RGB) -> Bool { c.0 - c.1 > 25 && c.2 - c.1 > 25 && abs(c.0 - c.2) < 70 }
for k in 0..<grids.count {
    var g = grids[k]
    for cy in 0..<ny { for cx in 0..<nx {
        guard let c = g[cy * nx + cx], tinted(c) else { continue }
        var best: RGB? = nil
        for r in 1...2 { for dy in -r...r { for dx in -r...r {
            let x = cx + dx, y = cy + dy
            guard x >= 0, y >= 0, x < nx, y < ny, let n = grids[k][y * nx + x], !tinted(n) else { continue }
            if best == nil || n.0 + n.1 + n.2 < best!.0 + best!.1 + best!.2 { best = n }
        } } ; if best != nil { break } }
        g[cy * nx + cx] = best ?? (28, 24, 30)
    } }
    grids[k] = g
}

// specks: drop opaque clusters that are not near the main subject. Everything within
// `keepRadius` cells of the largest connected cluster stays (e.g. steam above a mug).
let keepRadius = 10
for k in 0..<grids.count {
    var g = grids[k]
    var label = [Int](repeating: -1, count: nx * ny)
    var sizes: [Int] = []
    for start in 0..<(nx * ny) where g[start] != nil && label[start] < 0 {
        let id = sizes.count; var stack = [start]; label[start] = id; var n = 0
        while let i = stack.popLast() {
            n += 1
            let x = i % nx, y = i / nx
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let xx = x + dx, yy = y + dy
                guard xx >= 0, yy >= 0, xx < nx, yy < ny else { continue }
                let j = yy * nx + xx
                if g[j] != nil && label[j] < 0 { label[j] = id; stack.append(j) }
            }
        }
        sizes.append(n)
    }
    guard let mainId = sizes.indices.max(by: { sizes[$0] < sizes[$1] }) else { continue }
    // distance (in cells, BFS) from the main cluster, capped at keepRadius
    var dist = [Int](repeating: Int.max, count: nx * ny)
    var queue: [Int] = []
    for i in 0..<(nx * ny) where label[i] == mainId { dist[i] = 0; queue.append(i) }
    var head = 0
    while head < queue.count {
        let i = queue[head]; head += 1
        if dist[i] >= keepRadius { continue }
        let x = i % nx, y = i / nx
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let xx = x + dx, yy = y + dy
            guard xx >= 0, yy >= 0, xx < nx, yy < ny else { continue }
            let j = yy * nx + xx
            if dist[j] == Int.max { dist[j] = dist[i] + 1; queue.append(j) }
        }
    }
    for i in 0..<(nx * ny) where g[i] != nil && label[i] != mainId && dist[i] == Int.max { g[i] = nil }
    grids[k] = g
}

// crop to the union of opaque cells across all frames (+1 cell margin)
var x0 = nx, y0 = ny, x1 = 0, y1 = 0
for g in grids { for cy in 0..<ny { for cx in 0..<nx where g[cy * nx + cx] != nil {
    x0 = min(x0, cx); y0 = min(y0, cy); x1 = max(x1, cx); y1 = max(y1, cy)
} } }
x0 = max(0, x0 - 1); y0 = max(0, y0 - 1); x1 = min(nx - 1, x1 + 1)   // no margin below: the ground line is the bottom row
let fw = x1 - x0 + 1, fh = y1 - y0 + 1

func image(_ g: [RGB?]) -> CGImage {
    var px = [UInt8](repeating: 0, count: fw * fh * 4)
    for y in 0..<fh { for x in 0..<fw {
        if let c = g[(y + y0) * nx + (x + x0)] {
            let i = (y * fw + x) * 4
            px[i] = UInt8(c.0); px[i + 1] = UInt8(c.1); px[i + 2] = UInt8(c.2); px[i + 3] = 255
        }
    } }
    let provider = CGDataProvider(data: Data(px) as CFData)!
    return CGImage(width: fw, height: fh, bitsPerComponent: 8, bitsPerPixel: 32, bytesPerRow: fw * 4,
                   space: CGColorSpaceCreateDeviceRGB(), bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.last.rawValue),
                   provider: provider, decode: nil, shouldInterpolate: false, intent: .defaultIntent)!
}

func writePNG(_ img: CGImage, _ path: String) {
    let dest = CGImageDestinationCreateWithURL(URL(fileURLWithPath: path) as CFURL, UTType.png.identifier as CFString, 1, nil)!
    CGImageDestinationAddImage(dest, img, nil)
    CGImageDestinationFinalize(dest)
}

try? FileManager.default.createDirectory(atPath: outDir, withIntermediateDirectories: true)
let imgs = grids.map(image)

// still = frame 0
writePNG(imgs[0], "\(outDir)/\(name)-still.png")

// sprite sheet (row-major grid)
let rows = (imgs.count + cols - 1) / cols
let sheetCtx = CGContext(data: nil, width: fw * cols, height: fh * rows, bitsPerComponent: 8, bytesPerRow: 0,
                         space: CGColorSpaceCreateDeviceRGB(), bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)!
sheetCtx.interpolationQuality = .none
for (k, im) in imgs.enumerated() {
    let cx = k % cols, cy = k / cols
    sheetCtx.draw(im, in: CGRect(x: cx * fw, y: (rows - 1 - cy) * fh, width: fw, height: fh))
}
writePNG(sheetCtx.makeImage()!, "\(outDir)/\(name).png")

// animated PNG (plays in a plain <img>, keeps transparency)
let apng = CGImageDestinationCreateWithURL(URL(fileURLWithPath: "\(outDir)/\(name).apng.png") as CFURL, UTType.png.identifier as CFString, imgs.count, nil)!
CGImageDestinationSetProperties(apng, [kCGImagePropertyPNGDictionary: [kCGImagePropertyAPNGLoopCount: 0]] as CFDictionary)
for im in imgs {
    CGImageDestinationAddImage(apng, im, [kCGImagePropertyPNGDictionary: [kCGImagePropertyAPNGDelayTime: 1.0 / fps]] as CFDictionary)
}
CGImageDestinationFinalize(apng)

let manifest = """
{ "\(name)": { "frames": \(imgs.count), "cols": \(cols), "rows": \(rows), "frame_ms": \(Int(1000.0 / fps)), "width": \(fw), "height": \(fh) } }
"""
try! manifest.write(toFile: "\(outDir)/manifest.json", atomically: true, encoding: .utf8)
print("grid \(nx)x\(ny) cells, frame \(fw)x\(fh), frames \(imgs.count), sheet \(cols)x\(rows)")
