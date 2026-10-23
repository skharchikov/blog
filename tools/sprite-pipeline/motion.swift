// Motion report for a frame directory: motion <dir>
// Prints per-frame changed-pixel counts vs frame 0 and vs previous frame, with bounding boxes
// (background = magenta-ish pixels are ignored), plus a first-vs-last loop check.
import AppKit

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

func isBg(_ b: [UInt8], _ i: Int) -> Bool { b[i] > 170 && b[i + 1] < 110 && b[i + 2] > 170 }

let dir = CommandLine.arguments[1]
let files = try! FileManager.default.contentsOfDirectory(atPath: dir).filter { $0.hasSuffix(".png") }.sorted()
let (w, h, f0) = load("\(dir)/\(files[0])")
var prev = f0

func compare(_ a: [UInt8], _ b: [UInt8]) -> (Int, Int, Int, Int, Int) {
    var n = 0, x0 = w, y0 = h, x1 = 0, y1 = 0
    var y = 0
    while y < h {
        var x = 0
        while x < w {
            let i = (y * w + x) * 4
            if !(isBg(a, i) && isBg(b, i)) {
                let d = abs(Int(a[i]) - Int(b[i])) + abs(Int(a[i + 1]) - Int(b[i + 1])) + abs(Int(a[i + 2]) - Int(b[i + 2]))
                if d > 60 { n += 1; x0 = min(x0, x); y0 = min(y0, y); x1 = max(x1, x); y1 = max(y1, y) }
            }
            x += 2
        }
        y += 2
    }
    return (n, x0, y0, x1, y1)
}

print("size \(w)x\(h), frames \(files.count)")
var last = f0
for (k, f) in files.enumerated() where k > 0 {
    let (_, _, cur) = load("\(dir)/\(f)")
    let p = compare(prev, cur), z = compare(f0, cur)
    print(String(format: "%@ prev:%6d [%d,%d,%d,%d]  vs0:%6d [%d,%d,%d,%d]", f, p.0, p.1, p.2, p.3, p.4, z.0, z.1, z.2, z.3, z.4))
    prev = cur
    last = cur
}
let loop = compare(last, f0)
print("loop (last vs first): \(loop.0) changed samples, box [\(loop.1),\(loop.2),\(loop.3),\(loop.4)]")
