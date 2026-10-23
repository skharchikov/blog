// Estimate the art-pixel grid (period + phase per axis) of an upscaled pixel-art frame: grid <png>
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

let (w, h, b) = load(CommandLine.arguments[1])
func px(_ x: Int, _ y: Int) -> (Int, Int, Int) { let i = (y * w + x) * 4; return (Int(b[i]), Int(b[i + 1]), Int(b[i + 2])) }
func diff(_ a: (Int, Int, Int), _ c: (Int, Int, Int)) -> Int { abs(a.0 - c.0) + abs(a.1 - c.1) + abs(a.2 - c.2) }

// edge strength histograms
var ex = [Double](repeating: 0, count: w), ey = [Double](repeating: 0, count: h)
for y in 0..<h { for x in 1..<w { let d = diff(px(x, y), px(x - 1, y)); if d > 80 { ex[x] += 1 } } }
for x in 0..<w { for y in 1..<h { let d = diff(px(x, y), px(x, y - 1)); if d > 80 { ey[y] += 1 } } }

func best(_ e: [Double]) -> [(Double, Double, Double)] {
    let total = e.reduce(0, +)
    var res: [(Double, Double, Double)] = []
    var p = 3.0
    while p <= 16.0 {
        var ph = 0.0
        while ph < p {
            var s = 0.0, k = 0.0
            while ph + k * p < Double(e.count) - 1 {
                let c = Int((ph + k * p).rounded())
                if c > 0 && c < e.count { s += e[c] }
                k += 1
            }
            res.append((s / total, p, ph))
            ph += 0.25
        }
        p += 0.05
    }
    return Array(res.sorted { $0.0 > $1.0 }.prefix(5))
}
print("size \(w)x\(h)")
print("x:", best(ex).map { String(format: "score %.3f period %.2f phase %.2f", $0.0, $0.1, $0.2) })
print("y:", best(ey).map { String(format: "score %.3f period %.2f phase %.2f", $0.0, $0.1, $0.2) })
